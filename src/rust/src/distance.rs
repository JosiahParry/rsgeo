use extendr_api::prelude::*;

use sfconversions::{
    Geom,
    vctrs::is_rsgeo
};


use geo::{
    EuclideanDistance, HausdorffDistance,
    // HausdorffDistance
    // , GeodesicDistance, HaversineDistance, VincentyDistance
};


#[extendr]
fn euclidean_distance_pairwise(x: List, y: List) -> Doubles {
    let x_is_geo = match is_rsgeo(&x).inner() {
        0_i32 => false,
        1_i32 => true,
        i32::MAX => false,
        _ => false
    };

    let y_is_geo = match is_rsgeo(&y).inner() {
        0_i32 => false,
        1_i32 => true,
        i32::MAX => false,
        _ => false
    };

    if !x_is_geo || !y_is_geo {
        panic!("`x` and `y` must be rsgeo geometries")
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {

            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = <&Geom>::from_robj(&xi).unwrap();
                let yg = <&Geom>::from_robj(&yi).unwrap();
    
                let d = xg.geom.euclidean_distance(&yg.geom);
                Rfloat::from(d)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
fn hausdorff_distance_pairwise(x: List, y: List) -> Doubles {

    let x_is_geo = match is_rsgeo(&x).inner() {
        0_i32 => false,
        1_i32 => true,
        i32::MAX => false,
        _ => false
    };

    let y_is_geo = match is_rsgeo(&y).inner() {
        1_i32 => true,
        _ => false
    };

    if !x_is_geo || !y_is_geo {
        panic!("`x` and `y` must be rsgeo geometries")
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {

            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = <&Geom>::from_robj(&xi).unwrap();
                let yg = <&Geom>::from_robj(&yi).unwrap();
    
                let d = xg.geom.hausdorff_distance(&yg.geom);
                Rfloat::from(d)
            }
        })
        .collect::<Doubles>()
}

// Exporting
extendr_module! {
    mod distance;
    fn euclidean_distance_pairwise;
    fn hausdorff_distance_pairwise;
}
