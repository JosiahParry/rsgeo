use sfconversions::{
    geometry_from_list,
    vctrs::{as_rsgeo_vctr, geom_class, verify_rsgeo},
    Geom, IntoGeom,
};

use extendr_api::prelude::*;

use crate::construction::IsReal;
use geo::{BoundingRect, ConcaveHull, ConvexHull, Extremes, MinimumRotatedRect};
use geo_types::{Geometry, Point, Polygon};

use rayon::prelude::*;

#[extendr]
/// Compute Geometric Boundaries
///
/// From a vector of geometries identify different types of boundaries.
///
/// Note that if you want a convex or concave hull over an entire vector of geometries
/// you must first union or combine them using either `combine_geoms()` or `union_geoms()`
///
/// @param x an object of class `rsgeo`
/// @param concavity a value between 0 and 1 specifying the concavity of the convex hull
///
/// @export
/// @rdname boundaries
///
/// @examples
/// lns <- geom_linestring(
///   1:20,
///   runif(20, -5, 5),
///   rep.int(1:5, 4)
/// )
/// bounding_box(lns)
/// bounding_boxes(lns)
/// minimum_rotated_rect(lns)
/// convex_hull(lns)
/// concave_hull(lns, 0.5)
/// extreme_coords(lns)
///
/// @returns
///
/// - `bounding_box()` returns a named vector of xmin, ymin, xmax, and ymax
/// - `bounding_boxes()` returns a list of bounding box numeric vectors for each geometry
/// - `bounding_rect()` returns an `rs_POLYGON` of the bounding rectangle of each geometry
/// - `convex_hull()` returns an `rs_POLYGON` of the convex hull for each geometry
/// - `concave_hull()` returns an `rs_POLYGON` of the specified concavity for each geometry
/// - `extreme_coords()` returns the extreme coordinates of each geometry as a list where each element
///  is a named vector of xmin, ymin, xmax, and ymax where each element is a `Point` geometry of the extreme value
/// - `minimum_rotated_rect()` returns the minimum rotated rectangle covering a geometry as an `rs_POLYGON`

fn bounding_box(x: List) -> Robj {
    let bbox = x
        .iter()
        .fold([f64::MAX, f64::MAX, f64::MIN, f64::MIN], |acc, (_, xi)| {
            let g = <&Geom>::from_robj(&xi);

            match g {
                Ok(geo) => {
                    let (xmin, ymin) = geo.geom.bounding_rect().unwrap().min().x_y();
                    let (xmax, ymax) = geo.geom.bounding_rect().unwrap().max().x_y();

                    [
                        acc[0].min(xmin),
                        acc[1].min(ymin),
                        acc[2].max(xmax),
                        acc[3].max(ymax),
                    ]
                }
                Err(_) => acc,
            }
        });

    // TODO what if all values are NA? We will be returning massive numbers and that wouldnt be good

    Doubles::from_values(bbox)
        .into_robj()
        .set_names(["xmin", "ymin", "xmax", "ymax"])
        .unwrap()
}

#[extendr]
/// @rdname boundaries
/// @export
fn bounding_boxes(x: List) -> List {
    let res_vec = x
        .iter()
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
        .collect::<Vec<Robj>>();

    List::from_values(res_vec)
}

#[extendr]
/// @rdname boundaries
/// @export
fn bounding_rect(x: List) -> Robj {
    let res_vec = x
        .iter()
        .map(|(_, xi)| {
            if x.is_null() {
                ().into_robj()
            } else {
                let bb = <&Geom>::from_robj(&xi).unwrap().geom.bounding_rect();

                match bb {
                    Some(b) => Geom::from(Polygon::from(b)).into_robj(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<Vec<Robj>>();
    as_rsgeo_vctr(List::from_values(res_vec), "polygon")
}

#[extendr]
/// @rdname boundaries
/// @export
fn convex_hull(x: List) -> Robj {
    let res_vec = x
        .iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                ().into_robj()
            } else {
                let xi = <&Geom>::from_robj(&xi).unwrap().geom.convex_hull();
                Geom::from(xi).into_robj()
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), "polygon")
}

#[extendr]
/// @rdname boundaries
/// @export
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

    let res_vec = x
        .iter()
        .zip(concavity.iter())
        .map(|((_, xi), ci)| {
            if xi.is_null() || !ci.is_real() {
                ().into_robj()
            } else {
                let g = <&Geom>::from_robj(&xi).unwrap();

                match &g.geom {
                    Geometry::LineString(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::MultiLineString(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::MultiPolygon(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::MultiPoint(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    Geometry::Polygon(g) => g.concave_hull(ci.inner()).into_geom().into(),
                    _ => ().into_robj(),
                }
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), "polygon")
}

#[extendr]
/// @rdname boundaries
/// @export
fn extreme_coords(x: List) -> List {
    verify_rsgeo(&x);

    let res_vec = x
        .iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                ().into_robj()
            } else {
                let extremes = <&Geom>::from_robj(&xi).unwrap().geom.extremes();
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
        .collect::<Vec<Robj>>();

    List::from_values(res_vec)
}

#[extendr]
/// @rdname boundaries
/// @export
fn minimum_rotated_rect(x: List) -> Robj {
    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`")
    }

    let geoms = geometry_from_list(x);

    let res_vec = geoms
        .into_par_iter()
        .map(|xi| match xi {
            Some(g) => g.minimum_rotated_rect(),
            None => None,
        })
        .collect::<Vec<Option<Polygon>>>();

    let res = res_vec
        .into_iter()
        .map(|xi| match xi {
            Some(p) => Geom::from(p).into_robj(),
            None => ().into_robj(),
        })
        .collect::<Vec<Robj>>();

    // let res_vec = x.iter()
    //     .map(|(_, xi)| {
    //         if xi.is_null() {
    //             ().into_robj()
    //         } else {
    //             let bb = <&Geom>::from_robj(&xi).unwrap().geom.minimum_rotated_rect();
    //             match bb {
    //                 Some(b) => b.into_geom().into_robj(),
    //                 None => NULL.into_robj(),
    //             }
    //         }
    //     })
    //     .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res), "polygon")
}

extendr_module! {
    mod boundary;
    fn bounding_boxes;
    fn bounding_rect;
    fn minimum_rotated_rect;
    fn convex_hull;
    fn concave_hull;
    fn extreme_coords;
    fn bounding_box;
}
