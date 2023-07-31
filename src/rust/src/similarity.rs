use extendr_api::prelude::*;
use geo::FrechetDistance;
use geo_types::LineString;
use sfconversions::Geom;

#[extendr]
fn frechet_distance(x: List, y: List) -> Doubles {
    if !x.inherits("rs_LINESTRING") || !y.inherits("rs_LINESTRING") {
        panic!("`x` and `y` but be an `rs_LINESTRING` object")
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
