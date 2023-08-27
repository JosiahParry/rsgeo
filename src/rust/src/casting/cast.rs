use extendr_api::prelude::*;
use geo::CoordsIter;
use geo_types::{LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon};
use sfconversions::{Geom, vctrs::as_rsgeo_vctr, IntoGeom};

//# cast 1 : 1
//# expand 1 : many
//# combine many : 1

// This matrix documents the possible scalar conversions
// CASTING ------------------------------------------------------------------------
//                      to_point to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// from_point               TRUE          TRUE      FALSE           FALSE         FALSE              FALSE
// from_multipoint            NA          TRUE       TRUE            TRUE          TRUE               TRUE
// from_polygon               NA          TRUE       TRUE            TRUE          TRUE               TRUE
// from_multipolygon          NA          TRUE         NA            TRUE            NA               TRUE
// from_linestring            NA          TRUE      FALSE           FALSE          TRUE               TRUE
// from_multilinestring       NA          TRUE      FALSE           FALSE            NA               TRUE

// ————————————————————————
// Scalar Conversions
// ————————————————————————

// Point Conversions
// ————————————————————————

// from_point               TRUE          TRUE      FALSE           FALSE         FALSE              FALSE
fn cast_point_multipoint(x: Geom) -> Geom {
    let res = MultiPoint::new(vec![Point::from(x)]);
    Geom::from(res)
}

// MultiPoint Conversions
// ————————————————————————
// //                      to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// // from_multipoint              TRUE       TRUE            TRUE          TRUE               TRUE

fn cast_multipoint_polygon(x: Geom) -> Geom {
    let x = MultiPoint::from(x);
    let mut crds = x.0;
    crds.push(crds[0]);
    let ln = LineString::from(crds);
    Geom::from(Polygon::new(ln, vec![]))
}

// #[extendr]
fn cast_multipoint_multipolygon(x: Geom) -> Geom {
    let x = MultiPoint::from(x);
    let mut crds = x.0;
    crds.push(crds[0]);
    let ln = LineString::from(crds);
    Geom::from(MultiPolygon::new(vec![Polygon::new(ln, vec![])]))
}

fn cast_multipoint_linestring(x: Geom) -> Geom {
    let x = MultiPoint::from(x);
    let crds = x.0;
    let ln = LineString::from(crds);
    Geom::from(ln)
}

fn cast_multipoint_multilinestring(x: Geom) -> Geom {
    let x = MultiPoint::from(x);
    let crds = x.0;
    let ln = LineString::from(crds);
    Geom::from(MultiLineString::new(vec![ln]))
}

// Polygon Conversions
// ————————————————————————
// //                      to_point to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// // from_polygon               NA          TRUE       TRUE            TRUE          TRUE               TRUE
fn cast_polygon_multipoint(x: Geom) -> Geom {
    let x = Polygon::from(x);
    let pnts = x
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    Geom::from(MultiPoint::new(pnts))
}

fn cast_polygon_multipolygon(x: Geom) -> Geom {
    Geom::from(MultiPolygon::try_from(x.geom).unwrap())
}

fn cast_polygon_linestring(x: Geom) -> Geom {
    let x = Polygon::from(x);
    let x = x.into_inner();
    Geom::from(x.0)
}

fn cast_polygon_multilinestring(x: Geom) -> Geom {
    let x = Polygon::from(x);
    let (ext, holes) = x.into_inner();
    let mut ext = vec![ext];
    ext.extend(holes.into_iter());

    Geom::from(MultiLineString::new(ext))
}


// MultiPolygon Conversions
// ————————————————————————

// //                       to_multipoint  to_multipolygon  to_multilinestring
// // from_multipolygon              TRUE             TRUE                TRUE
fn cast_multipolygon_multipoint(x: Geom) -> Geom {
    let mply = MultiPolygon::try_from(x.geom).unwrap();

    let pnts = mply
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    let res = MultiPoint::new(pnts);
    Geom::from(res)
}

fn cast_multipolygon_multilinestring(x: Geom) -> Geom {
    let mply = MultiPolygon::try_from(x.geom).unwrap();
    let linestrings = mply
        .0
        .into_iter()
        .flat_map(|x| MultiLineString::try_from(cast_polygon_multilinestring(x.into_geom()).geom).unwrap().0)
        .collect::<Vec<LineString>>();

    Geom::from(MultiLineString::new(linestrings))
}

// LineString Conversions
// ————————————————————————
// //                      to_multipoint to_linestring to_multilinestring
// // from_linestring               TRUE          TRUE               TRUE
fn cast_linestring_multipoint(x: Geom) -> Geom {
    Geom::from(LineString::from(x).coords_iter().collect::<MultiPoint>())
}

