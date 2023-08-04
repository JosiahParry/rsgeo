use extendr_api::list;
use extendr_api::prelude::*;
use geo_types::*;
// use sfconversions::vctrs::geom_class;
//use crate::utils::geom_class;
use sfconversions::{
    vctrs::{as_rsgeo_vctr, verify_rsgeo},
    Geom,
};

// COMBINE ------------------------------------------------------------------------

// building primitives up
// vec points -> Line
// vec points -> multipoint

#[extendr]
fn combine_points(x: List) -> Robj {
    verify_rsgeo(&x);
    let x = x
        .into_iter()
        .map(|(_, x)| Point::try_from(Geom::from(x).geom).unwrap())
        .collect::<Vec<Point>>();

    let res = Geom::from(MultiPoint::from(x));
    as_rsgeo_vctr(list!(res), "point")
}

#[extendr]
fn combine_multipoints(x: List) -> Robj {
    verify_rsgeo(&x);
    let x = x
        .into_iter()
        .flat_map(|(_, x)| MultiPoint::try_from(Geom::from(x).geom).unwrap().0)
        .collect::<Vec<Point>>();

    let res = Geom::from(MultiPoint::from(x));
    as_rsgeo_vctr(list!(res), "multipoint")
}

#[extendr]
fn combine_linestrings(x: List) -> Robj {
    verify_rsgeo(&x);
    let x = x
        .into_iter()
        .map(|(_, x)| LineString::try_from(Geom::from(x).geom).unwrap())
        .collect::<Vec<LineString>>();

    let res = Geom::from(MultiLineString::new(x));
    as_rsgeo_vctr(list!(res), "linestring")
}

#[extendr]
fn combine_multilinestrings(x: List) -> Robj {
    verify_rsgeo(&x);
    let x = x
        .into_iter()
        .flat_map(|(_, x)| MultiLineString::from(Geom::from(x)).0)
        .collect::<Vec<LineString>>();

    let res = Geom::from(MultiLineString::new(x));
    as_rsgeo_vctr(list!(res), "multilinestring")
}

#[extendr]
fn combine_polygons(x: List) -> Robj {
    verify_rsgeo(&x);
    let x = x
        .into_iter()
        .map(|(_, x)| Polygon::try_from(Geom::from(x).geom).unwrap())
        .collect::<Vec<Polygon>>();

    let res = Geom::from(MultiPolygon::new(x));
    as_rsgeo_vctr(list!(res), "polygon")
}

#[extendr]
fn combine_multipolygons(x: List) -> Robj {
    verify_rsgeo(&x);
    let x = x
        .into_iter()
        .flat_map(|(_, x)| MultiPolygon::try_from(Geom::from(x).geom).unwrap().0)
        .collect::<Vec<Polygon>>();

    let res = Geom::from(MultiPolygon::new(x));
    as_rsgeo_vctr(list!(res), "multipolygon")
}

extendr_module! {
    mod combine;
    fn combine_points;
    fn combine_multipoints;
    fn combine_linestrings;
    fn combine_multilinestrings;
    fn combine_polygons;
    fn combine_multipolygons;
}
