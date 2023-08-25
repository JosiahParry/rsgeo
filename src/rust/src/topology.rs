use extendr_api::prelude::*;
use geo::{Contains, Intersects, Within};

use crate::spatial_index::create_cached_rtree;
use rstar::RTreeObject;
use sfconversions::Geom;

// This approach is generally slow it works by building two R* trees.
// using a cached envelope is faster than using a  computer bounding box so we
// opt to using the cached variant
// #[extendr]
// fn intersects_sparse_cached2(x: List, y: List) -> List {
//     let n = x.len();
//     let xtree  = create_cached_rtree(x);

//     let ytree = create_cached_rtree(y);

//     let candidates = xtree.intersection_candidates_with_other_tree(&ytree);

//     let mut index = vec![Vec::with_capacity(n); n];

//     for (x, y) in candidates {
//         let xid = x.data;
//         let yid = y.data;
//         let res = x.geom().geom.intersects(&y.geom().geom);
//         if res {
//             index[xid].push((yid as i32) + 1i32)
//         }
//     }
//     List::from_values(index)

// }
// TODO use rayon
#[extendr]
fn intersects_sparse(x: List, y: List) -> List {
    let n = x.len();
    let xtree = create_cached_rtree(x);

    let mut index = vec![Vec::with_capacity(n); n];

    for (i, (_, y)) in y.iter().enumerate() {
        let yi = Geom::try_from(y).unwrap();
        let env = yi.envelope();
        let cands = xtree.locate_in_envelope_intersecting(&env);

        // iterate through all candidates
        cands.for_each(|cnd| {
            if yi.geom.intersects(&cnd.geom().geom) {
                index[cnd.data].push((i as i32) + 1)
            }
        });
    }
    List::from_values(index)
}

#[extendr]
fn contains_sparse(x: List, y: List) -> List {
    let n = x.len();
    let xtree = create_cached_rtree(x);

    let mut index = vec![Vec::with_capacity(n); n];

    for (i, (_, y)) in y.iter().enumerate() {
        let yi = Geom::try_from(y).unwrap();
        let env = yi.envelope();
        let cands = xtree.locate_in_envelope_intersecting(&env);

        // iterate through all candidates
        cands.for_each(|cnd| {
            if yi.geom.contains(&cnd.geom().geom) {
                index[cnd.data].push((i as i32) + 1)
            }
        });
    }
    List::from_values(index)
}

#[extendr]
fn within_sparse(x: List, y: List) -> List {
    let n = x.len();
    let xtree = create_cached_rtree(x);

    let mut index = vec![Vec::with_capacity(n); n];

    for (i, (_, y)) in y.iter().enumerate() {
        let yi = Geom::try_from(y).unwrap();
        let env = yi.envelope();
        let cands = xtree.locate_in_envelope_intersecting(&env);

        // iterate through all candidates
        cands.for_each(|cnd| {
            if yi.geom.is_within(&cnd.geom().geom) {
                index[cnd.data].push((i as i32) + 1)
            }
        });
    }
    List::from_values(index)
}

// relates use something called DE-9IM
// its an intersection matrix between two geometries.
// Each geometry is evaluated in 3 "parts"
// I: interrior
// B: boundary
// E: exterior
//
// There are 4 types of intersections:
// F: false, or none
// 0: a single point
// 1: a shared line
// 3: an area intersection

extendr_module! {
    mod topology;
    fn intersects_sparse;
    fn contains_sparse;
    fn within_sparse;
}
