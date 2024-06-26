use extendr_api::prelude::*;
use geo_types::{coord, point, Coord, LineString, MultiPoint, Point, Polygon};
use sfconversions::{
    vctrs::{as_rsgeo_vctr, geom_class},
    Geom, IntoGeom,
};
use std::collections::BTreeMap;

pub trait IsReal {
    fn is_real(&self) -> bool;
}

impl IsReal for Rfloat {
    fn is_real(&self) -> bool {
        !(self.is_na() || self.is_infinite() || self.is_nan())
    }
}

#[extendr]
fn geom_point_(x: Doubles, y: Doubles) -> Robj {
    let n_x = x.len();
    let n_y = y.len();

    if n_x != n_y {
        panic!("`x` and `y` must be the same length")
    }

    let mut res = List::new(x.len());

    for (i, (xi, yi)) in x.iter().zip(y.iter()).enumerate() {
        if xi.is_real() && yi.is_real() {
            res.set_elt(
                i,
                sfconversions::constructors::geom_point(xi.inner(), yi.inner()),
            )
            .unwrap();
        }
    }

    res.set_attrib("class", geom_class("point")).unwrap();
    res.into()
}

#[extendr]
fn geom_multipoint_(x: Doubles, y: Doubles, id: Integers) -> Robj {
    let n_id = id.len();
    let n_x = x.len();
    let n_y = y.len();

    if n_x != n_y {
        panic!("`x` and `y` must be the same length")
    } else if (n_id != n_x) && (n_id != 1) {
        panic!("`id` must be the same length as `x` or length 1")
    }

    let id = match n_id == 1 {
        true => Integers::from_values(vec![1; n_x]),
        false => id,
    };

    // create empty hash map to store unique vectors of points
    let mut map_mpnts: BTreeMap<i32, Vec<Point>> = BTreeMap::new();

    // iterate through everything and create points
    for ((xi, yi), idx) in x.iter().zip(y.iter()).zip(id.iter()) {
        // check to see if xi and yi are real values / non-missing
        if xi.is_real() && yi.is_real() {
            let pnt = point!(x: xi.inner(), y: yi.inner());

            map_mpnts.entry(idx.inner()).or_insert(Vec::new()).push(pnt);
        }
    }

    // iterate through the hash map to create a new multipoint from each
    let res_vec = map_mpnts
        .into_iter()
        .map(|(_, pts)| Geom::from(MultiPoint::new(pts)))
        .collect::<Vec<Geom>>();

    // create multipoint vector
    let mut robj = List::from_values(res_vec);
    robj.set_class(geom_class("multipoint")).unwrap();
    robj.into()
}

#[extendr]
fn geom_linestring_(x: Doubles, y: Doubles, id: Integers) -> Robj {
    let n_id = id.len();
    let n_x = x.len();
    let n_y = y.len();

    if n_x != n_y {
        panic!("`x` and `y` must be the same length")
    } else if (n_id != n_x) && (n_id != 1) {
        panic!("`id` must be the same length as `x` or length 1")
    }

    let id = match n_id == 1 {
        true => Integers::from_values(vec![1; n_x]),
        false => id,
    };

    // create empty hash map to store unique vectors of points
    let mut map_mpnts: BTreeMap<i32, Vec<Coord>> = BTreeMap::new();

    // iterate through everything and create points
    for ((xi, yi), idx) in x.iter().zip(y.iter()).zip(id.iter()) {
        // check to see if xi and yi are real values / non-missing
        if xi.is_real() && yi.is_real() {
            let pnt = coord!(x: xi.inner(), y: yi.inner());

            map_mpnts.entry(idx.inner()).or_insert(Vec::new()).push(pnt);
        }
    }

    // iterate through the hash map to create a new multipoint from each
    let res_vec = map_mpnts
        .into_iter()
        .map(|(_, pts)| Geom::from(LineString::new(pts)))
        .collect::<Vec<Geom>>();

    // create multipoint vector
    let mut robj = List::from_values(res_vec);
    robj.set_class(geom_class("linestring")).unwrap();
    robj.into()
}

#[extendr]
fn geom_polygon_(x: Doubles, y: Doubles, id: Integers, ring: Integers) -> Robj {
    let n_id = id.len();
    let n_ring = ring.len();
    let n_x = x.len();
    let n_y = y.len();

    if n_x != n_y {
        panic!("`x` and `y` must be the same length")
    } else if (n_id != n_x) && (n_id != 1) {
        panic!("`id` must be the same length as `x` or length 1")
    } else if (n_ring != n_x) && (n_ring != 1) {
        panic!("`ring` must be the same length as `x` or length 1")
    }

    let id = match n_id == 1 {
        true => Integers::from_values(vec![1; n_x]),
        false => id,
    };

    let ring: Integers = match n_ring == 1 {
        true => Integers::from_values(vec![1; n_x]),
        false => ring,
    };

    // create empty hash map to store unique vectors of points for each ring
    let mut map_rings: BTreeMap<i32, BTreeMap<i32, Vec<Coord>>> = BTreeMap::new();

    // iterate through everything and create points
    for (((xi, yi), idx), ring_idx) in x.iter().zip(y.iter()).zip(id.iter()).zip(ring.iter()) {
        // check to see if xi and yi are real values / non-missing
        if xi.is_real() && yi.is_real() {
            let pnt = coord!(x: xi.inner(), y: yi.inner());

            map_rings
                .entry(ring_idx.inner())
                .or_insert(BTreeMap::new())
                .entry(idx.inner())
                .or_insert(Vec::new())
                .push(pnt);
        }
    }

    // iterate through the hash map to create polygons from each ring
    let res_vec = map_rings
        .into_iter()
        .map(|(_, ring_points)| {
            let polygons = ring_points
                .into_iter()
                .map(|(_, pts)| LineString::new(pts))
                .collect::<Vec<LineString>>();

            let n = polygons.len();

            let poly = if n > 1 {
                let mut piter = polygons.into_iter();
                let ext = piter.next().unwrap();
                let inner = piter.collect::<Vec<LineString>>();
                Polygon::new(ext, inner)
            } else {
                Polygon::new(polygons.into_iter().next().unwrap(), vec![])
            };

            Geom::from(poly)
        })
        .collect::<Vec<Geom>>();

    // create multipolygon vector
    let mut robj = List::from_values(res_vec);
    robj.set_class(geom_class("polygon")).unwrap();
    robj.into()
}

#[extendr]
/// @export
/// @rdname construction
fn geom_line(x: List, y: List) -> Robj {
    if !x.inherits("rs_POINT") || !y.inherits("rs_POINT") {
        panic!("`x` and `y` must be of class `rs_POINT`")
    }

    let res_vec = x
        .into_iter()
        .zip(y.into_iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                NULL.into_robj()
            } else {
                let p1 = Point::from(Geom::from(xi));
                let p2 = Point::from(Geom::from(yi));
                LineString::new(vec![p1.0, p2.0]).into_geom().into_robj()
            }
        })
        .collect::<Vec<Robj>>();

    let res = List::from_values(res_vec);

    as_rsgeo_vctr(res, "linestring")
}

extendr_module! {
    mod construction;
    fn geom_point_;
    fn geom_multipoint_;
    fn geom_linestring_;
    fn geom_polygon_;
    fn geom_line;
}
