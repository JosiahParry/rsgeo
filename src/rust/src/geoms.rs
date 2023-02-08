use std::vec;

use crate::mat_to_rs;
use crate::matrix_to_coords;
use crate::types::*;
use extendr_api::prelude::*;
use extendr_api::wrapper::{ExternalPtr, RMatrix};
use geo::{point, MultiLineString, MultiPoint, MultiPolygon};
use geo_types::{Geometry, LineString, Point, Polygon};
use ndarray::Axis;

#[extendr]
fn print_geom(x: Robj) {
    let x: Geom = x.try_into().unwrap();
    rprintln!("{x}");
}

#[extendr]
fn print_geoms(x: List) {
    for (_, robj) in x.into_iter() {
        let x = Geom::try_from(robj).unwrap();
        rprintln!("{x}");
    }
}
// POINTS

/// Create geometry
///
#[extendr]
/// @export
/// @rdname geometry
fn geom_point(x: f64, y: f64) -> Robj {
    let pnt = Point::new(x, y);
    let pnt: Geom = pnt.try_into().unwrap();
    to_pntr(pnt)
}

#[extendr]
/// @export
/// @rdname geometry
fn geom_points(x: List) -> Robj {
    let n = x.len();
    let mut res: Vec<Robj> = Vec::with_capacity(n);

    for i in 0..n {
        let xi: Doubles = x[i].to_owned().try_into().unwrap();
        res.push(geom_point(xi[0].0, xi[1].0));
    }

    List::from_values(res)
        .set_attrib("class", "rs_POINT")
        .unwrap()
}

/// Create a list of points
/// Given a matrix of x, y coordinates, create a list of points
#[extendr]
/// @export
/// @rdname geometry
pub fn geom_points_matrix(x: RMatrix<f64>) -> Robj {
    let arr = mat_to_rs(x);
    let n = arr.nrows();
    let mut res: Vec<Robj> = Vec::with_capacity(n);

    for row in arr.axis_iter(Axis(0)) {
        res.push(geom_point(row[0], row[1]));
    }

    List::from_values(res)
        .set_attrib("class", "rs_POINT")
        .unwrap()
}

// MULTIPOINT

#[extendr]
/// @export
/// @rdname geometry
fn geom_multipoint(x: RMatrix<f64>) -> Robj {
    let arr = mat_to_rs(x);

    let mpnt = MultiPoint::new(
        arr.axis_iter(Axis(0))
            .map(|x| point! {x : x[0], y: x[1]})
            .collect::<Vec<Point>>(),
    );

    to_pntr(Geom {
        geom: Geometry::from(mpnt),
    })
}

#[extendr]
/// @export
/// @rdname geometry
fn geom_multipoints(x: List) -> Robj {
    x.into_iter()
        .map(|(_, x)| geom_multipoint(RMatrix::try_from(x).unwrap()))
        .collect::<List>()
        .as_robj()
        .set_attrib("class", "rs_MULTIPOINT")
        .unwrap()
}

// POLYGONS

#[extendr]
/// @export
/// @rdname geometry
pub fn geom_polygon(x: List) -> Robj {
    let n = x.len();
    let mut linestrings: Vec<LineString> = Vec::with_capacity(n);

    let exterior = matrix_to_coords(x[0].as_matrix().unwrap());
    let exterior = LineString::new(exterior);

    if n > 1 {
        for i in 1..n {
            let xi: RMatrix<f64> = x[i].to_owned().try_into().unwrap();
            let coords = matrix_to_coords(xi);
            let line = LineString::new(coords);
            linestrings.push(line);
        }
    }

    let polygon = Polygon::new(exterior, linestrings);
    let polygon: Geom = polygon.into();

    to_pntr(polygon)
}

// List of polygons
// a list of polygons
#[extendr]
/// @export
/// @rdname geometry
fn geom_polygons(x: List) -> Robj {
    let n = x.len();
    let mut polygons: Vec<Robj> = Vec::with_capacity(n);

    for (_, robj) in x.into_iter() {
        let robj: List = robj.try_into().unwrap();
        polygons.push(geom_polygon(robj));
    }

    List::from_values(polygons)
        .set_attrib("class", "rs_POLYGON")
        .unwrap()
}

