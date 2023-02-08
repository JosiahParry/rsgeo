//use crate::geoms::*;
use crate::types::*;
use extendr_api::prelude::*;

//use geo::vincenty_distance::FailedToConvergeError;
use geo::prelude::*;
use geo::{EuclideanLength, Geometry};

#[extendr]
fn euclidean_length(x: Robj) -> f64 {
    let x: Geom = x.into();
    let geom = x.geom;

    match geom {
        Geometry::Line(geom) => geom.euclidean_length(),
        Geometry::LineString(geom) => geom.euclidean_length(),
        Geometry::MultiLineString(geom) => geom.euclidean_length(),
        // if not line linestring or multilinestring return 0
        _ => 0.,
    }
}

#[extendr]
fn geodesic_length(x: Robj) -> f64 {
    let x: Geom = x.into();
    let geom = x.geom;

    match geom {
        Geometry::Line(geom) => geom.geodesic_length(),
        Geometry::LineString(geom) => geom.geodesic_length(),
        Geometry::MultiLineString(geom) => geom.geodesic_length(),
        // if not line linestring or multilinestring return 0
        _ => 0.,
    }
}

#[extendr]
fn haversine_length(x: Robj) -> f64 {
    let x: Geom = x.into();
    let geom = x.geom;

    match geom {
        Geometry::Line(geom) => geom.haversine_length(),
        Geometry::LineString(geom) => geom.haversine_length(),
        Geometry::MultiLineString(geom) => geom.haversine_length(),
        // if not line linestring or multilinestring return 0
        _ => 0.,
    }
}

#[extendr]
fn vincenty_length(x: Robj) -> f64 {
    let x: Geom = x.into();
    let geom = x.geom;

    match geom {
        Geometry::Line(geom) => geom.vincenty_length().unwrap(),
        Geometry::LineString(geom) => geom.vincenty_length().unwrap(),
        Geometry::MultiLineString(geom) => geom.vincenty_length().unwrap(),
        // if not line linestring or multilinestring return 0
        _ => 0.,
    }
}

extendr_module! {
    mod length;
    fn euclidean_length;
    fn geodesic_length;
    fn vincenty_length;
}
