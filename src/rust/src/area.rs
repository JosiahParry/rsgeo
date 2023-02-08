use crate::geoms::*;
use extendr_api::prelude::*;

use geo::chamberlain_duquette_area::ChamberlainDuquetteArea;
use geo::Area;

#[extendr]
///@export
fn signed_area(x: List) -> Vec<f64> {
    let geoms = from_list(x);

    geoms
        .into_iter()
        .map(|geom| geom.geom.signed_area())
        .collect::<Vec<f64>>()
}

#[extendr]
///@export
fn unsigned_area(x: List) -> Vec<f64> {
    let geoms = from_list(x);

    geoms
        .into_iter()
        .map(|geom| geom.geom.unsigned_area())
        .collect::<Vec<f64>>()
}

#[extendr]
///@export
fn geodesic_signed_area(x: List) -> Vec<f64> {
    let geoms = from_list(x);

    geoms
        .into_iter()
        .map(|geom| geom.geom.chamberlain_duquette_signed_area())
        .collect::<Vec<f64>>()
}

#[extendr]
///@export
fn geodesic_unsigned_area(x: List) -> Vec<f64> {
    let geoms = from_list(x);

    geoms
        .into_iter()
        .map(|geom| geom.geom.chamberlain_duquette_unsigned_area())
        .collect::<Vec<f64>>()
}

// Macro to generate exports
extendr_module! {
    mod area;
    fn signed_area;
    fn unsigned_area;
    fn geodesic_signed_area;
    fn geodesic_unsigned_area;
}
