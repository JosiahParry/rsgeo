use extendr_api::prelude::*;

use geo::{BoundingRect};
use rstar::{RTree, RTreeObject, AABB};
use rstar::primitives::GeomWithData;
use crate::geoms::from_list;
use crate::types::Geom;

// implement RTreeObject for Geom struct (wrapper for geo-types::Geometry)
impl RTreeObject for Geom {
    type Envelope = AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope {
        let bbox = self.geom.bounding_rect().unwrap();
        let ll = bbox.min(); //lower left x coord
        let ur = bbox.max(); // upper right y
        AABB::from_corners(ll.into(), ur.into())
        
    }
}

// related:
// https://github.com/georust/rstar/issues/108
// https://github.com/georust/geo/issues/982
pub fn create_rtree(geoms: Vec<Geom>) -> RTree<GeomWithData<Geom, usize>> {
    let mut r_tree = RTree::new();
    for (index, geom) in geoms.into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }
    r_tree
}


#[extendr]
/// Create an rstar RTree from a vector of geometry
/// @param x a vector of rust geometry 
/// @export 
fn rstar_rtree(x: List) -> Robj {
    let geoms = from_list(x);

    let rtree = create_rtree(geoms);
    let rtree_size = rtree.size();
    
    ExternalPtr::new(rtree)
        .as_robj()
        .set_attrib("class", "rstar_rtree")
        .unwrap()
        .set_attrib("size", rtree_size)
        .unwrap()
}


#[extendr]
fn intersection_candidates(x: List, y: List) -> List {
    
    let x = from_list(x);
    let n = x.len(); 
    let y = from_list(y);

    let x_rtree = create_rtree(x);
    let y_rtree = create_rtree(y);

    let cands = x_rtree.intersection_candidates_with_other_tree(&y_rtree);
    // this is a vector of tuples (lhs index, rhs index)
    let res_cands = cands
        .map(|(x, y)| (x.data, y.data))
        .collect::<Vec<(usize, usize)>>();

    // need to create a sparse representation now 
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(n);
    
    // allocate internal vecs
    for _ in 0..n {
        res.push(Vec::with_capacity(n))
    }

    for (xin, yin) in res_cands.into_iter() {
        //if yin == 0 { continue; }
        res[xin].push(yin as i32)

    }

    List::from_values(res)

}



// helper to craft the AABB this is an alternative to a
// from trait becuase i dont know the types here
fn to_aabb(x: &Geom) -> AABB<[f64;2]> {
    let rct = x.geom.bounding_rect().unwrap();
    let ll = rct.min();
    let ur = rct.max();
    AABB::from_corners(ll.into(), ur.into())
}

#[extendr]
fn print_aabb(x: Robj) {
    let x = Geom::from(x);
    let aabb = to_aabb(&x);
    rprintln!("{:?}", aabb);
}

#[extendr]
fn locate_in_envelope(rtree: Robj, geom: Robj) -> Integers {
    let rtree: ExternalPtr<RTree<GeomWithData<Geom, usize>>> = rtree.try_into().unwrap();
    let env = to_aabb(&geom.into());

    let res = rtree.locate_in_envelope(&env);

    res
        .map(|x| Rint::from(x.data as i32))
        .collect::<Integers>()
}

// fn locate_nearest_nbs(rtree: Robj, geom: Robj) -> Integers {
//     let rtree: ExternalPtr<RTree<GeomWithData<Geom, usize>>> = rtree.try_into().unwrap();
//     let env = to_aabb(&geom.into());

//     let tree = *rtree;
//     tree.near

// }

extendr_module!{
    mod spatial_index;
    fn rstar_rtree;
    fn print_aabb;
    fn locate_in_envelope;
    fn intersection_candidates;
    //fn queen_contiguity;
}


// This is how i would do queen contiguity but intersects is just so friggin slow
// #[extendr]
// fn queen_contiguity(x: List) -> List {
//     let x = from_list(x);
//     let geoms = x.clone(); // clone for the R tree
//     let n = geoms.len();

//     let mut r_tree = RTree::new();
//     for (index, geom) in geoms.into_iter().enumerate() {
//         let geom = GeomWithData::new(geom, index);
//         r_tree.insert(geom);
//     }

//     let cands = r_tree.intersection_candidates_with_other_tree(&r_tree);
//     // this is a vector of tuples (lhs index, rhs index)
//     let res_cands = cands
//         .map(|(x, y)| (x.data, y.data))
//         .collect::<Vec<(usize, usize)>>();

//     // need to create a sparse representation now 
//     let mut res: Vec<Vec<i32>> = Vec::with_capacity(n);
    
//     // allocate internal vecs
//     for _ in 0..n {
//         res.push(Vec::with_capacity(n))
//     }

//     for (xin, yin) in res_cands.into_iter() {
//         //if yin == 0 { continue; }
//         if x[xin].geom.intersects(&x[yin].geom) { res[xin].push((yin as i32) + 1) }
//     }

//     List::from_values(res)


// }