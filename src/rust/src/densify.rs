use extendr_api::prelude::*;

use geo::{Densify, DensifyHaversine};
use geo_types::Geometry;
use sfconversions::{
    vctrs::{as_rsgeo_vctr, rsgeo_type},
    Geom, IntoGeom,
};

#[extendr]
/// Densify linear geometries
///
/// Adds coordinates along a `LineString` ensuring that no two coordinates are
/// further than a maximum distance apart from eachother.
///
/// @param x an object with linear geometries. Can be an `rsgeo` object _except_
///   `"rs_POINT"` or `"rs_MULTIPOINT"`.
/// @param max_distance the maximum allowed distance between coordinates.
///
/// @details
///
/// `max_distance` expects meters for `densify_haversine()` whereas
/// `densify_euclidean()` expects the units of the geometry.
///
/// Be sure to use the appropriate densification function based on
/// the type of geometries you have. rsgeo does not check if your coordinates
/// are geographic or planar. It is up to you to choose the correct algorithm.
///
/// @examples
///
/// line <- geom_linestring(1:10, 10:1)
/// densify_euclidean(line, 0.5)
/// densify_haversine(line, 100000)
///
/// @export
/// @rdname densify
fn densify_euclidean(x: List, max_distance: Doubles) -> Robj {
    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`.");
    } else if x.inherits("rs_POINT") || x.inherits("rs_MULTIPOINT") {
        panic!("`x` cannot densify point geometries.")
    }

    let out_class = rsgeo_type(&x);

    let n_x = x.len();
    let n_md = max_distance.len();

    if (n_x > n_md) && (n_md != 1) {
        panic!("`max_distance` must be the same length as `x` or length 1")
    }

    let max_distance = match n_md == 1 {
        true => Doubles::from_values(vec![max_distance[0]; n_x]),
        false => max_distance,
    };

    let res_vec = x
        .into_iter()
        .zip(max_distance.iter())
        .map(|((_, xi), md)| {
            let xi = <&Geom>::try_from(&xi).unwrap();
            match &xi.geom {
                Geometry::LineString(l) => l.densify(md.inner()).into_geom(),
                Geometry::MultiLineString(l) => l.densify(md.inner()).into_geom(),
                Geometry::Polygon(p) => p.densify(md.inner()).into_geom(),
                Geometry::MultiPolygon(p) => p.densify(md.inner()).into_geom(),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Geom>>();

    let res = List::from_values(res_vec);

    as_rsgeo_vctr(res, out_class.as_str())
}

#[extendr]
/// @export
/// @rdname densify
fn densify_haversine(x: List, max_distance: Doubles) -> Robj {
    if !x.inherits("rsgeo") {
        panic!("`x` must be of class `rsgeo`.");
    } else if x.inherits("rs_POINT") || x.inherits("rs_MULTIPOINT") {
        panic!("`x` cannot densify point geometries.")
    }

    let out_class = rsgeo_type(&x);
    let n_x = x.len();
    let n_md = max_distance.len();

    if (n_x > n_md) && (n_md != 1) {
        panic!("`max_distance` must be the same length as `x` or length 1")
    }

    let max_distance = match n_md == 1 {
        true => Doubles::from_values(vec![max_distance[0]; n_x]),
        false => max_distance,
    };

    let res_vec = x
        .into_iter()
        .zip(max_distance.iter())
        .map(|((_, xi), md)| {
            let xi = <&Geom>::try_from(&xi).unwrap();
            match &xi.geom {
                Geometry::LineString(l) => l.densify_haversine(md.inner()).into_geom(),
                Geometry::MultiLineString(l) => l.densify_haversine(md.inner()).into_geom(),
                Geometry::Polygon(p) => p.densify_haversine(md.inner()).into_geom(),
                Geometry::MultiPolygon(p) => p.densify_haversine(md.inner()).into_geom(),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Geom>>();

    let res = List::from_values(res_vec);

    as_rsgeo_vctr(res, out_class.as_str())
}

extendr_module! {
    mod densify;
    fn densify_euclidean;
    fn densify_haversine;
}
