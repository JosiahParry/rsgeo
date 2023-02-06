use crate::types::Geom;
use crate::geoms::to_pntr;
use extendr_api::prelude::*;
use extendr_api::Robj;

//use crate::geoms::from_list;
use geo_types::{Geometry, Polygon, Point};
use geo::{BoundingRect, ConcaveHull, polygon, ConvexHull, Extremes};




#[extendr]
fn bounding_rectangle(x: Robj) -> List {
    let x: Geom = x.try_into().unwrap();

    let rect = x.geom.bounding_rect().unwrap();

    let max = rect.max();
    let min = rect.min();

    List::from_names_and_values(
        ["x_min", "x_max", "y_min", "y_max"],
        [min.x,  max.x, min.y, max.y]
    ).unwrap()
}


#[extendr] 
fn concave_hull(x: Robj, concavity: f64) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x = x.geom;
    let hull = match x {
        Geometry::LineString(x) => x.concave_hull(concavity),
        Geometry::MultiLineString(x) => x.concave_hull(concavity),
        Geometry::MultiPoint(x) => x.concave_hull(concavity),
        Geometry::Polygon(x) => x.concave_hull(concavity),
        _ => polygon! {}
    };

    let res: Geom = hull.into();

    r![ExternalPtr::new(res)].set_attrib("class", "polygon").unwrap()

}

#[extendr]
fn convex_hull(x: Robj) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x: Polygon = x.geom.try_into().unwrap();

    let hull = x.convex_hull();

    let res: Geom = hull.into();

    r![ExternalPtr::new(res)].set_attrib("class", "polygon").unwrap()
}


/// Find extremes
/// @param x a geometry
/// @export
#[extendr]
fn extreme_coords(x: Robj) -> Robj {
    let res = Geom::from(x).geom.extremes().unwrap();

    
    List::from_names_and_values(
        ["x_min", "x_max", "y_min", "y_max"],
        [
            to_pntr(Geom::from(Point::from(res.x_min.coord))), 
            to_pntr(Geom::from(Point::from(res.x_max.coord))), 
            to_pntr(Geom::from(Point::from(res.y_min.coord))), 
            to_pntr(Geom::from(Point::from(res.y_max.coord)))
            ]
    ).unwrap()
    .set_attrib("class", "rs_POINT")
    .unwrap()
}


extendr_module! {
    mod boundary;
    fn bounding_rectangle;
    fn concave_hull;
    fn convex_hull;
    fn extreme_coords;
}