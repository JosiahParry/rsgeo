use extendr_api::prelude::*;
use geo::prelude::*;
use geo::{EuclideanLength, Geometry};
use rayon::prelude::*;
use sfconversions::{geometry_from_list, Geom};

#[extendr]
/// Calculate LineString Length
///
/// For a given LineString or MultiLineString geometry, calculate its length.
/// Other geometries will return a value of `NA`.
///
/// ### Notes
///
/// * Vicenty, Geodesic, and Haversine methods will return in units of meters.
/// * Geodesic length will always converge and is more accurate than the Vicenty methods.
/// * Haversine uses a mean earth radius of 6371.088 km.
///
/// See [`geo`](https://docs.rs/geo/latest/geo/index.html#length) docs for more details.
///
/// @param x an object of class `rsgeo`
///
/// @examples
/// set.seed(0)
/// y <- runif(25, -5, 5)
/// x <- 1:25
///
/// ln <- geom_linestring(x, y)
///
/// length_euclidean(ln)
/// length_geodesic(ln)
/// length_vincenty(ln)
/// length_haversine(ln)
/// @export
/// @rdname length
/// @returns A numeric vector
fn length_euclidean(x: List) -> Doubles {
    if !x.inherits("rsgeo") {
        panic!("`x` must be an object of class `rsgeo`")
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = <&Geom>::try_from(&xi).unwrap();

                match &geom.geom {
                    Geometry::Line(geom) => geom.euclidean_length().into(),
                    Geometry::LineString(geom) => geom.euclidean_length().into(),
                    Geometry::MultiLineString(geom) => geom.euclidean_length().into(),
                    _ => Rfloat::na(),
                }
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname length
fn length_geodesic(x: List) -> Doubles {
    if !x.inherits("rsgeo") {
        panic!("`x` must be an object of class `rsgeo`")
    }

    let x = geometry_from_list(x);

    let res_vec = x
        .into_par_iter()
        .map(|xi| match xi {
            Some(Geometry::Line(geom)) => Some(geom.geodesic_length()),
            Some(Geometry::LineString(geom)) => Some(geom.geodesic_length()),
            Some(Geometry::MultiLineString(geom)) => Some(geom.geodesic_length()),
            _ => None,
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
}

#[extendr]
/// @export
/// @rdname length
fn length_haversine(x: List) -> Doubles {
    if !x.inherits("rsgeo") {
        panic!("`x` must be an object of class `rsgeo`")
    }

    let x = geometry_from_list(x);

    let res_vec = x
        .into_par_iter()
        .map(|xi| match xi {
            Some(Geometry::Line(geom)) => Some(geom.haversine_length()),
            Some(Geometry::LineString(geom)) => Some(geom.haversine_length()),
            Some(Geometry::MultiLineString(geom)) => Some(geom.haversine_length()),
            _ => None,
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)
}

#[extendr]
/// @export
/// @rdname length
fn length_vincenty(x: List) -> Doubles {
    let x = geometry_from_list(x);

    let res_vec = x
        .into_par_iter()
        .map(|xi| match xi {
            Some(Geometry::Line(geom)) => geom.vincenty_length().ok(),
            Some(Geometry::LineString(geom)) => geom.vincenty_length().ok(),
            Some(Geometry::MultiLineString(geom)) => geom.vincenty_length().ok(),
            _ => None,
        })
        .collect::<Vec<Option<f64>>>();

    Doubles::from_values(res_vec)

    // x.iter()
    //     .map(|(_, xi)| {
    //         if xi.is_null() {
    //             Rfloat::na()
    //         } else {
    //             let geom = <&Geom>::try_from(&xi).unwrap();

    //             match &geom.geom {
    //                 Geometry::Line(geom) => match geom.vincenty_length().into() {
    //                     Ok(l) => l.into(),
    //                     Err(_) => Rfloat::na(),
    //                 },
    //                 Geometry::LineString(geom) => match geom.vincenty_length().into() {
    //                     Ok(l) => l.into(),
    //                     Err(_) => Rfloat::na(),
    //                 },
    //                 Geometry::MultiLineString(geom) => match geom.vincenty_length().into() {
    //                     Ok(l) => l.into(),
    //                     Err(_) => Rfloat::na(),
    //                 },
    //                 _ => Rfloat::na(),
    //             }
    //         }
    //     })
    //     .collect::<Doubles>()
}

extendr_module! {
    mod length;
    fn length_euclidean;
    fn length_geodesic;
    fn length_vincenty;
    fn length_haversine;
}
