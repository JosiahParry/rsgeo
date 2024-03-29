use extendr_api::prelude::*;
use geo_types::*;
use sfconversions::{vctrs::as_rsgeo_vctr, Geom};

// EXPAND -------------------------------------------------------------------------
// multis to the single varietys
#[extendr]
fn expand_multipolygon(x: Robj) -> Robj {
    let res = MultiPolygon::try_from(Geom::from(x).geom)
        .unwrap()
        .0
        .into_iter()
        .map(|p| Geom::from(p))
        .collect::<Vec<Geom>>();

    as_rsgeo_vctr(List::from_values(res), "polygon")
}

#[extendr]
fn expand_multipoint(x: Robj) -> Robj {
    let res = MultiPoint::try_from(Geom::from(x).geom)
        .unwrap()
        .0
        .into_iter()
        .map(|p| Geom::from(p))
        .collect::<Vec<Geom>>();

    as_rsgeo_vctr(List::from_values(res), "point")
}

#[extendr]
fn expand_multilinestring(x: Robj) -> Robj {
    let res = MultiLineString::try_from(Geom::from(x).geom)
        .unwrap()
        .0
        .into_iter()
        .map(|p| Geom::from(p))
        .collect::<Vec<Geom>>();

    as_rsgeo_vctr(List::from_values(res), "linestring")
}

// // primitives to components
// // polygon to linestring

#[extendr]
fn expand_polygon(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();

    let rings = x.into_inner();
    let mut res_vec = vec![rings.0];

    res_vec.extend(rings.1);

    let res = res_vec
        .into_iter()
        .map(|i| Geom::from(i))
        .collect::<Vec<Geom>>();

    as_rsgeo_vctr(List::from_values(res), "linestring")
}

#[extendr]
fn expand_linestring(x: Robj) -> Robj {
    let x = LineString::try_from(Geom::from(x).geom).unwrap();

    let res = x
        .into_points()
        .into_iter()
        .map(|i| Geom::from(i))
        .collect::<Vec<Geom>>();

    as_rsgeo_vctr(List::from_values(res), "point")
}

#[extendr]
/// Expand Geometries
///
/// Expands geometries into a list of vectors of their components.
///
/// @param x an object of class `rsgeo`
///
/// @details
///
/// - `rs_MULTIPOINT` expands into a vector of points
/// - `rs_LINESTRING` expands into a vector points
/// - `rs_MULTILINESTRING` expands into a vector of linestrings
/// - `rs_POLYGON` expands into a vector of linestrings
/// - `rs_MULTIPOLYGON` expands into a vector of polygons
///
/// If you wish to have a single vector returned, pass the results
/// into `flatten_geoms()`.
///
/// @returns
///
/// A list of `rsgeo` vectors containing each original geometry's
/// components as a new vector.
///
/// @export
/// @examples
/// mpnts <- geom_multipoint(runif(10), runif(10), rep.int(1:5, 2))
/// expand_geoms(mpnts)
fn expand_geoms(x: List) -> List {
    let cls = x.class().unwrap().next().unwrap();
    let f = match cls {
        // "rs_POINT" => x,
        "rs_MULTIPOINT" => expand_multipoint,
        "rs_LINESTRING" => expand_linestring,
        "rs_MULTILINESTRING" => expand_multilinestring,
        "rs_POLYGON" => expand_polygon,
        "rs_MULTIPOLYGON" => expand_multipolygon,
        &_ => unimplemented!("not implemented for {}", cls),
    };

    let res = x
        .into_iter()
        .map(|(_, robj)| if robj.is_null() { robj } else { f(robj) })
        .collect::<Vec<Robj>>();

    List::from_values(res)
}

// Expansion Hierarchy
// MultiPolygon -> Polygon
// (Polygon -> LineString) OR (MultiLineString -> LineString)
// LineString -> Point

extendr_module! {
    mod expand;
    fn expand_linestring;
    fn expand_multipolygon;
    fn expand_multilinestring;
    fn expand_multipoint;
    fn expand_polygon;
    fn expand_geoms;
}
