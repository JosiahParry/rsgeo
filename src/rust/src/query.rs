use crate::geoms::*;
use crate::types::Geom;
use extendr_api::prelude::*;

use geo::{Bearing, Point};


/// Calculate Bearing
/// 
/// @param x an object of class `point`
/// @param y for `bearing()` an object of class `point`. For `bearings()` an object of class `rs_POINT`
/// 
/// @returns
/// A vector of doubles of the calculated bearing for between x and y
/// 
/// @export
#[extendr]
fn bearing(x: Robj, y: Robj) -> f64 {
    let x: Geom = x.into();
    let x: Point = x.into();

    let y: Geom = y.into();
    let y: Point = y.into();

    x.bearing(y)
}

#[extendr]
///@rdname bearing
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

