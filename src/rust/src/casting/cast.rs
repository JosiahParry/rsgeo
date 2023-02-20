use extendr_api::prelude::*;
use geo::CoordsIter;
use geo_types::{MultiPolygon, MultiPoint, MultiLineString, Polygon, LineString, Point};
use crate::types::Geom;
use crate::to_pntr;
use crate::geoms::from_list;



//# cast 1 : 1 
//# expand 1 : many
//# combine many : 1 


// CASTING ------------------------------------------------------------------------
//                      to_point to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// from_point               TRUE          TRUE      FALSE           FALSE         FALSE              FALSE
// from_multipoint            NA          TRUE       TRUE            TRUE          TRUE               TRUE
// from_polygon               NA          TRUE       TRUE            TRUE          TRUE               TRUE
// from_multipolygon          NA          TRUE         NA            TRUE            NA               TRUE
// from_linestring            NA          TRUE      FALSE           FALSE          TRUE               TRUE
// from_multilinestring       NA          TRUE      FALSE           FALSE            NA               TRUE


// from_point               TRUE          TRUE      FALSE           FALSE         FALSE              FALSE

fn cast_point_multipoint(x: Robj) -> Robj {
    let res = MultiPoint::new(vec![Point::try_from(Geom::from(x).geom).unwrap()]);
    to_pntr(res.into())
}

#[extendr]
fn cast_point(x: Robj, to: &str) -> Robj {
    match to {
        "point" => x,
        "multipoint" => cast_point_multipoint(x),
        &_ => Robj::from(extendr_api::NA_LOGICAL) // if not matched return  self
    }
}

//                      to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// from_multipoint              TRUE       TRUE            TRUE          TRUE               TRUE

fn cast_multipoint_polygon(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let mut crds = x.0;
    crds.push(crds[0]);
    let ln = LineString::from(crds);
    to_pntr(Geom::from(Polygon::new(ln, vec![])))
}

#[extendr]
fn cast_multipoint_multipolygon(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let mut crds = x.0;
    crds.push(crds[0]);
    let ln = LineString::from(crds);
    to_pntr(Geom::from(MultiPolygon::new(vec![Polygon::new(ln, vec![])])))
}

fn cast_multipoint_linestring(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let crds = x.0;
    let ln = LineString::from(crds);
    to_pntr(Geom::from(ln))
}


fn cast_multipoint_multilinestring(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let crds = x.0;
    let ln = LineString::from(crds);
    to_pntr(Geom::from(MultiLineString::new(vec![ln])))
}


#[extendr]
fn cast_multipoint(x: Robj, to: &str) -> Robj {
    match to {
        "multipoint" => x,
        "polygon" => cast_multipoint_polygon(x),
        "multipolygon" => cast_multipoint_multipolygon(x),
        "linestring" => cast_multipoint_linestring(x),
        "multilinestring" => cast_multipoint_multilinestring(x),
        &_ => Robj::from(extendr_api::NA_LOGICAL)

    }
}



//                      to_point to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// from_polygon               NA          TRUE       TRUE            TRUE          TRUE               TRUE
fn cast_polygon_multipoint(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();
    let pnts = x
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::new(pnts)))

}


fn cast_polygon_multipolygon(x: Robj) -> Robj {
    to_pntr(Geom::from(MultiPolygon::try_from(Geom::from(x).geom).unwrap()))
}

fn cast_polygon_linestring(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();
    let pnts = x
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(LineString::from(pnts)))
}

fn cast_polygon_multilinestring(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();
    let (interrior, holes) = x.into_inner();
    let mut interrior = vec![interrior];
    interrior.extend(holes.into_iter());
    
    to_pntr(Geom::from(MultiLineString::new(interrior)))
}

#[extendr]
fn cast_polygon(x: Robj, to: &str) -> Robj {
    match to {
        "polygon" => x, 
        "multipolygon" => cast_polygon_multipolygon(x),
        "multipoint" => cast_polygon_multipoint(x),
        "linestring" => cast_polygon_linestring(x),
        "multilinestring" => cast_polygon_multilinestring(x),
        &_ => Robj::from(extendr_api::NA_LOGICAL)
    }
}

//                       to_multipoint  to_multipolygon  to_multilinestring
// from_multipolygon              TRUE             TRUE                TRUE
fn cast_multipolygon_multipoint(x: Robj) -> Robj {
    let mply = MultiPolygon::try_from(Geom::from(x).geom).unwrap();

    let pnts = mply
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    let res = MultiPoint::new(pnts);
    to_pntr(Geom::from(res))
}

fn cast_multipolygon_multilinestring(x: Robj) -> Robj {
    let mply = MultiPolygon::try_from(Geom::from(x).geom).unwrap();
    let linestrings = mply.0
        .into_iter()
        .map(|x| LineString::from_iter(x.coords_iter()))
        .collect::<Vec<LineString>>();

    to_pntr(Geom::from(MultiLineString::new(linestrings)))
}


