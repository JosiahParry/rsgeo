use extendr_api::prelude::*;
use sfconversions::geometry_from_list;
use sfconversions::vctrs::as_rsgeo_vctr;
use sfconversions::Geom;

use geo::{
    Closest, ClosestPoint, GeodesicBearing, HaversineBearing, HaversineClosestPoint, IsConvex,
    LineInterpolatePoint, LineLocatePoint,
    LineStringSegmentize
};

use crate::construction::IsReal;
use geo_types::{LineString, Point, MultiLineString};

use rayon::prelude::*;

#[extendr]
/// Calculate Bearing
///
/// Calculates the bearing between two point geometries. 
/// 
/// @param x an object of class `rs_POINT`
/// @param y an object of class `rs_POINT`
/// 
/// @returns
/// A vector of doubles of the calculated bearing for between x and y
///
/// @export
/// @rdname bearing
/// @examples
/// x <- geom_point(runif(10, 0, 90), rnorm(10, 1, 90))
/// y <- geom_point(runif(10, 0, 90), rnorm(10, 1, 90))
/// bearing_geodesic(x, y)
/// bearing_haversine(x, y)
fn bearing_haversine(x: List, y: List) -> Doubles {
    
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must be point geometries of class `rs_POINT`");
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let p1: Point = Geom::try_from(xi).unwrap().geom.try_into().unwrap();
                let p2: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                p1.haversine_bearing(p2).into()
            }
        })
        .collect::<Doubles>()
}

#[extendr]
/// @export
/// @rdname bearing
fn bearing_geodesic(x: List, y: List) -> Doubles {
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must be point geometries of class `rs_POINT`");
    }
    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let p1: Point = Geom::try_from(xi).unwrap().geom.try_into().unwrap();
                let p2: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                p1.geodesic_bearing(p2).into()

            }
        })
        .collect::<Doubles>()
}


#[extendr]
/// Find Closest Point
/// 
/// For a given geometry, find the closest point on that geometry 
/// to a point. The closest point may be an intersection, a single point,
/// or unable to be determined. 
/// 
/// @param x an object of class `rsgeo`
/// @param y an object of class `rs_POINT`
/// @export
/// @examples
/// x <- geom_linestring(1:100, runif(100, 0, 90), rep.int(1:10, 10))
/// y <- geom_point(runif(10, 0, 90), rnorm(10, 1, 90))
/// closest_point(x, y)
/// closest_point_haversine(x, y)
/// @returns
/// An `rs_POINT` vector
fn closest_point(x: List, y: List) -> Robj {

    if !y.inherits("rs_POINT") {
        panic!("`y` must be point geometries of class `rs_POINT`");
    } else if !x.inherits("rsgeo") {
        panic!("`x` must be an `rsgeo` object")
    }

    let res_vec = x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                NULL.into_robj()
            } else {
                let p: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();
                let closest = Geom::try_from(xi).unwrap().geom.closest_point(&p);

                match closest {
                    Closest::SinglePoint(pnt) => Geom::from(pnt).into(),
                    Closest::Intersection(pnt) => Geom::from(pnt).into(),
                    Closest::Indeterminate => NULL.into_robj(),
                }
            }
        })
        .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res_vec), "point")
        
}

#[extendr]
/// @export
/// @rdname closest_point
fn closest_point_haversine(x: List, y: List) -> Robj {

    if !y.inherits("rs_POINT") {
        panic!("`y` must be point geometries of class `rs_POINT`");
    } else if !x.inherits("rsgeo") {
        panic!("`x` must be an `rsgeo` object")
    }


    let res_vec = x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                NULL.into_robj()
            } else {
                let p: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                let closest = Geom::try_from(xi).unwrap().geom.haversine_closest_point(&p);

                match closest {
                    Closest::SinglePoint(pnt) => Geom::from(pnt).into(),
                    Closest::Intersection(pnt) => Geom::from(pnt).into(),
                    Closest::Indeterminate => NULL.into_robj(),
                }
            }
        })
        .collect::<Vec<Robj>>();

        as_rsgeo_vctr(List::from_values(res_vec), "point")
}

#[extendr]

/// Determine the Convexity of a LineString
/// 
/// For a given `rs_LINESTRING` vector, test its convexity. Convexity can be tested
/// strictly or strongly, as well as based on winding.
/// 
/// @param x an object of class `rs_LINESTRING`
/// 
/// See [`geo` docs for further details](https://docs.rs/geo/latest/geo/algorithm/is_convex/trait.IsConvex.html)
/// @export
/// @rdname convex
/// @returns a logical vector 
/// @examples
/// lns <- geom_linestring(
///     1:20,
///     runif(20, -5, 5),
///     rep.int(1:5, 4)
///   )
///   
/// is_convex(lns)
/// is_cw_convex(lns)
/// is_ccw_convex(lns)
/// is_strictly_convex(lns)
/// is_strictly_cw_convex(lns)
/// is_strictly_ccw_convex(lns)
fn is_convex(x: List) -> Logicals {
    // check that y is a point
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}

#[extendr]
/// @export
/// @rdname convex
fn is_ccw_convex(x: List) -> Logicals {
    // check that y is a point
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_ccw_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}

