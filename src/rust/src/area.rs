use extendr_api::prelude::*;

use geo::chamberlain_duquette_area::ChamberlainDuquetteArea;
use geo::Area;
use geo::GeodesicArea;
use sfconversions::Geom;

#[extendr]
/// Calculate the area of a polygon
///
/// Functions to calculate different types of area for polygons.
///
/// @param x an object of class `rsgeo`
/// @export
/// @rdname area
/// @returns a numeric vector of the area contained by the geometry
/// @details
///
/// - functions assume counter clock-wise winding in accordance with the simple feature
/// access standard
/// - functions ending in `_cd` use the Chamberlain-Duquette algorithm for spherical area
/// - Chamberlain-Duquette and Geodesic areas are returned in meters squared and assume non-planar geometries
///
/// See geo docs for more:
///
/// - [GeodesicArea](https://docs.rs/geo/latest/geo/algorithm/geodesic_area/trait.GeodesicArea.html#)
/// - [Area](https://docs.rs/geo/latest/geo/algorithm/area/trait.Area.html#)
/// - [ChamberlainDuquetteArea](https://docs.rs/geo/latest/geo/algorithm/chamberlain_duquette_area/trait.ChamberlainDuquetteArea.html)
///
/// @examples
/// x <- c(0, 1, 1, 0, 0)
/// y <- c(0, 0, 1, 1, 0)
/// p <- geom_polygon(x, y)
///
/// signed_area(p)
/// unsigned_area(p)
/// signed_area_cd(p)
/// unsigned_area_cd(p)
/// signed_area_geodesic(p)
/// unsigned_area_geodesic(p)
fn signed_area(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = <&Geom>::try_from(&xi).unwrap().geom.signed_area();
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
                let area = <&Geom>::try_from(&xi).unwrap().geom.unsigned_area();
                Rfloat::from(area)
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname area
fn signed_area_cd(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = <&Geom>::try_from(&xi)
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
fn unsigned_area_cd(x: List) -> Doubles {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rfloat::na()
            } else {
                let area = <&Geom>::try_from(&xi)
                    .unwrap()
                    .geom
                    .chamberlain_duquette_unsigned_area();
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
                let area = <&Geom>::try_from(&xi)
                    .unwrap()
                    .geom
                    .geodesic_area_unsigned();
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
                let area = <&Geom>::try_from(&xi).unwrap().geom.geodesic_area_signed();
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
    fn signed_area_cd;
    fn unsigned_area_cd;
    fn signed_area_geodesic;
    fn unsigned_area_geodesic;
}
