use extendr_api::prelude::*;
use sfconversions::{Geom, IntoGeom, vctrs::as_rsgeo_vctr};

use geo::{CoordsIter, Point};

#[extendr]
/// Coordinate Utilities
/// 
/// Utility functions for accessing coordinates from a geometry. 
/// 
/// @details
/// 
/// - `n_coords` returns the total number of coordinates in a geometry
/// - `coord_first()` returns the first coordinate in a geometry
/// - `coord_last()` returns the last coordinate in a geometry
/// - `coord_n()` returns the nth coordinate in a geometry
/// 
/// @returns an object of class `rs_POINT`. 
/// Whereas `n_coords()` returns an integer vector of the same length as `x`.
/// @param x an object of class `rsgeo`
/// @param n the index position of the coordinate
/// @export 
/// @rdname coord_utils
fn n_coords(x: List) -> Integers {
    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`")
    }

    x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rint::na()
            } else {
                let n = <&Geom>::from_robj(&xi).unwrap().geom.coords_count() as i32;
                Rint::from(n)
            }
        })
        .collect::<Integers>()
}

#[extendr]
/// @export
/// @rdname coord_utils
fn coord_last(x: List) -> Robj {

    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`")
    }

    let geo_class = sfconversions::vctrs::rsgeo_type(&x);

    let res_vec = x
    .into_iter()
    .map(|(_, xi)| {
        if xi.is_null() {
            NULL.into_robj()
        } else {
            let pnt: Point = <&Geom>::from_robj(&xi).unwrap().geom
                .coords_iter()
                .last()
                .unwrap()
                .into();
            
            pnt.into_geom().into_robj()

        }
    })
    .collect::<Vec<Robj>>();

    let res = List::from_values(res_vec);
    as_rsgeo_vctr(res, geo_class.as_str())
}

#[extendr]
/// @export
/// @rdname coord_utils
fn coord_first(x: List) -> Robj {

    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`")
    }

    let geo_class = sfconversions::vctrs::rsgeo_type(&x);

    let res_vec = x
    .into_iter()
    .map(|(_, xi)| {
        if xi.is_null() {
            NULL.into_robj()
        } else {
            let pnt: Point = <&Geom>::from_robj(&xi).unwrap().geom
                .coords_iter()
                .next()
                .unwrap()
                .into();
            pnt.into_geom().into_robj()

        }
    })
    .collect::<Vec<Robj>>();

    let res = List::from_values(res_vec);
    as_rsgeo_vctr(res, geo_class.as_str())
}

#[extendr]
fn coord_n_(x: List, n: Integers) -> Robj {

    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`")
    }

    let geo_class = sfconversions::vctrs::rsgeo_type(&x);

    let n_x = x.len();
    let n_n = n.len(); 

    if (n_x > n_n) && (n_n != 1) {
        panic!("`n` must be the same length as `x` or length 1")
    }

    let n = match n_n == 1 {
        true => Integers::from_values(vec![n[0]; n_x]),
        false => n
    };

    let res_vec = x
    .into_iter()
    .zip(n.iter())
    .map(|((_, xi), ni)| {
        if xi.is_null() || ni.is_na() {
            NULL.into_robj()
        } else {
            let crd = <&Geom>::from_robj(&xi).unwrap().geom
                .coords_iter()
                .nth((ni.inner() - 1) as usize);

            match crd {
                Some(c) => Point::from(c).into_geom().into_robj(),
                None => NULL.into_robj()
            }
        }
    })
    .collect::<Vec<Robj>>();

    let res = List::from_values(res_vec);
    as_rsgeo_vctr(res, geo_class.as_str())
}


extendr_module! {
    mod coord_utils;
    fn n_coords;
    fn coord_first;
    fn coord_last;
    fn coord_n_;
}