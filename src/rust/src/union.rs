use extendr_api::{prelude::*};
use geo::{
    BooleanOps, 
    LineString, MultiLineString, 
    MultiPoint, 
    MultiPolygon, RemoveRepeatedPoints, 

};
use geo_types::Point;
use sfconversions::vctrs::{as_rsgeo_vctr, geom_class};

use std::rc::Rc;

use sfconversions::{Geom, IntoGeom};
use crate::spatial_index::create_cached_rtree;

use rstar::primitives::{GeomWithData, CachedEnvelope};
use rstar::{ParentNode, RTreeNode};

extendr_module! {
    mod union;
    fn union_geoms;
}


/// @export
/// @rdname combine_geoms
#[extendr]
fn union_geoms(x: List) -> Robj {
    let mut geom_type = x.class().unwrap();

    //match geom_type
    let geo_type = geom_type.nth(0).unwrap();

    let res = match geo_type {
        "rs_POINT" => union_points(x),
        "rs_MULTIPOINT" => union_multipoints(x),
        "rs_POLYGON" => union_polygons(x),
        "rs_MULTIPOLYGON" => union_multipolygons(x),
        "rs_LINESTRING" => union_linestrings(x),
        "rs_MULTILINESTRING" => union_multilinestrings(x),
        _ => as_rsgeo_vctr(list!(extendr_api::NULL), geom_class("geometry")),
    };
    res
}



#[extendr]
fn union_points(x: List) -> Robj {
    let pnts = x  
        .into_iter()
        .filter(|(_, xi)| xi.is_null())
        .map(|(_, robj)| Point::from(Geom::from(robj)))
        .collect::<Vec<Point>>();

    let mpnt = MultiPoint::from(pnts);

    let res = mpnt
        .remove_repeated_points()
        .into_geom();

    as_rsgeo_vctr(list!(res), geom_class("multipoint"))
}


// duplicate points are not removed
#[extendr]
fn union_multipoints(x: List) -> Robj {
    let pnts = x  
        .into_iter()
        .filter(|(_, xi)| xi.is_null())
        .flat_map(|(_, robj)| MultiPoint::from(Geom::from(robj)).0)
        .collect::<Vec<Point>>();

    let mpnt = MultiPoint::from(pnts);

    let res = mpnt
        .remove_repeated_points()
        .into_geom();

    as_rsgeo_vctr(list!(res), geom_class("multipoint"))

}

fn union_linestrings(x: List) -> Robj {

    let lns = x  
        .into_iter()
        .filter(|(_, xi)| xi.is_null())
        .map(|(_, robj)| LineString::from(Geom::from(robj)))
    .collect::<Vec<LineString>>();

    let res = MultiLineString::new(lns)
        .remove_repeated_points()
        .into_geom();

    as_rsgeo_vctr(list!(res), geom_class("multilinestring"))
}

fn union_multilinestrings(x: List) -> Robj {

    let lns = x  
        .into_iter()
        .filter(|(_, xi)| xi.is_null())
        .flat_map(|(_, robj)| MultiLineString::from(Geom::from(robj)).0)
    .collect::<Vec<LineString>>();

    let res = MultiLineString::new(lns)
        .remove_repeated_points()
        .into_geom();

    as_rsgeo_vctr(list!(res), geom_class("multilinestring"))
}


fn union_multipolygons(x: List) -> Robj {
    // first extract the underlying multipolygons into a single
    // vector only then do we insert it into the tree.
    let x = x
        .into_iter()
        // filter out missing values
        .filter(|(_, xi)| xi.is_null())
        // extract multipolygons in sub polygons
        .flat_map(|(_, x)| {
            MultiPolygon::from(x.into_geom())
                .0
                .into_iter()
                .map(|xi| Geom::from(xi))
                .collect::<Vec<Geom>>()
        })
        // collect into a vector of geoms
        .collect::<Vec<Geom>>();

    // convert to a list cannot collect due to list collection bugh
    let x = List::from_values(x);

    union_polygons(x)
}


fn union_polygons(x: List) -> Robj {

    let shared_geo = Rc::new(x);

    let tree = create_cached_rtree(shared_geo.clone().as_list().unwrap());

    let res = tree
        .root()
        .children()
        .into_iter()
        .fold(MultiPolygon::new(vec![]), |accum, child| {
            match child {
                RTreeNode::Leaf(val) => {
                    let robj = MultiPolygon::from(shared_geo.elt(val.data).unwrap().into_geom());
                    accum.union(&robj)
                }
                RTreeNode::Parent(parent) => {
                    inner(parent, shared_geo.clone())
                }
            }
        }).into_geom();

    as_rsgeo_vctr(list!(res), geom_class("multipolygon"))
}


fn inner(papa: &ParentNode<GeomWithData<CachedEnvelope<Geom>, usize>>, shared_geo: Rc<List>) -> MultiPolygon {
    papa
        .children()
        .into_iter()
        .fold(MultiPolygon::new(vec![]), |accum, child| {
            match child {
                RTreeNode::Leaf(val) => {
                    let robj = MultiPolygon::from(shared_geo.elt(val.data).unwrap().into_geom());
                    accum.union(&robj)
                }
                RTreeNode::Parent(parent) => {
                    let value = inner(parent, shared_geo.clone());
                    accum.union(&value)
                }
            }
        })
}

// fn sort_points(x: &mut Vec<Point>) -> MultiPoint {
//     x
//         .sort_by(|a, b| {
//             a.x_y().partial_cmp(&b.x_y()).unwrap()
//         });

//     x.dedup_by(|a, b| a.eq(&b));

//     MultiPoint::new(x.to_vec())
// }