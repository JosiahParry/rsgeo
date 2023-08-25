use extendr_api::prelude::*;
use sfconversions::Geom;
use geo::prelude::*;
use geo::{EuclideanLength, Geometry};

#[extendr]
/// Calculate LineString Length
/// 
/// For a given LineString or MultiLineString geometry, calculate its length. 
/// Other geometries will return a value of `NA`.
/// 
/// ### Notes
/// 
/// * Vicenty, Geodeisc, and Haversine methods will return in units of meters.
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
fn length_euclidean(x: List) -> Doubles {
    
    if !x.inherits("rsgeo") {
        panic!("`x` must be an object of class `rsgeo`")
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = <&Geom>::from_robj(&xi).unwrap();

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

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = <&Geom>::from_robj(&xi).unwrap();

                match &geom.geom {
                    Geometry::Line(geom) => geom.geodesic_length().into(),
                    Geometry::LineString(geom) => geom.geodesic_length().into(),
                    Geometry::MultiLineString(geom) => geom.geodesic_length().into(),
                    _ => Rfloat::na(),
                }
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname length
fn length_haversine(x: List) -> Doubles {

    if !x.inherits("rsgeo") {
        panic!("`x` must be an object of class `rsgeo`")
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = <&Geom>::from_robj(&xi).unwrap();

                match &geom.geom {
                    Geometry::Line(geom) => geom.haversine_length().into(),
                    Geometry::LineString(geom) => geom.haversine_length().into(),
                    Geometry::MultiLineString(geom) => geom.haversine_length().into(),
                    _ => Rfloat::na(),
                }
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname length
fn length_vincenty(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = <&Geom>::from_robj(&xi).unwrap();

                match &geom.geom {
                    Geometry::Line(geom) => match geom.vincenty_length().into() {
                        Ok(l) => l.into(),
                        Err(_) => Rfloat::na(),
                    },
                    Geometry::LineString(geom) => match geom.vincenty_length().into() {
                        Ok(l) => l.into(),
                        Err(_) => Rfloat::na(),
                    },
                    Geometry::MultiLineString(geom) => match geom.vincenty_length().into() {
                        Ok(l) => l.into(),
                        Err(_) => Rfloat::na(),
                    },
                    _ => Rfloat::na(),
                }
            }
        })
        .collect::<Doubles>()
}

extendr_module! {
    mod length;
    fn length_euclidean;
    fn length_geodesic;
    fn length_vincenty;
    fn length_haversine;
}
