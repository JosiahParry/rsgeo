use extendr_api::prelude::*;
use geo::LinesIter;
use geo_types::*;
use sfconversions::{vctrs::as_rsgeo_vctr, Geom};

#[extendr]
fn explode_linestrings_(x: List) -> Robj {
    let res_vec = x
        .into_iter()
        .flat_map(|(_, xi)| {
            if xi.is_null() {
                vec![().into_robj()]
            } else {
                let xi = Geom::from(xi);
                let li = LineString::try_from(xi.geom).unwrap();

                li.lines()
                    .map(|li| Geom::from(LineString::from(li)).into_robj())
                    .collect::<Vec<Robj>>()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), "linestring")
}

#[extendr]
fn explode_multilinestrings_(x: List) -> Robj {
    let res_vec = x
        .into_iter()
        .flat_map(|(_, xi)| {
            if xi.is_null() {
                vec![().into_robj()]
            } else {
                let xi = Geom::from(xi);
                let li = MultiLineString::try_from(xi.geom).unwrap();
                li.lines_iter()
                    .map(|li| Geom::from(LineString::from(li)).into_robj())
                    .collect::<Vec<Robj>>()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), "linestring")
}

extendr_module! {
    mod explode;
    fn explode_linestrings_;
    fn explode_multilinestrings_;
}