// MULTIPOLYGON
#[extendr]
/// @export
/// @rdname geometry
fn geom_multipolygon(x: List) -> Robj {
    let res = MultiPolygon::new(
        x.into_iter()
            .map(|(_, x)| polygon_inner(List::try_from(x).unwrap()))
            .collect::<Vec<Polygon>>(),
    );

    let res: Geom = res.into();

    to_pntr(res)
}

#[extendr]
/// @export
/// @rdname geometry
fn geom_multipolygons(x: List) -> Robj {
    x.into_iter()
        .map(|(_, x)| geom_multipolygon(List::try_from(x).unwrap()))
        .collect::<List>()
        .set_attrib("class", "rs_MULTIPOLYGON")
        .unwrap()
}

// LINESTRING

#[extendr]
/// @export
/// @rdname geometry
fn geom_linestring(x: RMatrix<f64>) -> Robj {
    let coords = matrix_to_coords(x);
    let lns = LineString::new(coords);
    to_pntr(Geom::try_from(lns).unwrap())
}

#[extendr]
/// @export
/// @rdname geometry
fn geom_linestrings(x: List) -> Robj {
    x.into_iter()
        .map(|(_, x)| geom_linestring(RMatrix::try_from(x).unwrap()))
        .collect::<List>()
        .set_attrib("class", "rs_LINESTRING")
        .unwrap()
}

// utility function to take a list and convert to a Polygon
// will be used to collect into `Vec<Polygon>` and thus into `MultiPolygon`
fn polygon_inner(x: List) -> Polygon {
    let n = x.len();
    let mut linestrings: Vec<LineString> = Vec::with_capacity(n);

    let exterior = matrix_to_coords(x[0].as_matrix().unwrap());
    let exterior = LineString::new(exterior);

    if n > 1 {
        for i in 1..n {
            let xi: RMatrix<f64> = x[i].to_owned().try_into().unwrap();
            let coords = matrix_to_coords(xi);
            let line = LineString::new(coords);
            linestrings.push(line);
        }
    }

    Polygon::new(exterior, linestrings)
}

// MUlTILINESTRING
#[extendr]
/// @export
/// @rdname geometry
fn geom_multilinestring(x: List) -> Robj {
    let vec_lns = x
        .into_iter()
        .map(|(_, x)| LineString::new(matrix_to_coords(RMatrix::try_from(x).unwrap())))
        .collect::<Vec<LineString>>();

    let res = MultiLineString::new(vec_lns).into();
    to_pntr(res)
}

#[extendr]
/// @export
/// @rdname geometry
fn geom_multilinestrings(x: List) -> Robj {
    x.into_iter()
        .map(|(_, x)| geom_multilinestring(List::try_from(x).unwrap()))
        .collect::<List>()
        .set_attrib("class", "rs_MULTILINESTRING")
        .unwrap()
}

// utility function to extract a Vec of Geoms from a list
pub fn from_list(x: List) -> Vec<Geom> {
    x.into_iter()
        .map(|(_, robj)| Geom::try_from(robj).unwrap())
        .collect::<Vec<_>>()
}

// helpers to cast to the proper external pointer
pub fn to_pntr(x: Geom) -> Robj {
    let cls = match x.geom {
        Geometry::Point(ref _geom) => "point",
        Geometry::MultiPoint(ref _geom) => "multipoint",
        Geometry::LineString(ref _geom) => "linestring",
        Geometry::MultiLineString(ref _geom) => "multilinestring",
        Geometry::Polygon(ref _geom) => "polygon",
        Geometry::MultiPolygon(ref _geom) => "multipolygon",
        _ => "",
    };

    ExternalPtr::new(x)
        .as_robj()
        .set_attrib("class", cls)
        .unwrap()
}

// Macro to generate exports
extendr_module! {
    mod geoms;
    fn geom_point; // a single point
    fn geom_points; // a list of points
    fn geom_points_matrix; // a matrix of coordinates
    fn geom_multipoint; // a single multipoint from matrix of coords
    fn geom_multipoints; // a list of coordinates (sfc of multipoints)
    fn geom_polygon; // a list of coordinates
    fn geom_polygons; // a list of polygons
    fn geom_multipolygon; // a list of a list of coordinates
    fn geom_multipolygons; // a list of a list of a list of coordinates
    fn geom_linestring;
    fn geom_linestrings;
    fn geom_multilinestring;
    fn geom_multilinestrings;
    fn print_geom;
    fn print_geoms;
}