#[extendr]
fn cast_multipolygon(x: Robj, to: &str) -> Robj {
    match to {
        "multipolygon" => x,
        "multipoint" => cast_multipolygon_multipoint(x),
        "multilinestring" => cast_multipolygon_multilinestring(x),
        &_ => Robj::from(extendr_api::NA_LOGICAL)
    }
}

//                      to_multipoint to_linestring to_multilinestring
// from_linestring               TRUE          TRUE               TRUE
fn cast_linestring_multipoint(x: Robj) -> Robj {
    to_pntr(
        Geom::from(
            LineString::try_from(Geom::from(x).geom).unwrap()
                .coords_iter()
                .collect::<MultiPoint>()
            )
        )
}

fn cast_linestring_polygon(x: Robj) -> Robj {
    let mut coords = LineString::try_from(Geom::from(x).geom).unwrap().0;
    coords.push(coords[0]);
    to_pntr(Geom::from(Polygon::new(LineString::from(coords), vec![])))
}


fn cast_linestring_multilinestring(x: Robj) -> Robj {
    to_pntr(Geom::from(MultiLineString::new(vec![LineString::try_from(Geom::from(x).geom).unwrap()])))
}

#[extendr]
fn cast_linestring(x: Robj, to: &str) -> Robj {
    match to {
        "linestring" => x,
        "multipoint" => cast_linestring_multipoint(x),
        "multilinestring" => cast_linestring_multilinestring(x),
        "polygon" => cast_linestring_polygon(x),
        &_ => Robj::from(extendr_api::NA_LOGICAL)
    }
}

//                      to_multipoint to_multilinestring
// from_multilinestring          TRUE               TRUE
fn cast_multilinestring_multipoint(x: Robj) -> Robj {
    let res: Geom = MultiPoint::from_iter(MultiLineString::try_from(
        Geom::from(x)
        .geom
    )
    .unwrap()
    .coords_iter())
    .into();

    to_pntr(res)
    
}

fn cast_multilinestring_multipolygon(x: Robj) -> Robj {
    let x = MultiLineString::try_from(Geom::from(x).geom).unwrap();
    let res = x.0
        .into_iter()
        .map(|lns| {
            let mut coords = lns.0;
            coords.push(coords[0]);
            Polygon::new(LineString::from(coords), vec![])

        })
        .collect::<Vec<Polygon>>();

    to_pntr(Geom::from(MultiPolygon::new(res)))
    
}

#[extendr]
fn cast_multilinestring(x: Robj, to: &str) -> Robj {
    match to {
        "multilinestring" => x,
        "multipoint" => cast_multilinestring_multipoint(x),
        "multipolygon" => cast_multilinestring_multipolygon(x),
        &_ => Robj::from(extendr_api::NA_LOGICAL)
    }
}


// COMBINE ------------------------------------------------------------------------

// building primitives up
// vec points -> Line
// vec points -> multipoint
#[extendr]
fn combine_points_line(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| Point::try_from(x.geom).unwrap())
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(LineString::from(x)))
}

#[extendr]
fn combine_points_multipoint(x: List) -> Robj {
    let x = from_list(x)
    .into_iter()
    .map(|x| Point::try_from(x.geom).unwrap())
    .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::from(x)))
}

// vec lines -> polygon
// vec lines -> multiline
#[extendr]
fn combine_lines_polygon(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| LineString::try_from(x.geom).unwrap())
        .collect::<Vec<LineString>>();

        // shoot do i need to handle winding here? 
        // assuming the user has the widning correct here
    let n = x.len();
    let exterior = x[0].clone();
    let interior = if n > 1 {
        x[1..n].to_owned()
    } else {
        vec![]
    };

    to_pntr(Geom::from(Polygon::new(exterior, interior)))
}

// #[extendr]
// fn cast_lines_multilinestring(x: List) -> Robj {
//     let x = from_list(x)
//         .into_iter()
//         .map(|x| LineString::try_from(x.geom).unwrap())
//         .collect::<Vec<LineString>>();

//         to_pntr(Geom::from(MultiLineString::new(x)))
// }

// #[extendr]
// fn cast_multiline_polygon(x: Robj) -> Robj {
//     let x = MultiLineString::try_from(Geom::from(x).geom).unwrap().0;
    
//     let n = x.len();
//     let exterior = x[0].clone();
//     let interior = if n > 1 {
//         x[1..n].to_owned()
//     } else {
//         vec![]
//     };

//     to_pntr(Geom::from(Polygon::new(exterior, interior)))
// }

// multiline -> polygon
// polygons -> multipolygon


extendr_module! {
    mod cast;
    fn cast_point;
    fn cast_multipoint;
    fn cast_linestring;
    fn cast_multilinestring;
    fn cast_polygon;
    fn cast_multipolygon;
}