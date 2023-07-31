use extendr_api::prelude::*;
use sfconversions::Geom;

//use geo::vincenty_distance::FailedToConvergeError;
use geo::prelude::*;
use geo::{EuclideanLength, Geometry};

#[extendr]
// fn euclidean_length(x: Robj) -> f64 {
//     let x: Geom = x.into();
//     let geom = x.geom;

//     match geom {
//         Geometry::Line(geom) => geom.euclidean_length(),
//         Geometry::LineString(geom) => geom.euclidean_length(),
//         Geometry::MultiLineString(geom) => geom.euclidean_length(),
//         // if not line linestring or multilinestring return 0
//         _ => 0.,
//     }
// }

fn length_euclidean(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;

                match geom {
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
fn length_geodesic(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;

                match geom {
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
fn length_haversine(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;

                match geom {
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
fn length_vincenty(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;

                match geom {
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
}
