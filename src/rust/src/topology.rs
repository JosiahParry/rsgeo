use geo::{Contains, Intersects, Within};
use extendr_api::prelude::*;

use crate::types::Geom;
use crate::{geoms::from_list, spatial_index::create_rtree};

// Contains
#[extendr]
/// Spatial Predicates
/// @export
/// @rdname predicates
fn contains_sparse(x: List, y: List) -> List {
    let x = from_list(x);
    let y = from_list(y);

    let xtree = create_rtree(x.clone());
    let ytree = create_rtree(y.clone());

    let cands = xtree.intersection_candidates_with_other_tree(&ytree);

    let res_cands = cands
            .map(|(x, y)| (x.data, y.data))
            .collect::<Vec<(usize, usize)>>();
    
    // need to create a sparse representation now 
    let nx = x.len();
    let ny = y.len();
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(nx);
    
    // allocate internal vecs
    for _ in 0..nx {
        res.push(Vec::with_capacity(ny))
    }

    for (xin, yin) in res_cands.into_iter() {
        //if yin == 0 { continue; } 
        if x[xin].geom.contains(&y[yin].geom) { res[xin].push((yin as i32) + 1) }
    }

    List::from_values(res)


}


#[extendr]
/// @export
/// @rdname predicates
fn contains(x: Robj, y: List) -> Vec<bool> {
    let x = Geom::from(x);
    let y = from_list(y);

    y
        .into_iter()
        .map(|y| x.geom.contains(&y.geom))
        .collect::<Vec<bool>>()
}

#[extendr]
/// @export
/// @rdname predicates
fn contains_pairwise(x: List, y: List) -> Vec<bool> {
    let x = from_list(x);
    let y = from_list(y);

    x
        .into_iter()
        .enumerate()
        .map(|(i, xi)| xi.geom.contains(&y[i].geom))
        .collect::<Vec<bool>>()
}


// Intersects ------------------------------------------------------------------------

#[extendr]
/// @export
/// @rdname predicates
fn intersects_sparse(x: List, y: List) -> List {
    let x = from_list(x);
    let y = from_list(y);

    let xtree = create_rtree(x.clone());
    let ytree = create_rtree(y.clone());

    let cands = xtree.intersection_candidates_with_other_tree(&ytree);

    let res_cands = cands
            .map(|(x, y)| (x.data, y.data))
            .collect::<Vec<(usize, usize)>>();
    
    // need to create a sparse representation now 
    let nx = x.len();
    let ny = y.len();
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(nx);
    
    // allocate internal vecs
    for _ in 0..nx {
        res.push(Vec::with_capacity(ny))
    }

    for (xin, yin) in res_cands.into_iter() {
        //if yin == 0 { continue; } 
        if x[xin].geom.intersects(&y[yin].geom) { res[xin].push((yin as i32) + 1) }
    }

    List::from_values(res)

}


#[extendr]
/// @export
/// @rdname predicates
fn intersects(x: Robj, y: List) -> Vec<bool> {
    let x = Geom::from(x);
    let y = from_list(y);

    y
        .into_iter()
        .map(|y| x.geom.intersects(&y.geom))
        .collect::<Vec<bool>>()
}

#[extendr]
/// @export
/// @rdname predicates
fn intersects_pairwise(x: List, y: List) -> Vec<bool> {
    let x = from_list(x);
    let y = from_list(y);

    x
        .into_iter()
        .enumerate()
        .map(|(i, xi)| xi.geom.intersects(&y[i].geom))
        .collect::<Vec<bool>>()
}


// Within ------------------------------------------------------------------
#[extendr]
/// @export
/// @rdname predicates
fn within_sparse(x: List, y: List) -> List {
    let x = from_list(x);
    let y = from_list(y);

    let xtree = create_rtree(x.clone());
    let ytree = create_rtree(y.clone());

    let cands = xtree.intersection_candidates_with_other_tree(&ytree);

    let res_cands = cands
            .map(|(x, y)| (x.data, y.data))
            .collect::<Vec<(usize, usize)>>();
    
    // need to create a sparse representation now 
    let nx = x.len();
    let ny = y.len();
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(nx);
    
    // allocate internal vecs
    for _ in 0..nx {
        res.push(Vec::with_capacity(ny))
    }

    for (xin, yin) in res_cands.into_iter() {
        //if yin == 0 { continue; } 
        if x[xin].geom.is_within(&y[yin].geom) { res[xin].push((yin as i32) + 1) }
    }

    List::from_values(res)


}


#[extendr]
/// @export
/// @rdname predicates
fn within(x: Robj, y: List) -> Vec<bool> {
    let x = Geom::from(x);
    let y = from_list(y);

    y
        .into_iter()
        .map(|y| x.geom.is_within(&y.geom))
        .collect::<Vec<bool>>()
}

#[extendr]
/// @export
/// @rdname predicates
fn within_pairwise(x: List, y: List) -> Vec<bool> {
    let x = from_list(x);
    let y = from_list(y);

    x
        .into_iter()
        .enumerate()
        .map(|(i, xi)| xi.geom.is_within(&y[i].geom))
        .collect::<Vec<bool>>()
}


extendr_module! {
    mod topology;
    fn contains; // vectorized along y
    fn contains_sparse; 
    fn contains_pairwise;
    fn intersects_sparse;
    fn intersects;
    fn intersects_pairwise;
    fn within;
    fn within_sparse;
    fn within_pairwise;
}