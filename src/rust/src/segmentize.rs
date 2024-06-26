use extendr_api::prelude::*;
use geo::{LineStringSegmentize, LineStringSegmentizeHaversine};
use rayon::prelude::*;
use sfconversions::{geometry_from_list, vctrs::as_rsgeo_vctr, Geom};

use geo_types::{LineString, MultiLineString};

// wrapped and documented externally
#[extendr]
fn line_segmentize_(x: List, n: Integers) -> Robj {
    let n_x = x.len();
    let n_n = n.len();

    if (n_x > n_n) && (n_n != 1) {
        panic!("`n` must be the same length as `x` or length 1")
    }

    let n = match n_n == 1 {
        true => Integers::from_values(vec![n[0]; n_x]),
        false => n,
    };

    let x = geometry_from_list(x);

    let res_vec = x
        .into_par_iter()
        .zip(n.into_par_iter())
        .map(|(xi, ni)| {
            if ni.is_na() {
                None
            } else {
                match xi {
                    Some(g) => LineString::try_from(g)
                        .unwrap()
                        .line_segmentize(ni.inner() as usize),
                    None => None,
                }
            }
        })
        .collect::<Vec<Option<MultiLineString>>>();

    let res = res_vec
        .into_iter()
        .map(|xi| match xi {
            Some(xi) => Geom::from(xi).into_robj(),
            None => ().into_robj(),
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res), "multilinestring")
}

#[extendr]
fn line_segmentize_haversine_(x: List, n: Integers) -> Robj {
    let n_x = x.len();
    let n_n = n.len();

    if (n_x > n_n) && (n_n != 1) {
        panic!("`n` must be the same length as `x` or length 1")
    }

    let n = match n_n == 1 {
        true => Integers::from_values(vec![n[0]; n_x]),
        false => n,
    };

    let x = geometry_from_list(x);

    let res_vec = x
        .into_par_iter()
        .zip(n.into_par_iter())
        .map(|(xi, ni)| {
            if ni.is_na() {
                None
            } else {
                match xi {
                    Some(g) => LineString::try_from(g)
                        .unwrap()
                        .line_segmentize_haversine(ni.inner() as usize),
                    None => None,
                }
            }
        })
        .collect::<Vec<Option<MultiLineString>>>();

    let res = res_vec
        .into_iter()
        .map(|xi| match xi {
            Some(xi) => Geom::from(xi).into_robj(),
            None => ().into_robj(),
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res), "multilinestring")
}

extendr_module! {
    mod segmentize;
    fn line_segmentize_;
    fn line_segmentize_haversine_;
}
