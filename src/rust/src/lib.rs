// module imports
mod utils;
use utils::get_utils_metadata;

mod geoms;
use geoms::{get_geoms_metadata, to_pntr};

mod area;
use area::get_area_metadata;

mod length;
use length::get_length_metadata;

mod query;
use query::get_query_metadata;

mod distance;
use distance::get_distance_metadata;

mod boundary;
use boundary::get_boundary_metadata;

mod union;
use union::get_union_metadata;

mod conversion;
use conversion::get_conversion_metadata;

mod simplification;
use simplification::get_simplification_metadata;

mod io;
pub mod types;
use io::get_io_metadata;

mod spatial_index;
use spatial_index::get_spatial_index_metadata;

mod topology;
use topology::get_topology_metadata;

mod casting;
use casting::get_casting_metadata;

use crate::types::Geom;
use extendr_api::prelude::*;
use extendr_api::wrapper::{ExternalPtr, RMatrix};
use geo::{coord, Centroid, ChaikinSmoothing, HaversineDestination, HaversineIntermediate};
use geo_types::line_string;
use ndarray::{Array2, ShapeBuilder};

use geo::geometry::{Coord, Geometry, Point};

// This is so much slower than the 1 - 1 and so much slower than geos
// Tried without cloning. same speed. It must be the claiming ownership?
// or the deparsing. idk intersect_poly_poly is SOOO much faster than c
// but the loop is slower idk
// man idfk
// #[extendr]
// fn intersect_poly_polys(lhs: Robj, rhs: List) -> Logicals {
//     let n = rhs.len();
//     let mut res = Logicals::new(n);
//     let xpoly: ExternalPtr<Polygon> = lhs.try_into().unwrap();

//     for i in 0..n {
//         let ypoly: ExternalPtr<Polygon> = rhs[i].to_owned().try_into().unwrap();
//         res.set_elt(i, Rbool::from(xpoly.intersects(&*ypoly)));
//     }
//     res

// }

// Intersect wrapper

// mod intersects;
// use crate::intersects::poly_geoms;

// mod utils;
// use crate::utils::as_vec_geoms;
// ///@export
// #[extendr (use_try_from = true)]
// fn poly_intersect(x: ExternalPtr<Polygon>, y: List) -> Integers {
//     let geoms = as_vec_geoms(y);

//     let res = poly_geoms(*x, geoms);

//     let res: Integers = res.into_iter()
//         .map(|index| Rint::from(index as i32))
//         .collect();

//     res
// }

// Helpers -----------------------------------------------------------------

// internal function to cast an R matrix to ndarray (2 dimensions)
fn mat_to_rs(x: RMatrix<f64>) -> Array2<f64> {
    let nrow = x.nrows();
    let ncol = x.ncols();
    let mat_dat = x.data().to_owned();
    Array2::from_shape_vec((nrow, ncol).f(), mat_dat).unwrap()
}

// First, I need to take a matrix and convert into coordinates
fn matrix_to_coords(x: RMatrix<f64>) -> Vec<Coord> {
    let nrow = x.nrows();
    let ncol = x.ncols();

    if ncol != 2 {
        panic!("Matrix should have only 2 columns for x and y coordinates, respectively.")
    }

    //let n = nrow.clone();
    let mut coords: Vec<Coord> = Vec::with_capacity(nrow);

    for i in 0..nrow {
        let crd = coord! {x: x[[i, 0]], y: x[[i, 1]]};
        coords.push(crd);
    }
    coords
}

// MISC algos -------

/// Find centroid
/// @param x an object of class `point`
///@export
#[extendr]
fn centroid(x: Robj) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let res = x.geom.centroid().unwrap();

    let res: Geom = res.into();

   to_pntr(res)
}

/// @rdname centroid
/// @export
#[extendr]
fn centroids(x: List) -> Robj {
    x.into_iter()
        .map(|(_, robj)| centroid(robj))
        .collect::<List>()
        .set_attrib("class", crate::utils::geom_class("point"))
        .unwrap()
}

/// Haversine Destination
///@export
#[extendr]
fn haversine_destination(x: Robj, bearing: f64, distance: f64) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x: Point = x.try_into().unwrap();

    let point = x.haversine_destination(bearing, distance);

    let res = Geom::from(point);

    r![ExternalPtr::new(res)]
        .set_attrib("class", "point")
        .unwrap()
}

/// Haversine Intermediate
///@export
#[extendr]
fn haversine_intermediate(x: Robj, y: Robj, distance: f64) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x: Point = x.try_into().unwrap();

    let y: Geom = y.try_into().unwrap();
    let y: Point = y.try_into().unwrap();

    let point = x.haversine_intermediate(&y, distance);
    let res = Geom::from(point);

    to_pntr(res)
}

/// Chaikin Smoothing
///@export
#[extendr]
fn chaikin_smoothing(x: Robj, niter: f64) -> Robj {
    let x: Geom = x.try_into().unwrap();
    let x = x.geom;

    let res = match x {
        Geometry::LineString(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
        Geometry::MultiLineString(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
        Geometry::MultiPolygon(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
        Geometry::Polygon(x) => Geom::from(x.chaikin_smoothing(niter as usize)),
        // these types will return themselves
        Geometry::Point(x) => Geom::from(x),
        Geometry::MultiPoint(x) => Geom::from(x),
        Geometry::Rect(x) => Geom::from(x),
        Geometry::Line(x) => Geom::from(x),

        _ => Geom::from(line_string![]),
    };

    to_pntr(res)
}


// ---------------------------------------------------------------------------------

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rsgeo;
    //fn linestring_to_points;
    fn centroid;
    fn centroids;
    fn haversine_destination;
    fn haversine_intermediate;
    fn chaikin_smoothing;
    use utils; 
    use area;
    use geoms;
    use length;
    use query;
    use distance;
    use boundary;
    use union;
    use conversion;
    use simplification;
    use io;
    use spatial_index;
    use topology;
    use casting;
}
