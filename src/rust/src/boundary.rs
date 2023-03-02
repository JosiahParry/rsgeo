use crate::geoms::to_pntr;
use crate::types::Geom;
use crate::utils::geom_class;
use extendr_api::prelude::*;
use extendr_api::Robj;

//use crate::geoms::from_list;
use geo::{polygon, BoundingRect, ConcaveHull, ConvexHull, Extremes};
use geo_types::{Geometry, Point, Polygon};

#[extendr]
fn bounding_box_(x: Robj) -> List {

    let x: Geom = x.try_into().unwrap();

    let rect = x.geom.bounding_rect().unwrap();

    let max = rect.max();
    let min = rect.min();

    List::from_names_and_values(
        ["xmin", "ymin", "xmax", "ymax"],
        [min.x, min.y, max.x, max.y],
    )
    .unwrap()
}


#[extendr]
/// Compute Geometric Boundaries 
/// 
/// @export
/// @rdname boundaries
/// @param x a rust geometry either a scalar or a vector for functions ending in `s`. See "Details" for more. 
///
/// @details
/// 
/// - `bounding_box()` returns a named list of x and y maximums and minimums
/// - `bounding_rectangle()` returns a polygon of the bounding rectangle
/// - `convex_hull()` returns a polygon of the convex hull
/// - `concave_hull()` returns a polygon of the specified concavity
/// 
/// Each function, with the exception of `bounding_box()` has a plural version ending 
/// with an `s` which is vectorized over `x`. 
fn bounding_rectangle(x: Robj) -> Robj {
    let x: Geom = x.try_into().unwrap();

    let rect = x.geom.bounding_rect().unwrap();
    to_pntr(Geom::from(Polygon::from(rect)))

}

#[extendr]
/// @export
/// @rdname boundaries
fn bounding_rectangles(x: List) -> Robj {
    let res = x
        .into_iter()
        .map(|(_, x)| bounding_rectangle(x))
        .collect::<List>();

    res
        .set_attrib("class", geom_class("polygon"))
        .unwrap() 
}


#[extendr]
/// @export
/// @rdname boundaries
fn concave_hull(x: Robj, concavity: f64) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x = x.geom;
    let hull = match x {
        Geometry::LineString(x) => x.concave_hull(concavity),
        Geometry::MultiLineString(x) => x.concave_hull(concavity),
        Geometry::MultiPoint(x) => x.concave_hull(concavity),
        Geometry::Polygon(x) => x.concave_hull(concavity),
        _ => polygon! {},
    };

    let res: Geom = hull.into();

    to_pntr(res)
}

#[extendr]
/// @export
/// @rdname boundaries
fn concave_hulls(x: List, concavity: f64) -> Robj {
    let res = x
    .into_iter()
    .map(|(_, x)| concave_hull(x, concavity))
    .collect::<List>();

    res
        .set_attrib("class", geom_class("polygon"))
        .unwrap() 
}

#[extendr]
/// @export
/// @rdname boundaries
fn convex_hull(x: Robj) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x: Polygon = x.geom.try_into().unwrap();

    let hull = x.convex_hull();

    let res: Geom = hull.into();

    to_pntr(res)
}

#[extendr]
/// @export
/// @rdname boundaries
fn convex_hulls(x: List) -> Robj {
    let res = x
        .into_iter()
        .map(|(_, x)| convex_hull(x))
        .collect::<List>();

    res
        .set_attrib("class", geom_class("polygon"))
        .unwrap() 

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
            to_pntr(Geom::from(Point::from(res.y_max.coord))),
        ],
    )
    .unwrap()
    .set_attrib("class", crate::utils::geom_class("point"))
    .unwrap()
}

extendr_module! {
    mod boundary;
    fn bounding_box_;
    fn bounding_rectangle;
    fn bounding_rectangles;
    fn concave_hull;
    fn concave_hulls;
    fn convex_hull;
    fn convex_hulls;
    fn extreme_coords;
}
