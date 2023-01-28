use crate::geoms::*;
use crate::types::Geom;
use extendr_api::prelude::*;

use geo::{Bearing, Point};

#[extendr]
///@export
fn bearing(x: Robj, y: Robj) -> f64 {
    let x: Geom = x.into();
    let x: Point = x.into();

    let y: Geom = y.into();
    let y: Point = y.into();

    x.bearing(y)
}

#[extendr]
///@export
fn bearings(x: Robj, y: List) -> Vec<f64> {
    let points = from_list(y);

    let x: Geom = x.into();
    let x: Point = x.into();

    points.into_iter()
        .map(|pnt| 
            x.bearing(pnt.geom.try_into().unwrap())
        )
        .collect::<Vec<f64>>()

}

extendr_module! {
    mod query;
    fn bearing;
    fn bearings;
}

