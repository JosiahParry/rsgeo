use extendr_api::prelude::*;
use geo_types::*;
//use crate::utils::geom_class;
use crate::geoms::from_list;
use crate::to_pntr;
use crate::types::Geom;

// COMBINE ------------------------------------------------------------------------

// building primitives up
// vec points -> Line
// vec points -> multipoint

#[extendr]
fn combine_points(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| Point::try_from(x.geom).unwrap())
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::from(x)))
}

#[extendr]
fn combine_multipoints(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .flat_map(|x| MultiPoint::try_from(x.geom).unwrap().0)
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::from(x)))
}

#[extendr]
fn combine_linestrings(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| LineString::try_from(x.geom).unwrap())
        .collect::<Vec<LineString>>();

    to_pntr(Geom::from(MultiLineString::new(x)))
}

#[extendr]
fn combine_multilinestrings(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .flat_map(|x| MultiLineString::try_from(x.geom).unwrap().0)
        .collect::<Vec<LineString>>();

    to_pntr(Geom::from(MultiLineString::new(x)))
}

#[extendr]
fn combine_polygons(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| Polygon::try_from(x.geom).unwrap())
        .collect::<Vec<Polygon>>();

    to_pntr(Geom::from(MultiPolygon::new(x)))
}

#[extendr]
fn combine_multipolygons(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .flat_map(|x| MultiPolygon::try_from(x.geom).unwrap().0)
        .collect::<Vec<Polygon>>();

    to_pntr(Geom::from(MultiPolygon::new(x)))
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