#[extendr]
/// @export
/// @rdname convex
fn is_cw_convex(x: List) -> Logicals {
    // check that y is a point
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_cw_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}

#[extendr]
/// @export
/// @rdname convex
fn is_strictly_convex(x: List) -> Logicals {
    // check that y is a point
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_strictly_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}

#[extendr]
/// @export
/// @rdname convex
fn is_strictly_ccw_convex(x: List) -> Logicals {
    // check that y is a point
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_strictly_ccw_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}


#[extendr]
/// @export
/// @rdname convex
fn is_strictly_cw_convex(x: List) -> Logicals {
    // check that y is a point
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be LineString geometries of class `rs_LINESTRING`");
    }

    x.iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(Geom::try_from(xi).unwrap())
                    .unwrap()
                    .is_strictly_cw_convex()
                    .into()
            }
        })
        .collect::<Logicals>()
}


#[extendr]
/// Interpolate a Point on a LineString
/// 
/// Finds the point that lies a given fraction along a line.
/// 
/// @param x an object of class `rs_LINESTRING`
/// @param fraction a numeric vector of length 1 or the same length as `x`. Must be a value between 0 and 1 inclusive.
/// 
/// @export
/// @returns 
/// An object of class `rs_POINT`
/// @examples
/// x <- geom_linestring(c(-1, 0, 0), c(0, 0, 1))
/// line_interpolate_point(x, 0.5)
fn line_interpolate_point(x: List, fraction: Doubles) -> Robj {
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be a `rs_LINESTRING`")
    }

    let n_f = fraction.len(); 
    let n_x = x.len();


    if (n_x > n_f) && (n_f != 1) {
        panic!("`fraction` must be the same length as `x` or length 1")
    }

    let fraction = match n_f == 1 {
        true => Doubles::from_values(vec![fraction[0]; n_x]),
        false => fraction
    };

    let res_vec = x.iter()
        .zip(fraction.into_iter())
        .map(|((_, xi), fi)| {
            if xi.is_null() || !fi.is_real() {
                NULL.into_robj()
            } else {
                let l: LineString = Geom::try_from(xi).unwrap().try_into().unwrap();

                let res = l.line_interpolate_point(fi.inner());

                match res {
                    Some(res) => Geom::from(res).into(),
                    None => NULL.into_robj(),
                }
            }
        })
        .collect::<Vec<Robj>>();
    
    as_rsgeo_vctr(List::from_values(res_vec), "point")
        
}

#[extendr]
/// Locate a Point on a LineString
/// 
/// Calculates the fraction of a LineString's length to a point 
/// that is closes to a corresponding point in `y`.
/// 
/// @param x an object of class `rs_LINESTRING`
/// @param y an object of class `rs_POINT`
/// 
/// @export
/// @returns 
/// A numeric vector containing the fraction of of the LineString that
/// would need to be traveled to reach the closest point.
/// @examples
/// x <- geom_linestring(c(-1, 0, 0), c(0, 0, 1))
/// y <- geom_point(-0.5, 0)
/// locate_point_on_line(x, y)
fn locate_point_on_line(x: List, y: List) -> Doubles {
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be an `rs_LINESTRING`")
    } else if !y.inherits("rs_POINT") {
        panic!("`y` must be an `rs_POINT")
    }

    let n_y = y.len(); 
    let n_x = x.len();

    if n_y != n_x {
        panic!("`y` must be the same length as `x`")
    }

    x.iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let l: LineString = Geom::try_from(xi).unwrap().geom.try_into().unwrap();

                let p: Point = Geom::try_from(yi).unwrap().geom.try_into().unwrap();

                l.line_locate_point(&p).into()
            }
        })
        .collect::<Doubles>()
}

// wrapped and documented externally
#[extendr]
fn line_segmentize_(x: List, n: Integers) -> Robj {

    let n_x = x.len();
    let n_n = n.len(); 

    if (n_x > n_n) && (n_n != 1) {
        panic!("`n` must be the same length as `x` or length 1")
    }

    let n = match n_n == 1 {
        true => Integers::from_values(vec![n[0]; n_x]),
        false => n
    };

    let x = geometry_from_list(x);

    let res_vec = x
        .into_par_iter()
        .zip(n.into_par_iter())
        .map(|(xi, ni)| {
            if ni.is_na() {
                None
            } else {
                match xi {
                    Some(g) => LineString::try_from(g)
                        .unwrap()
                        .line_segmentize(ni.inner() as usize),
                    None => None
                }
            }
        })
        .collect::<Vec<Option<MultiLineString>>>();

    
    let res = 
        res_vec
            .into_iter()
            .map(|xi| match xi {
                Some(xi) => Geom::from(xi).into_robj(),
                None => ().into_robj()
            })
            .collect::<Vec<Robj>>();

    as_rsgeo_vctr(List::from_values(res), "multilinestring")
}


extendr_module! {
    mod query;
    fn bearing_geodesic;
    fn bearing_haversine;
    fn closest_point;
    fn closest_point_haversine;
    fn is_convex;
    fn is_ccw_convex;
    fn is_cw_convex;
    fn is_strictly_convex;
    fn is_strictly_ccw_convex;
    fn is_strictly_cw_convex;
    fn line_interpolate_point;
    fn locate_point_on_line;
    fn line_segmentize_;
}
