use sfconversions::{
    vctrs::{geom_class, verify_rsgeo},
    Geom, IntoGeom,
};

use extendr_api::prelude::*;

use geo::{BoundingRect, ConcaveHull, ConvexHull, Extremes, MinimumRotatedRect};
use geo_types::{Geometry, Point, Polygon};

#[extendr]
fn bounding_boxes(x: List) -> List {
    x.iter()
        .map(|(_, xi)| {
            if x.is_null() {
                let bb = [Rfloat::na(); 4];
                Doubles::from_values(bb)
                    .into_robj()
                    .set_names(["xmin", "ymin", "xmax", "ymax"])
                    .unwrap()
            } else {
                let bb = Geom::try_from(xi).unwrap().geom.bounding_rect();

                match bb {
                    Some(b) => {
                        let (xmin, ymin) = b.min().x_y();
                        let (xmax, ymax) = b.max().x_y();
                        Doubles::from_values([xmin, ymin, xmax, ymax])
                            .into_robj()
                            .set_names(["xmin", "ymin", "xmax", "ymax"])
                            .unwrap()
                    }
                    None => {
                        let bb = [Rfloat::na(); 4];
                        Doubles::from_values(bb)
                            .into_robj()
                            .set_names(["xmin", "ymin", "xmax", "ymax"])
                            .unwrap()
                    }
                }
            }
        })
        .collect::<List>()
}

#[extendr]
fn bounding_rect(x: List) -> Robj {
    x.iter()
        .map(|(_, xi)| {
            if x.is_null() {
                ().into_robj()
            } else {
                let bb = Geom::try_from(xi).unwrap().geom.bounding_rect();

                match bb {
                    Some(b) => Geom::from(Polygon::from(b)).into_robj(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<List>()
        .set_class(geom_class("polygon"))
        .unwrap()
}

#[extendr]
fn convex_hull(x: List) -> Robj {
    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                ().into_robj()
            } else {
                let xi = Geom::try_from(xi).unwrap().geom.convex_hull();

                Geom::from(xi).into_robj()
            }
        })
        .collect::<List>()
        .set_class(geom_class("polygon"))
        .unwrap()
}

#[extendr]
fn concave_hull(x: List, concavity: Doubles) -> Robj {
    let n = x.len();
    let n_c = concavity.len();
    let cls = x.class().unwrap().next().unwrap();

    if x.inherits("rs_POINT") {
        return x.into();
    } else if !cls.starts_with("rs_") {
        panic!("`x` must be a Rust geometry type")
    }

    let concavity = if n_c == 1 {
        Doubles::from_values(vec![concavity[0]; n])
    } else if n_c != n {
        panic!("`concavity` must be length 1 or the same length as `x`")
    } else {
        concavity
    };

    x.iter()
        .zip(concavity.iter())
        .map(|((_, xi), ci)| {
            if xi.is_null() || ci.is_na() || ci.is_nan() || ci.is_infinite() {
                ().into_robj()
            } else {
                let g = Geom::try_from(xi).unwrap().geom;

                match g {
                    Geometry::LineString(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::MultiLineString(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::MultiPolygon(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::MultiPoint(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::Polygon(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    _ => ().into_robj(),
                }
            }
        })
        .collect::<List>()
        .set_class(geom_class("polygon"))
        .unwrap()
}

#[extendr]
fn extreme_coords(x: List) -> List {
    verify_rsgeo(&x);

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                ().into_robj()
            } else {
                let extremes = Geom::try_from(xi).unwrap().geom.extremes();
                match extremes {
                    Some(ext) => {
                        let crds = [
                            Point::from(ext.x_min.coord).into_geom(),
                            Point::from(ext.y_min.coord).into_geom(),
                            Point::from(ext.x_max.coord).into_geom(),
                            Point::from(ext.y_max.coord).into_geom(),
                        ];

                        List::from_values(crds)
                            .set_class(geom_class("point"))
                            .unwrap()
                            .set_names(["xmin", "ymin", "xmax", "ymax"])
                            .unwrap()
                    }
                    _ => ().into_robj(),
                }
            }
        })
        .collect()
}

#[extendr]
fn minimum_bounding_rect(x: List) -> Robj {
    x.iter()
        .map(|(_, xi)| {
            if x.is_null() {
                ().into_robj()
            } else {
                let bb = Geom::try_from(xi).unwrap().geom.minimum_rotated_rect();

                match bb {
                    Some(b) => Geom::from(Polygon::from(b)).into_robj(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<List>()
        .set_class(geom_class("polygon"))
        .unwrap()
}

// /// Find extremes
// /// @param x a geometry
// /// @export
// #[extendr]
// fn extreme_coords(x: Robj) -> Robj {
//     let res = Geom::from(x).geom.extremes().unwrap();

//     List::from_names_and_values(
//         ["x_min", "x_max", "y_min", "y_max"],
//         [
//             to_pntr(Geom::from(Point::from(res.x_min.coord))),
//             to_pntr(Geom::from(Point::from(res.x_max.coord))),
//             to_pntr(Geom::from(Point::from(res.y_min.coord))),
//             to_pntr(Geom::from(Point::from(res.y_max.coord))),
//         ],
//     )
//     .unwrap()
//     .set_attrib("class", crate::utils::geom_class("point"))
//     .unwrap()
// }

// #[extendr]
// /// Compute Geometric Boundaries
// ///
// /// @export
// /// @rdname boundaries
// /// @param x a rust geometry either a scalar or a vector for functions ending in `s`. See "Details" for more.
// ///
// /// @details
// ///
// /// - `bounding_box()` returns a named list of x and y maximums and minimums
// /// - `bounding_rectangle()` returns a polygon of the bounding rectangle
// /// - `convex_hull()` returns a polygon of the convex hull
// /// - `concave_hull()` returns a polygon of the specified concavity
// ///
// /// Each function, with the exception of `bounding_box()` has a plural version ending
// /// with an `s` which is vectorized over `x`.

extendr_module! {
    mod boundary;
    fn bounding_boxes;
    fn bounding_rect;
    fn convex_hull;
    fn concave_hull;
    fn extreme_coords;
    // fn bounding_box_;
    // fn bounding_rectangle;
    // fn bounding_rectangles;
    // fn concave_hull;
    // fn concave_hulls;

    // fn convex_hulls;
    // fn extreme_coords;
}
