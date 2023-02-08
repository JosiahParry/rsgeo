use crate::geoms::*;
use crate::types::Geom;
use extendr_api::prelude::*;
use geo::{Geometry, Simplify, SimplifyVW};

/// Simplfiy Geometries
/// @export
#[extendr]
fn simplify_geom(x: Robj, epsilon: f64) -> Robj {
    // check for a valid class
    let is_lns = x.inherits("linestring");
    let is_mlns = x.inherits("multilinestring");
    let is_mply = x.inherits("multipolygon");
    let is_ply = x.inherits("polygon");
    let all_inheritances = [is_lns, is_mlns, is_mply, is_ply];
    let check = all_inheritances.into_iter().any(|cls| cls);

    if !check {
        panic!("`x` is an invalid geometry type")
    }

    let x = Geom::from(x).geom;

    let res_geom = match x {
        Geometry::LineString(geom) => Geom::from(geom.simplify(&epsilon)),
        Geometry::MultiLineString(geom) => Geom::from(geom.simplify(&epsilon)),
        Geometry::Polygon(geom) => Geom::from(geom.simplify(&epsilon)),
        Geometry::MultiPolygon(geom) => Geom::from(geom.simplify(&epsilon)),
        _ => Geom::from(geo::point!(x: 0.0, y: 0.0)),
    };

    to_pntr(res_geom)
}

/// @export
/// @rdname simplify_geom
#[extendr]
fn simplify_geoms(x: List, epsilon: f64) -> Robj {
    let cls = x.class().unwrap();

    let res = x
        .into_iter()
        .map(|(_, x)| simplify_geom(x, epsilon))
        .collect::<List>();

    res.set_class(cls).unwrap()
}

/// @export
/// @rdname simplify_geom
#[extendr]
fn simplify_vw_geom(x: Robj, epsilon: f64) -> Robj {
    // check for a valid class
    let is_lns = x.inherits("linestring");
    let is_mlns = x.inherits("multilinestring");
    let is_mply = x.inherits("multipolygon");
    let is_ply = x.inherits("polygon");
    let all_inheritances = [is_lns, is_mlns, is_mply, is_ply];
    let check = all_inheritances.into_iter().any(|cls| cls);

    if !check {
        panic!("`x` is an invalid geometry type")
    }

    let x = Geom::from(x).geom;

    let res_geom = match x {
        Geometry::LineString(geom) => Geom::from(geom.simplifyvw(&epsilon)),
        Geometry::MultiLineString(geom) => Geom::from(geom.simplifyvw(&epsilon)),
        Geometry::Polygon(geom) => Geom::from(geom.simplifyvw(&epsilon)),
        Geometry::MultiPolygon(geom) => Geom::from(geom.simplifyvw(&epsilon)),
        _ => Geom::from(geo::point!(x: 0.0, y: 0.0)),
    };

    to_pntr(res_geom)
}

/// @export
/// @rdname simplify_geom
#[extendr]
fn simplify_vw_geoms(x: List, epsilon: f64) -> Robj {
    let cls = x.class().unwrap();

    let res = x
        .into_iter()
        .map(|(_, x)| simplify_geom(x, epsilon))
        .collect::<List>();

    res.set_class(cls).unwrap()
}

extendr_module! {
    mod simplification;
    fn simplify_geom;
    fn simplify_geoms;
    fn simplify_vw_geom;
    fn simplify_vw_geoms;
}
