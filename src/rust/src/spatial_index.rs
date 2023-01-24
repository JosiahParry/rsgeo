use geo_types::{Polygon, Point, LineString};
use rstar::{RTree, RTreeObject};
use rstar::primitives::GeomWithData;

// create a tree from polygons
pub fn create_rtree_polygon(geoms: Vec<Polygon>) -> RTree<GeomWithData<Polygon, usize>> {

    let mut r_tree = RTree::new();
    for (index, geom) in geoms.into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }
    r_tree
}

pub fn create_rtree_points(geoms: Vec<Point>) -> RTree<GeomWithData<Point, usize>> {
    let mut r_tree = RTree::new();
    for (index, geom) in geoms.into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }
    r_tree
}

pub fn create_rtree_linestring(geoms: Vec<LineString>) -> RTree<GeomWithData<LineString, usize>> {
    let mut r_tree = RTree::new();
    for (index, geom) in geoms.into_iter().enumerate() {
        let geom = GeomWithData::new(geom, index);
        r_tree.insert(geom);
    }
    r_tree
}

