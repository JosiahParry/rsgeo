use extendr_api::prelude::*;
use geo::{Contains, Intersects, Within};

use crate::spatial_index::create_cached_rtree;
use rstar::RTreeObject;
use sfconversions::{Geom, geometry_from_list};

use rayon::prelude::*;
use std::sync::Mutex;


#[extendr]
/// Binary Predicates
/// 
/// Functions to ascertain the binary relationship between 
/// two geometry vectors. Binary predicates are provided both pairwise 
/// as a sparse matrix.
/// 
/// @param x an object of class `rsgeo`
/// @param y an object of class `rsgeo`
/// 
/// @export
/// @rdname topology
/// @examples
/// if (rlang::is_installed("sf")) {
///     nc <- sf::st_read(
///       system.file("shape/nc.shp", package = "sf"),
///       quiet = TRUE
///     )
///     
///     x <- as_rsgeo(nc$geometry[1:5])
///     y <- rev(x)
///     
///     # intersects
///     intersects_sparse(x, y)
///     intersects_pairwise(x, y)
///     # contains 
///     contains_sparse(x, y)
///     contains_pairwise(x, y)
///     # within
///     within_sparse(x, y)
///     within_pairwise(x, y)
/// }
/// @returns 
/// - For `_sparse` a list of integer vectors containing the position 
/// of the geometry in `y`
/// 
/// - For `_pairwise` a logical vector
fn intersects_sparse(x: List, y: List) -> List {

    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be of class `rsgeo`")
    }

    let n = x.len();
    let xtree = create_cached_rtree(x);

    let y = geometry_from_list(y);
    let index = Mutex::new(vec![Vec::with_capacity(n); n]);

    y
        .into_par_iter()
        .enumerate()
        .for_each(|(i, yi)| {
            if let Some(yi) = yi {
                let yi = Geom::from(yi);
                let env = yi.envelope();
                let cands = xtree.locate_in_envelope_intersecting(&env);

                cands.for_each(|cnd| {
                    if yi.geom.intersects(&cnd.geom().geom) {
                        let mut ind = index.lock().unwrap();
                        ind[cnd.data].push((i as i32) + 1)
                    }})
            }
        });

    let mut index = index.into_inner().unwrap();

    // we sort the results because the order will be different
    // each and every time. we need the result to be reproducible
    index
        .par_iter_mut()
        .for_each(|xi| xi.sort());   

    List::from_values(index)
}


#[extendr]
/// @export
/// @rdname topology
fn contains_sparse(x: List, y: List) -> List {

    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be of class `rsgeo`")
    }

    let n = x.len();
    let xtree = create_cached_rtree(x);

    let y = geometry_from_list(y);
    let index = Mutex::new(vec![Vec::with_capacity(n); n]);

    y
        .into_par_iter()
        .enumerate()
        .for_each(|(i, yi)| {
            if let Some(yi) = yi {
                let yi = Geom::from(yi);
                let env = yi.envelope();
                let cands = xtree.locate_in_envelope_intersecting(&env);

                cands.for_each(|cnd| {
                    if yi.geom.contains(&cnd.geom().geom) {
                        let mut ind = index.lock().unwrap();
                        ind[cnd.data].push((i as i32) + 1)
                    }})
            }
        });

    let mut index = index.into_inner().unwrap();

    index
        .par_iter_mut()
        .for_each(|xi| xi.sort());   

    List::from_values(index)
}

#[extendr]
/// @export
/// @rdname topology
fn within_sparse(x: List, y: List) -> List {

    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be of class `rsgeo`")
    }

    let n = x.len();
    let xtree = create_cached_rtree(x);

    let y = geometry_from_list(y);
    let index = Mutex::new(vec![Vec::with_capacity(n); n]);

    y
        .into_par_iter()
        .enumerate()
        .for_each(|(i, yi)| {
            if let Some(yi) = yi {
                let yi = Geom::from(yi);
                let env = yi.envelope();
                let cands = xtree.locate_in_envelope_intersecting(&env);

                cands.for_each(|cnd| {
                    if yi.geom.is_within(&cnd.geom().geom) {
                        let mut ind = index.lock().unwrap();
                        ind[cnd.data].push((i as i32) + 1)
                    }})
            }
        });

    let mut index = index.into_inner().unwrap();

    index
        .par_iter_mut()
        .for_each(|xi| xi.sort());   

    List::from_values(index)
}


#[extendr]
/// @export
/// @rdname topology
fn intersects_pairwise(x: List, y: List) -> Logicals {

    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be of class `rsgeo`")
    }

    let x = geometry_from_list(x);
    let y = geometry_from_list(y);
    
    let res = x
        .into_par_iter()
        .zip(y.into_par_iter())
        .map(|(xi, yi)| {
            if let Some(xi) = xi {
                if let Some(yi) = yi {
                    Some(xi.intersects(&yi))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<Option<bool>>>();

    Logicals::from_values(res)
}


#[extendr]
/// @export
/// @rdname topology
fn contains_pairwise(x: List, y: List) -> Logicals {

    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be of class `rsgeo`")
    }

    let x = geometry_from_list(x);
    let y = geometry_from_list(y);
    
    let res = x
        .into_par_iter()
        .zip(y.into_par_iter())
        .map(|(xi, yi)| {
            if let Some(xi) = xi {
                if let Some(yi) = yi {
                    Some(xi.contains(&yi))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<Option<bool>>>();
    
    Logicals::from_values(res)
}

#[extendr]
/// @export
/// @rdname topology
fn within_pairwise(x: List, y: List) -> Logicals {

    if !x.inherits("rsgeo") || !y.inherits("rsgeo") {
        panic!("`x` and `y` must be of class `rsgeo`")
    }

    let x = geometry_from_list(x);
    let y = geometry_from_list(y);
    
    let res = x
        .into_par_iter()
        .zip(y.into_par_iter())
        .map(|(xi, yi)| {
            if let Some(xi) = xi {
                if let Some(yi) = yi {
                    Some(xi.is_within(&yi))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<Option<bool>>>();
    
    Logicals::from_values(res)
}

// TODO
// disjoint
// DE-9IM relates

// relates use something called DE-9IM
// its an intersection matrix between two geometries.
// Each geometry is evaluated in 3 "parts"
// I: interior
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
    fn intersects_pairwise;
    fn contains_sparse;
    fn contains_pairwise;
    fn within_sparse;
    fn within_pairwise;
}



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