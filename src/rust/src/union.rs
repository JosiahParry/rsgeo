use extendr_api::prelude::*;
use geo::{
    BooleanOps, Geometry, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon, RemoveRepeatedPoints,
};

use crate::geoms::*;
use crate::types::*;
//use crate::geoms::to_pntr;

use rstar::primitives::GeomWithData;
use rstar::{ParentNode, RTree, RTreeNode};

#[extendr]
///@export
fn union_geoms(x: List) -> Robj {
    let mut geom_type = x.class().unwrap();
    let x = from_list(x);
    let x: Vec<Geometry> = x.into_iter().map(|x| x.geom).collect();

    //match geom_type
    let geo_type = geom_type.nth(0).unwrap();
    
    let res = match geo_type {
        "rs_POINT" => {
            let x = x
                .into_iter()
                .map(|x| Point::try_from(x).unwrap())
                .collect::<Vec<Point>>();
            to_pntr(Geometry::from(union_point(x)).into())
        }

        "rs_POLYGON" => {
            let x = x
                .into_iter()
                .map(|x| Polygon::try_from(x).unwrap())
                .collect::<Vec<Polygon>>();
            to_pntr(Geometry::from(union_polygon(x)).into())
        }

        "rs_MULTIPOLYGON" => {
            let x = x
                .into_iter()
                .map(|x| MultiPolygon::try_from(x).unwrap())
                .collect::<Vec<MultiPolygon>>();
            to_pntr(Geometry::from(union_multipolygon(x)).into())
        }

        "rs_MULTIPOINT" => {
            let x = x
                .into_iter()
                .map(|x| MultiPoint::try_from(x).unwrap())
                .collect::<Vec<MultiPoint>>();
            to_pntr(Geometry::from(union_multipoint(x)).into())
        }
        "rs_LINESTRING" => {
            let x = x
                .into_iter()
                .map(|x| LineString::try_from(x).unwrap())
                .collect::<Vec<LineString>>();
            to_pntr(Geometry::from(union_linestring(x)).into())
        }
        "rs_MULTILINESTRING" => {
            let x = x
                .into_iter()
                .map(|x| MultiLineString::try_from(x).unwrap())
                .collect::<Vec<MultiLineString>>();
            to_pntr(Geom::from(Geometry::from(union_multilinestring(x))))
        }
        _ => Robj::from(extendr_api::NULL)
    };

    let lst = List::from_values(vec![res]);
    crate::utils::restore_geoms(lst.into())

}

//#[extendr]
fn union_polygon(x: Vec<Polygon>) -> MultiPolygon {
    let mut r_tree = RTree::new();

    // insert into rtree with index as data
    for (index, geom) in x.into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }

    let papa = r_tree.root();

    inner(papa)
}

// #[extendr]
// fn union_polys(x: List) -> Robj {
//     let x = from_list(x);

//     let x: Vec<Polygon> = x.into_iter()
//         .map(|x| Polygon::try_from(x.geom).unwrap())
//         .collect();

//         // create the tree
//     let mut r_tree = RTree::new();

//     // insert into rtree with index as data
//     for (index, geom) in x.into_iter().enumerate() {
//         let geom = GeomWithData::new(geom, index);
//         r_tree.insert(geom);
//     }

//     let papa = r_tree.root();

//     let x = inner(papa);

//     to_pntr(Geom { geom: Geometry::from(x)})

// }

fn union_multipolygon(x: Vec<MultiPolygon>) -> MultiPolygon {
    // first extract the underlying multipolygons into a single
    // vector only then do we insert it into the tree.
    let x = x.into_iter().flat_map(|x| x.0).collect::<Vec<Polygon>>();

    let mut r_tree = RTree::new();

    // insert into rtree with index as data
    for (index, geom) in x.into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }

    let papa = r_tree.root();

    inner(papa)
}

// duplicate points are
fn union_point(x: Vec<Point>) -> MultiPoint {
    let res: MultiPoint = x.try_into().unwrap();

    // remove repeated points (similar to sf::st_union())
    res.remove_repeated_points()
}

// duplicate points are not removed
fn union_multipoint(x: Vec<MultiPoint>) -> MultiPoint {
    let point_vec = x.into_iter().flat_map(|x| x.0).collect::<Vec<Point>>();

    MultiPoint::new(point_vec)
}

fn union_linestring(x: Vec<LineString>) -> MultiLineString {
    let res = MultiLineString::new(x);
    res.remove_repeated_points()
}

fn union_multilinestring(x: Vec<MultiLineString>) -> MultiLineString {
    let line_vecs = x.into_iter().flat_map(|x| x.0).collect::<Vec<LineString>>();

    MultiLineString::new(line_vecs).remove_repeated_points()
}

extendr_module! {
    mod union;
    fn union_geoms;
}

fn inner(papa: &ParentNode<GeomWithData<Polygon, usize>>) -> MultiPolygon {
    papa.children()
        .iter()
        .fold(MultiPolygon::new(vec![]), |accum, child| match child {
            RTreeNode::Leaf(value) => {
                let v = MultiPolygon::try_from(value.geom().to_owned()).unwrap();
                accum.union(&v)
            }
            RTreeNode::Parent(parent) => {
                let value = inner(parent);
                accum.union(&value)
            }
        })
}
