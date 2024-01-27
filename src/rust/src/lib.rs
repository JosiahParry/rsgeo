// module imports
mod area;
mod boundary;
mod casting;
mod coord_utils;
mod densify;
mod distance;
mod length;
mod query;
mod segmentize;
mod similarity;
mod simplification;
// mod io;
mod construction;
mod coords;
mod spatial_index;
mod topology;
mod union;

use extendr_api::prelude::*;
pub use sfconversions::{fromsf::sfc_to_rsgeo, vctrs::*, Geom};

mod utils;
// MISC algos -------

mod rnetmerge;

use crate::construction::IsReal;

use geo::{Centroid, HaversineDestination};
use geo_types::Point;

/// Extract Centroids
///
/// Given a vector of geometries, extract their centroids.
///
/// @param x an object of class `rsgeo`
///
/// @export
/// @examples
/// lns <- geom_linestring(1:100, runif(100, -10, 10), rep.int(1:5, 20))
/// centroids(lns)
/// @returns an object of class `rs_POINT`
#[extendr]
fn centroids(x: List) -> Robj {
    verify_rsgeo(&x);
    let centroids = x
        .iter()
        .map(|(_, x)| {
            if x.is_null() {
                x
            } else {
                let geo = <&Geom>::from_robj(&x).unwrap().geom.centroid();

                match geo {
                    Some(cnt) => Geom::from(cnt).into_robj(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(centroids), "point")
}

#[extendr]
fn from_sfc(x: List) -> Robj {
    sfc_to_rsgeo(x)
}

#[extendr]
fn to_sfc(x: List) -> List {
    let res = x
        .into_iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                NULL.into_robj()
            } else {
                sfconversions::tosf::to_sfg(Geom::from(xi))
            }
        })
        .collect::<Vec<Robj>>();

    List::from_values(res)
}

#[extendr]
/// Identify a destination point
///
/// Given a vector of point geometries, bearings, and distances,
/// identify a destination location.
///
/// @param x an object of class `rs_POINT`
/// @param bearing a numeric vector specifying the degree of the direction where 0 is north
/// @param distance a numeric vector specifying the distance to travel in the direction specified by `bearing` in meters
/// @returns an object of class `rs_POINT`
/// @examples
/// # create 10 points at the origin
/// pnts <- geom_point(rep(0, 10), rep(0, 10))
///
/// # set seed for reproducibiliy
/// set.seed(1)
///
/// # generate random bearings
/// bearings <- runif(10, 0, 360)
///
/// # generate random distances
/// distances <- runif(10, 10000, 100000)
///
/// # find the destinations
/// dests <- haversine_destination(pnts, bearings, distances)
///
/// # plot points
/// if (rlang::is_installed(c("sf", "wk"))) {
///   plot(pnts, pch = 3)
///   plot(dests, add = TRUE, pch = 17)
/// }
/// @export
fn haversine_destination(x: List, bearing: Doubles, distance: Doubles) -> Robj {
    if !x.inherits("rs_POINT") {
        panic!("`x` must be of class `rs_POINT`")
    }

    let n = x.len();
    let n_b = bearing.len();
    let n_d = distance.len();

    let bearing = if n_b == 1 {
        Doubles::from_values(vec![bearing[0].inner(); n])
    } else {
        bearing
    };

    let distance = if n_d == 1 {
        Doubles::from_values(vec![distance[0].inner(); n])
    } else {
        distance
    };

    let mut res: Vec<Robj> = Vec::with_capacity(n);

    for i in 0..n {
        let geo = x.elt(i);

        let b = bearing[i];
        let d = distance[i];

        let xi = if !b.is_real() || !d.is_real() {
            NULL.into_robj()
        } else {
            match geo {
                Ok(g) => {
                    let g = Point::from(Geom::try_from(g).unwrap());
                    let p = g.haversine_destination(b.inner(), d.inner());
                    Geom::from(p).into_robj()
                }
                Err(_) => NULL.into_robj(),
            }
        };

        res.push(xi);
    }

    as_rsgeo_vctr(List::from_values(res), "point")
}

use geo::HaversineIntermediate;

#[extendr]
/// Identifies a point between two points
///
/// Identifies the location between two points on a great circle
/// along a specified fraction of the distance.
///
/// @param x an `rs_POINT` vector
/// @param y an `rs_POINT` vector
///
/// @param distance a numeric vector of either length 1 or the same length as x and y
///
/// @returns an object of class `rs_POINT`
///
/// @examples
/// x <- geom_point(1:10, rep(5, 10))
/// y <- geom_point(1:10, rep(0, 10))
/// res <- haversine_intermediate(x, y, 0.5)
/// if (rlang::is_installed(c("wk", "sf"))) {
///   plot(
///     c(x, y, res),
///     col = sort(rep.int(c("red", "blue", "purple"), 10)),
///     pch = 16
///   )
/// }
/// @export
fn haversine_intermediate(x: List, y: List, distance: Doubles) -> Robj {
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must be of class `rs_POINT`")
    }

    let n_x = x.len();
    let n_y = y.len();
    let n = n_x.max(n_y);

    if n_x != n_y && (n_x == 1 || n_y == 1) {
        panic!("`x` and `y` must be the same length or length 1");
    }

    let mut x_cycle = x.iter().cycle();
    let mut y_cycle = y.iter().cycle();

    let n_d = distance.len();

    if ((n_d != n_x) && (n_d != n_y)) && (n_d != 1) {
        panic!("`distance` must be the same length as `x`, `y`, or length 1");
    }

    let distance = if n_d == 1 {
        Doubles::from_values(vec![distance[0].inner(); n])
    } else {
        distance
    };

    let mut res: Vec<Robj> = Vec::with_capacity(n);

    for i in 0..n {
        // early return for missing distance
        let d = distance[i];
        if d.is_na() || d.is_infinite() || d.is_nan() {
            return NULL.into_robj();
        }

        // cycle through the points
        let (_, xi) = x_cycle.next().unwrap();
        let (_, yi) = y_cycle.next().unwrap();

        // early return if either xi or yi are null
        if xi.is_null() || yi.is_null() {
            return NULL.into_robj();
        }

        let xi = Point::from(Geom::try_from(xi).unwrap());
        let yi = Point::from(Geom::try_from(yi).unwrap());
        let p = xi.haversine_intermediate(&yi, d.inner());

        res.push(Geom::from(p).into());
    }

    List::from_values(res)
        .set_attrib("class", geom_class("point"))
        .unwrap()
}

// /// Chaikin Smoothing
// ///@export
// #[extendr]
// fn chaikin_smoothing(x: Robj, niter: f64) -> Robj {
//     let x: Geom = x.try_into().unwrap();
//     let x = x.geom;

//     let res = match x {
//         Geometry::LineString(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
//         Geometry::MultiLineString(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
//         Geometry::MultiPolygon(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
//         Geometry::Polygon(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
//         // these types will return themselves
//         Geometry::Point(x) => Geom::from(x),
//         Geometry::MultiPoint(x) => Geom::from(x),
//         Geometry::Rect(x) => Geom::from(x),
//         Geometry::Line(x) => Geom::from(x),

//         _ => Geom::from(line_string![]),
//     };

//     to_pntr(res)
// }

// --------------------------------------------------

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rsgeo;
    fn from_sfc;
    fn to_sfc;
    fn centroids;
    fn haversine_destination;
    fn haversine_intermediate;
    use area;
    use boundary;
    use coords;
    use coord_utils;
    use construction;
    use densify;
    use distance;
    use length;
    use query;
    use segmentize;
    use simplification;
    use similarity;
    use topology;
    use union;
    use utils;
    use casting;
    use rnetmerge;
}
