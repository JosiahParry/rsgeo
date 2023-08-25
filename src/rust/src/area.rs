use extendr_api::prelude::*;

use geo::chamberlain_duquette_area::ChamberlainDuquetteArea;
use geo::Area;
use sfconversions::Geom;

#[extendr]
/// Calculate the area of a polygon
/// 
/// Functions to calculate different types of area for polygons. 
/// @param x an object of class `rsgeo`
/// @export
/// @rdname area
fn signed_area(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = <&Geom>::from_robj(&xi).unwrap().geom.signed_area();
                Rfloat::from(area)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname area
fn unsigned_area(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = <&Geom>::from_robj(&xi).unwrap().geom.unsigned_area();
                Rfloat::from(area)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname area
fn signed_area_geodesic(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = <&Geom>::from_robj(&xi)
                    .unwrap()
                    .geom
                    .chamberlain_duquette_signed_area();
                Rfloat::from(area)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname area
fn unsigned_area_geodesic(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = 
                <&Geom>::from_robj(&xi)
                    .unwrap()
                    .geom
                    .chamberlain_duquette_unsigned_area();
                Rfloat::from(area)
            }
        })
        .collect::<Doubles>()
}

// Macro to generate exports
extendr_module! {
    mod area;
    fn signed_area;
    fn unsigned_area;
    fn signed_area_geodesic;
    fn unsigned_area_geodesic;
}
