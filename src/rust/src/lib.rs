// module imports
mod area;
mod boundary;
mod casting;
mod length;
mod query;
mod similarity;
mod simplification;
// mod conversion;
// mod distance;
// mod io;
mod spatial_index;
mod topology;
// mod union;
mod construction;

use extendr_api::prelude::*;
pub use sfconversions::{fromsf::sfc_to_rsgeo, vctrs::*, Geom};

// MISC algos -------

use geo::Centroid;
/// Find centroid
/// @param x an object of class `point`
///@export
#[extendr]
fn centroids(x: List) -> Robj {
    let centroids = x
        .iter()
        .map(|(_, x)| {
            if x.is_null() {
                x
            } else {
                let geo = Geom::try_from(x).unwrap().geom.centroid();

                match geo {
                    Some(cnt) => Geom::from(cnt).into_robj(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<List>();

    as_rsgeo_vctr(centroids, geom_class("point"))
}

#[extendr]
fn from_sfc(x: List) -> Robj {
    sfc_to_rsgeo(x)
}

#[extendr]
fn to_sfc(x: List) -> List {
    // crate::boundary::boun
    x.into_iter()
        .map(|(_, xi)| sfconversions::tosf::to_sfg(Geom::from(xi)))
        .collect::<List>()
}

// /// Haversine Destination
// ///@export
// #[extendr]
// fn haversine_destination(x: Robj, bearing: f64, distance: f64) -> Robj {
//     let x: Geom = x.try_into().unwrap();
//     let x: Point = x.try_into().unwrap();

//     let point = x.haversine_destination(bearing, distance);

//     let res = Geom::from(point);

//     r![ExternalPtr::new(res)]
//         .set_attrib("class", "point")
//         .unwrap()
// }

use geo::HaversineDestination;
use geo_types::Point;

#[extendr]
fn haversine_destination(x: List, bearing: Doubles, distance: Doubles) -> Robj {
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

    for i in 0..(n - 1) {
        let geo = x.elt(i);

        let b = bearing[i];
        let d = distance[i];

        let xi = if b.is_na()
            || b.is_infinite()
            || b.is_nan()
            || d.is_na()
            || d.is_infinite()
            || d.is_nan()
        {
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

    List::from_values(res)
        .set_attrib("class", geom_class("point"))
        .unwrap()
}

use geo::HaversineIntermediate;

#[extendr]
/// @param x an `rs_POINT` vector
/// @param y an `rs_POINT` vector
/// @param distance a numeric vector of either length 1 or the same length as x and y.
fn haversine_intermediate(x: List, y: List, distance: Doubles) -> Robj {
    let n_x = x.len();
    let n_y = y.len();
    let n = n_x.max(n_y);

    if n_x != n_y && (n_x == 1 || n_y == 1) {
        panic!("`x` and `y` must be the same length or length 1");
    }

    let mut x_cycle = x.iter().cycle();
    let mut y_cycle = y.iter().cycle();

    let n_d = distance.len();

    //rprintln!("nd {} nx {} ny {}", n_d, n_x, n_y);
    //rprintln!("cond1 {} cond 2 {}", ((n_d != n_x) && (n_d != n_y)), (n_d != 1));
    if ((n_d != n_x) && (n_d != n_y)) && (n_d != 1) {
        panic!("`distance` must be the same length as `x`, `y`, or length 1");
    }

    let distance = if n_d == 1 {
        Doubles::from_values(vec![distance[0].inner(); n])
    } else {
        distance
    };

    let mut res: Vec<Robj> = Vec::with_capacity(n);

    for i in 0..(n - 1) {
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
    use length;
    use query;
    use boundary;
    use simplification;
    use similarity;
    use topology;
    use construction;
}