fn cast_linestring_polygon(x: Geom) -> Geom {
    let mut coords = LineString::from(x).0;
    coords.push(coords[0]);
    Geom::from(Polygon::new(LineString::from(coords), vec![]))
}

fn cast_linestring_multilinestring(x: Geom) -> Geom {
    Geom::from(MultiLineString::new(vec![LineString::from(x)]))
}

// MultiLineString Conversions
// ————————————————————————
// //                      to_multipoint to_multilinestring
// // from_multilinestring          TRUE               TRUE

fn cast_multilinestring_multipoint(x: Geom) -> Geom {
    Geom::from(MultiPoint::from_iter(
        MultiLineString::from(x).coords_iter(),
    ))
}

fn cast_multilinestring_multipolygon(x: Geom) -> Geom {
    let x = MultiLineString::from(x);
    let res =
        x.0.into_iter()
            .map(|lns| {
                let mut coords = lns.0;
                coords.push(coords[0]);
                Polygon::new(LineString::from(coords), vec![])
            })
            .collect::<Vec<Polygon>>();

    Geom::from(MultiPolygon::new(res))
}

// ———————————————————————
// Vector Implementations
// ———————————————————————

#[extendr]
fn cast_points(x: List, to: &str) -> Robj {
    if !x.inherits("rs_POINT") {
        panic!("`x` must be an `rs_POINT`")
    }

    let f = match to {
        "point" => |x| x,
        "multipoint" => cast_point_multipoint,
        &_ => unimplemented!("for provided `to` geometry type"), // unreachable (in theory)
    };

    let res_vec = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                xi
            } else {
                f(Geom::from(xi)).into()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), to)
}

#[extendr]
fn cast_multipoints(x: List, to: &str) -> Robj {
    if !x.inherits("rs_MULTIPOINT") {
        panic!("`x` must be an `rs_MULTIPOINT`")
    }

    let f = match to {
        "multipoint" => |x| x,
        "polygon" => cast_multipoint_polygon,
        "multipolygon" => cast_multipoint_multipolygon,
        "linestring" => cast_multipoint_linestring,
        "multilinestring" => cast_multipoint_multilinestring,
        &_ => unimplemented!("for provided `to` geometry type"),
    };

    let res_vec = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                xi
            } else {
                f(Geom::from(xi)).into()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), to)
}

#[extendr]
fn cast_linestrings(x: List, to: &str) -> Robj {
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be an `rs_LINESTRING`")
    }

    let f = match to {
        "linestring" => |x| x,
        "multipoint" => cast_linestring_multipoint,
        "multilinestring" => cast_linestring_multilinestring,
        "polygon" => cast_linestring_polygon,
        &_ => unimplemented!("for provided `to` geometry type"),
    };

    let res_vec = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                xi
            } else {
                f(Geom::from(xi)).into()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), to)
}

#[extendr]
fn cast_multilinestrings(x: List, to: &str) -> Robj {
    if !x.inherits("rs_MULTILINESTRING") {
        panic!("`x` must be an `rs_MULTILINESTRING`")
    }

    let f = match to {
        "multilinestring" => |x| x,
        "multipoint" => cast_multilinestring_multipoint,
        "multipolygon" => cast_multilinestring_multipolygon,
        &_ => unimplemented!("for provided `to` geometry type"),
    };

    let res_vec = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                xi
            } else {
                f(Geom::from(xi)).into()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), to)
}

#[extendr]
fn cast_polygons(x: List, to: &str) -> Robj {
    if !x.inherits("rs_POLYGON") {
        panic!("`x` must be an `rs_POLYGON`")
    }

    let f = match to {
        "polygon" => |x| x,
        "multipolygon" => cast_polygon_multipolygon,
        "multipoint" => cast_polygon_multipoint,
        "linestring" => cast_polygon_linestring,
        "multilinestring" => cast_polygon_multilinestring,
        &_ => unimplemented!("for provided `to` geometry type"),
    };

    let res_vec = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                xi
            } else {
                f(Geom::from(xi)).into()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), to)
}

#[extendr]
fn cast_multipolygons(x: List, to: &str) -> Robj {
    
    if !x.inherits("rs_MULTIPOLYGON") {
        panic!("`x` must be an `rs_MULTIPOLYGON`")
    }

    let f = match to {
        "multipolygon" => |x| x,
        "multipoint" => cast_multipolygon_multipoint,
        "multilinestring" => cast_multipolygon_multilinestring,
        &_ => unimplemented!("for provided `to` geometry type"),
    };

    let res_vec = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                xi
            } else {
                f(Geom::from(xi)).into()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), to)

}




extendr_module! {
    mod cast;
    fn cast_points;
    fn cast_multipoints;
    fn cast_linestrings;
    fn cast_multilinestrings;
    fn cast_polygons;
    fn cast_multipolygons;
}
