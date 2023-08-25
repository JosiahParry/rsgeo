use extendr_api::prelude::*;
use geo::FrechetDistance;
use geo_types::LineString;
use sfconversions::Geom;

#[extendr]
/// Calculate Frechet Distance
/// 
/// Given two LineStrings compare thier similarity 
/// by calculating the Fréchet distance. 
/// 
/// @param x an object of class `rs_LINESTRING` 
/// @param y an object of class `rs_LINESTRING` 
/// 
/// @returns
/// A numeric vector
/// @examples
/// x <- geom_linestring(1:10, runif(10, -1, 1))
/// y <- geom_linestring(1:10, runif(10, -3, 3))
/// frechet_distance(x, y)
/// @export 
fn frechet_distance(x: List, y: List) -> Doubles {
    if !x.inherits("rs_LINESTRING") || !y.inherits("rs_LINESTRING") {
        panic!("`x` and `y` but be an `rs_LINESTRING` object")
    }

    if x.len() != y.len() {
        panic!("`x` and `y` must be the same length")
    }
    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let l1: LineString = Geom::try_from(xi).unwrap().into();
                let l2: LineString = Geom::try_from(yi).unwrap().into();
                l1.frechet_distance(&l2).into()
            }
        })
        .collect::<Doubles>()
}
extendr_module! {
    mod similarity;
    fn frechet_distance;
}
