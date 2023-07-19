use extendr_api::prelude::*;
use geo::{PreparedGeometry, Relate};

use crate::spatial_index::{
    // create_rtree,
     create_cached_rtree
};
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


// #[extendr]
// fn intersects_sparse(x: List, y: List) -> List {
//     let n = x.len();
//     let xtree  = create_rtree(x);

//     let mut index = vec![Vec::with_capacity(n); n];

//     for (i, (_, y)) in y.iter().enumerate() {
//         let yi = Geom::try_from(y).unwrap();
//         let env = yi.envelope();
//         let cands = xtree.locate_in_envelope_intersecting(&env);

//         // iterate through all candidates
//         cands
//             .for_each(|cnd| {
//                 if yi.geom.intersects(&cnd.geom().geom) {
//                     index[cnd.data].push((i as i32) + 1)
//                 }
//             });

//     }
//     List::from_values(index)
// }

#[extendr]
fn intersects_sparse(x: List, y: List) -> List {
    let n = x.len();
    let xtree  = create_cached_rtree(x);

    let mut index = vec![Vec::with_capacity(n); n];

    for (i, (_, y)) in y.iter().enumerate() {
        let yi = Geom::try_from(y).unwrap();
        let env = yi.envelope();
        let cands = xtree.locate_in_envelope_intersecting(&env);

        let prepared_y = PreparedGeometry::from(&yi.geom);
        // iterate through all candidates
        cands
            .for_each(|cnd| {
                if prepared_y.relate(&cnd.data.1).is_intersects() {
                    index[cnd.data.0].push((i as i32) + 1)
                }
            });

    }
    List::from_values(index)
}


#[extendr]
fn contains_sparse(x: List, y: List) -> List {
    let n = x.len();
    let xtree  = create_cached_rtree(x);

    let mut index = vec![Vec::with_capacity(n); n];

    for (i, (_, y)) in y.iter().enumerate() {
        let yi = Geom::try_from(y).unwrap();
        let prepared_y = PreparedGeometry::from(&yi.geom);
        let env = yi.envelope();
        let cands = xtree.locate_in_envelope_intersecting(&env);

        // iterate through all candidates
        cands
            .for_each(|cnd| {
                if prepared_y.relate(&cnd.data.1).is_contains() {
                    index[cnd.data.0].push((i as i32) + 1)
                }
            });
    }
    List::from_values(index)
}



#[extendr]
fn within_sparse(x: List, y: List) -> List {
    let n = x.len();
    let xtree  = create_cached_rtree(x);

    let mut index = vec![Vec::with_capacity(n); n];

    for (i, (_, y)) in y.iter().enumerate() {
        let yi = Geom::try_from(y).unwrap();
        let prepared_y = PreparedGeometry::from(&yi.geom);
        let env = yi.envelope();
        let cands = xtree.locate_in_envelope_intersecting(&env);

        // iterate through all candidates
        cands
            .for_each(|cnd| {
                if prepared_y.relate(&cnd.data.1).is_within() {
                    index[cnd.data.0].push((i as i32) + 1)
                }
            });

    }
    List::from_values(index)
}


extendr_module! {
    mod topology;
    // fn contains; // vectorized along y
    // fn contains_sparse;
    // fn contains_pairwise;
    // fn intersects;
    // fn intersects_sparse_cached2;
    // fn intersects_sparse_cached;
    fn intersects_sparse;
    fn contains_sparse;
    // fn contains_sparse_cached;
    fn within_sparse;
    // fn within_sparse_cached;
    // fn intersects_sparse2;
    // fn intersects_pairwise;
    // fn within;
    // fn within_sparse;
    // fn within_pairwise;
}
