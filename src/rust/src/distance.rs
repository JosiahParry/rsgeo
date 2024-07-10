use extendr_api::prelude::*;
use geo_types::{Geometry, Point};
use rayon::prelude::*;
use sfconversions::Geom; // for parallel processing

use geo::{
    EuclideanDistance, GeodesicDistance, HausdorffDistance, HaversineDistance, VincentyDistance,
};

#[extendr]
/// Calculate Distances
///
/// Calculates distances between two vectors of geometries. There are
/// a number of different distance methods that can be utilized.
///
/// There are `_pairwise()` and `_matrix()` suffixed functions to
/// generate distances pairwise or as a dense matrix respectively.
/// The pairwise functions calculate distances between the ith element
/// of each vector. Whereas the matrix functions calculate the distance
/// between each and every geometry.
///
/// Euclidean distance should be used for planar geometries. Haversine,
/// Geodesic, and Vicenty are all methods of calculating distance
/// based on spherical geometries. There is no concept of spherical
/// geometries in rsgeo, so choose your distance measure appropriately.
///
/// ### Notes
///
/// * Hausdorff distance is calculated using Euclidean distance.
/// * Haversine, Geodesic, and Vicenty distances only work with `rs_POINT` geometries.
/// @param x and object of class `rsgeo`
/// @param y and object of class `rsgeo`
/// @export
/// @rdname distance
/// @examples
/// set.seed(1)
/// x <- geom_point(runif(5, -1, 1), runif(5, -1, 1))
/// y <- rev(x)
///
/// distance_euclidean_matrix(x, y)
/// distance_hausdorff_matrix(x, y)
/// distance_vicenty_matrix(x, y)
/// distance_geodesic_matrix(x, y)
/// distance_haversine_matrix(x, y)
///
/// distance_euclidean_pairwise(x, y)
/// distance_hausdorff_pairwise(x, y)
/// distance_vicenty_pairwise(x, y)
/// distance_geodesic_pairwise(x, y)
/// distance_haversine_pairwise(x, y)
/// @returns
///
/// For `_matrix` functions, returns a dense matrix of distances whereas `_pairwise`
/// functions return a numeric vector.
fn distance_euclidean_pairwise(x: List, y: List) -> Doubles {
    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be rsgeo geometries")
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = <&Geom>::try_from(&xi).unwrap();
                let yg = <&Geom>::try_from(&yi).unwrap();

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
    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be rsgeo geometries")
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = <&Geom>::try_from(&xi).unwrap();
                let yg = <&Geom>::try_from(&yi).unwrap();

                let d = xg.geom.hausdorff_distance(&yg.geom);
                Rfloat::from(d)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname distance
fn distance_vicenty_pairwise(x: List, y: List) -> Doubles {
    let x_is_point = x.inherits("rs_POINT");
    let y_is_point = x.inherits("rs_POINT");

    if !x_is_point || !y_is_point {
        panic!("`x` and `y` must be `rs_POINT` geometries")
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let xg = Point::from(Geom::from(xi));
                let yg = Point::from(Geom::from(yi));

                let d = xg.vincenty_distance(&yg);
                match d {
                    Ok(d) => Rfloat::from(d),
                    Err(_) => Rfloat::na(),
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

    x.iter()
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

    x.iter()
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
    fn distance_hausdorff_matrix;
    fn distance_vicenty_matrix;
    fn distance_geodesic_matrix;
    fn distance_haversine_matrix;
}

// TODO check if x and y are identical then only calculate
// one triangle
#[extendr]
/// @export
/// @rdname distance
fn distance_euclidean_matrix(x: List, y: List) -> Robj {
    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must both be `rsgeo` geometries")
    }

    let n_x = x.len();
    let n_y = y.len();

    let x = x
        .into_iter()
        .map(|(_, xi)| match <&Geom>::try_from(&xi) {
            Ok(g) => Some(g.geom.clone()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Geometry>>>();

    let y = y
        .into_iter()
        .map(|(_, yi)| match <&Geom>::try_from(&yi) {
            Ok(g) => Some(g.geom.clone()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Geometry>>>();

    let res_vec = y
        .into_par_iter()
        .flat_map(|yi| match yi {
            Some(yi) => x
                .iter()
                .map(|xi| match xi {
                    Some(xi) => Some(yi.euclidean_distance(xi)),
                    None => None,
                })
                .collect::<Vec<Option<f64>>>(),
            None => vec![None; n_y],
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
        .into_robj()
        .set_class(["matrix", "array"])
        .unwrap()
        .set_attrib("dim", [n_y, n_x])
        .unwrap()
        .clone()
        .into_robj()
}

// TODO check if x and y are identical then only calculate
// one triangle
#[extendr]
/// @export
/// @rdname distance
fn distance_hausdorff_matrix(x: List, y: List) -> Robj {
    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must both be `rsgeo` geometries")
    }

    let n_x = x.len();
    let n_y = y.len();

    let x = x
        .into_iter()
        .map(|(_, xi)| match <&Geom>::try_from(&xi) {
            Ok(g) => Some(g.geom.clone()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Geometry>>>();

    let y = y
        .into_iter()
        .map(|(_, yi)| match <&Geom>::try_from(&yi) {
            Ok(g) => Some(g.geom.clone()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Geometry>>>();

    let res_vec = y
        .into_par_iter()
        .flat_map(|yi| match yi {
            Some(yi) => x
                .iter()
                .map(|xi| match xi {
                    Some(xi) => Some(yi.hausdorff_distance(xi)),
                    None => None,
                })
                .collect::<Vec<Option<f64>>>(),
            None => vec![None; n_y],
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
        .into_robj()
        .set_class(["matrix", "array"])
        .unwrap()
        .set_attrib("dim", [n_y, n_x])
        .unwrap()
        .clone()
        .into_robj()
}

#[extendr]
/// @export
/// @rdname distance
fn distance_haversine_matrix(x: List, y: List) -> Robj {
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must both be `rs_POINT` geometries")
    }

    let n_x = x.len();
    let n_y = y.len();

    let x = x
        .into_iter()
        .map(|(_, xi)| match <&Geom>::try_from(&xi) {
            Ok(g) => Some(Point::try_from(g.geom.clone()).unwrap()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Point>>>();

    let y = y
        .into_iter()
        .map(|(_, yi)| match <&Geom>::try_from(&yi) {
            Ok(g) => Some(Point::try_from(g.geom.clone()).unwrap()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Point>>>();

    let res_vec = y
        .into_par_iter()
        .flat_map(|yi| match yi {
            Some(yi) => x
                .iter()
                .map(|xi| match xi {
                    Some(xi) => Some(yi.haversine_distance(xi)),
                    None => None,
                })
                .collect::<Vec<Option<f64>>>(),
            None => vec![None; n_y],
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
        .into_robj()
        .set_class(["matrix", "array"])
        .unwrap()
        .set_attrib("dim", [n_y, n_x])
        .unwrap()
        .clone()
        .into_robj()
}

#[extendr]
/// @export
/// @rdname distance
fn distance_vicenty_matrix(x: List, y: List) -> Robj {
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must both be `rs_POINT` geometries")
    }

    let n_x = x.len();
    let n_y = y.len();

    let x = x
        .into_iter()
        .map(|(_, xi)| match <&Geom>::try_from(&xi) {
            Ok(g) => Some(Point::try_from(g.geom.clone()).unwrap()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Point>>>();

    let y = y
        .into_iter()
        .map(|(_, yi)| match <&Geom>::try_from(&yi) {
            Ok(g) => Some(Point::try_from(g.geom.clone()).unwrap()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Point>>>();

    let res_vec = y
        .into_par_iter()
        .flat_map(|yi| match yi {
            Some(yi) => x
                .iter()
                .map(|xi| match xi {
                    Some(xi) => match yi.vincenty_distance(xi) {
                        Ok(r) => Some(r),
                        Err(_) => None,
                    },
                    None => None,
                })
                .collect::<Vec<Option<f64>>>(),
            None => vec![None; n_y],
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
        .into_robj()
        .set_class(["matrix", "array"])
        .unwrap()
        .set_attrib("dim", [n_y, n_x])
        .unwrap()
        .clone()
        .into_robj()
}

#[extendr]
/// @export
/// @rdname distance
fn distance_geodesic_matrix(x: List, y: List) -> Robj {
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must both be `rs_POINT` geometries")
    }

    let n_x = x.len();
    let n_y = y.len();

    let x = x
        .into_iter()
        .map(|(_, xi)| match <&Geom>::try_from(&xi) {
            Ok(g) => Some(Point::try_from(g.geom.clone()).unwrap()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Point>>>();

    let y = y
        .into_iter()
        .map(|(_, yi)| match <&Geom>::try_from(&yi) {
            Ok(g) => Some(Point::try_from(g.geom.clone()).unwrap()),
            Err(_) => None,
        })
        .collect::<Vec<Option<Point>>>();

    let res_vec = y
        .into_par_iter()
        .flat_map(|yi| match yi {
            Some(yi) => x
                .iter()
                .map(|xi| match xi {
                    Some(xi) => Some(yi.geodesic_distance(xi)),
                    None => None,
                })
                .collect::<Vec<Option<f64>>>(),
            None => vec![None; n_y],
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
        .into_robj()
        .set_class(["matrix", "array"])
        .unwrap()
        .set_attrib("dim", [n_y, n_x])
        .unwrap()
        .clone()
        .into_robj()
}
