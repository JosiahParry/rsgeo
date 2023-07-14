
use sfconversions::Geom;
use extendr_api::prelude::*;
use geo::{Simplify, SimplifyVw, SimplifyVwPreserve};
use geo_types::Geometry;

#[extendr]
fn simplify_geoms(x: List,  epsilon: Doubles) -> Robj {
    let n_e = epsilon.len();
    let n_x = x.len();

    if (n_x != n_e) && (n_e != 1) {
        panic!("`epsilon` must be the same length as `x` or length `1`");
    }
    let epsilon = if n_e == 1 {
        Doubles::from_values(vec![epsilon[0].inner(); n_x])
    } else {
        epsilon
    };

    // determine the input class the output must be the same type
    let cls = x.class().unwrap();

    x
        .iter()
        .zip(epsilon.iter())
        .map(|((_, xi), ei)| {
            if xi.is_null() || ei.is_na() || ei.is_infinite() || ei.is_nan() {
                NULL.into_robj()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;
                let ei = ei.inner();

                match geom {
                    Geometry::LineString(geom) => Geom::from(geom.simplify(&ei)).into(),
                    Geometry::MultiLineString(geom) => Geom::from(geom.simplify(&ei)).into(),
                    Geometry::Polygon(geom) => Geom::from(geom.simplify(&ei)).into(),
                    Geometry::MultiPolygon(geom) => Geom::from(geom.simplify(&ei)).into(),
                    _ => NULL.into_robj()
                }
            }
        }).collect::<List>()
        .set_class(cls)
        .unwrap()
}

#[extendr]
fn simplify_vw_geoms(x: List,  epsilon: Doubles) -> Robj {
    let n_e = epsilon.len();
    let n_x = x.len();

    if (n_x != n_e) && (n_e != 1) {
        panic!("`epsilon` must be the same length as `x` or length `1`");
    }
    let epsilon = if n_e == 1 {
        Doubles::from_values(vec![epsilon[0].inner(); n_x])
    } else {
        epsilon
    };

    // determine the input class the output must be the same type
    let cls = x.class().unwrap();

    x
        .iter()
        .zip(epsilon.iter())
        .map(|((_, xi), ei)| {
            if xi.is_null() || ei.is_na() || ei.is_infinite() || ei.is_nan() {
                NULL.into_robj()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;
                let ei = ei.inner();

                match geom {
                    Geometry::LineString(geom) => Geom::from(geom.simplify_vw(&ei)).into(),
                    Geometry::MultiLineString(geom) => Geom::from(geom.simplify_vw(&ei)).into(),
                    Geometry::Polygon(geom) => Geom::from(geom.simplify_vw(&ei)).into(),
                    Geometry::MultiPolygon(geom) => Geom::from(geom.simplify_vw(&ei)).into(),
                    _ => NULL.into_robj()
                }
            }
        }).collect::<List>()
        .set_class(cls)
        .unwrap()
}

#[extendr]
fn simplify_vw_preserve_geoms(x: List,  epsilon: Doubles) -> Robj {
    let n_e = epsilon.len();
    let n_x = x.len();

    if (n_x != n_e) && (n_e != 1) {
        panic!("`epsilon` must be the same length as `x` or length `1`");
    }
    let epsilon = if n_e == 1 {
        Doubles::from_values(vec![epsilon[0].inner(); n_x])
    } else {
        epsilon
    };

    // determine the input class the output must be the same type
    let cls = x.class().unwrap();

    x
        .iter()
        .zip(epsilon.iter())
        .map(|((_, xi), ei)| {
            if xi.is_null() || ei.is_na() || ei.is_infinite() || ei.is_nan() {
                NULL.into_robj()
            } else {
                let geom = Geom::try_from(xi).unwrap().geom;
                let ei = ei.inner();

                match geom {
                    Geometry::LineString(geom) => Geom::from(geom.simplify_vw_preserve(&ei)).into(),
                    Geometry::MultiLineString(geom) => Geom::from(geom.simplify_vw_preserve(&ei)).into(),
                    Geometry::Polygon(geom) => Geom::from(geom.simplify_vw_preserve(&ei)).into(),
                    Geometry::MultiPolygon(geom) => Geom::from(geom.simplify_vw_preserve(&ei)).into(),
                    _ => NULL.into_robj()
                }
            }
        }).collect::<List>()
        .set_class(cls)
        .unwrap()
}

extendr_module! {
    mod simplification;
    fn simplify_geoms;
    fn simplify_vw_geoms;
    fn simplify_vw_preserve_geoms;
}




