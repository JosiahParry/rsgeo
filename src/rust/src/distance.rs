use extendr_api::prelude::*;

use sfconversions::{
    Geom,
    vctrs::is_rsgeo
};


use geo_types::{Point, Geometry};

use geo::{
    EuclideanDistance, 
    HausdorffDistance,
    GeodesicDistance, 
    HaversineDistance, 
    VincentyDistance
};


#[extendr]
/// @export
/// @rdname distance
fn distance_euclidean_pairwise(x: List, y: List) -> Doubles {
    let x_is_geo = match is_rsgeo(&x).inner() {
        1_i32 => true,
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
    
                let d = xg.geom.euclidean_distance(&yg.geom);
                Rfloat::from(d)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname distance
fn distance_hausdorff_pairwise(x: List, y: List) -> Doubles {

    let x_is_geo = match is_rsgeo(&x).inner() {
        1_i32 => true,
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

#[extendr]
fn distance_vicenty_pairwise(x: List, y: List) -> Doubles {

    let x_is_point = x.inherits("rs_POINT");
    let y_is_point = x.inherits("rs_POINT");


    if !x_is_point || !y_is_point {
        panic!("`x` and `y` must be `rs_POINT` geometries")
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {

            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = Point::from(Geom::from(xi));
                let yg = Point::from(Geom::from(yi));

                let d= xg.vincenty_distance(&yg);
                match d {
                    Ok(d) => Rfloat::from(d),
                    Err(_) => Rfloat::na()
                }
            }
        })
        .collect::<Doubles>()
}


#[extendr]
/// @export
/// @rdname distance
fn distance_geodesic_pairwise(x: List, y: List) -> Doubles {

    let x_is_point = x.inherits("rs_POINT");
    let y_is_point = x.inherits("rs_POINT");


    if !x_is_point || !y_is_point {
        panic!("`x` and `y` must be `rs_POINT` geometries")
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {

            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = Point::from(Geom::from(xi));
                let yg = Point::from(Geom::from(yi));

                Rfloat::from(xg.geodesic_distance(&yg))
            }
        })
        .collect::<Doubles>()
}


#[extendr]
/// @export
/// @rdname distance
fn distance_haversine_pairwise(x: List, y: List) -> Doubles {

    let x_is_point = x.inherits("rs_POINT");
    let y_is_point = x.inherits("rs_POINT");


    if !x_is_point || !y_is_point {
        panic!("`x` and `y` must be `rs_POINT` geometries")
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {

            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = Point::from(Geom::from(xi));
                let yg = Point::from(Geom::from(yi));

                Rfloat::from(xg.haversine_distance(&yg))
            }
        })
        .collect::<Doubles>()
}


// Exporting
extendr_module! {
    mod distance;
    fn distance_euclidean_pairwise;
    fn distance_hausdorff_pairwise;
    fn distance_vicenty_pairwise;
    fn distance_geodesic_pairwise;
    fn distance_haversine_pairwise;
    fn distance_euclidean_matrix;
}


// lets go nuts and use rayon
use rayon::prelude::*;

#[extendr]
fn distance_euclidean_matrix(x: List, y: List) -> List {

    let n_y = y.len();
    
    let x = x
        .into_iter()
        .map(|(_, xi)| {
            match <&Geom>::from_robj(&xi) {
                Ok(g) => Some(g.geom.clone()),
                Err(_) => None
            }
        }).collect::<Vec<Option<Geometry>>>();

    let y = y
        .into_iter()
        .map(|(_, yi)| {
            match <&Geom>::from_robj(&yi) {
                Ok(g) => Some(g.geom.clone()),
                Err(_) => None
            }
        }).collect::<Vec<Option<Geometry>>>();
    
    
    let res_vec = x
        .into_par_iter()
        .map(|xi| {

            match xi {
                Some(xi) => {
                    y.iter().map(|yi| {
                        match yi {
                            Some(yi) => Some(xi.euclidean_distance(yi)),
                            None => None
                        }
                    }).collect::<Vec<Option<f64>>>()
                },
                None => vec![None; n_y]
            }
        })
        .collect::<Vec<Vec<Option<f64>>>>();

    let res = res_vec
        .into_iter()
        .map(Doubles::from_values)
        .collect::<Vec<Doubles>>();

    List::from_values(res)

}



