use extendr_api::prelude::*;
use geo_types::{MultiPolygon, MultiPoint, MultiLineString, Polygon, LineString, Point};
use crate::types::Geom;
use crate::to_pntr;
use crate::utils::{geom_class};
use crate::geoms::from_list;


// multis to the single varietys 
#[extendr]
fn expand_multipolygon(x: Robj) -> Robj {
    let x = Geom::from(x).geom;
    MultiPolygon::try_from(x)
        .unwrap()
        .0
        .into_iter()
        .map(|poly| to_pntr(Geom::from(poly)))
        .collect::<List>()
        .into_robj()
        .set_attrib("class", geom_class("polygon"))
        .unwrap()
}

#[extendr]
fn expand_multipoint(x: Robj) -> Robj {
    let x = Geom::from(x).geom;
    MultiPoint::try_from(x)
        .unwrap()
        .0
        .into_iter()
        .map(|pnt| to_pntr(Geom::from(pnt)))
        .collect::<List>()
        .into_robj()
        .set_attrib("class", geom_class("point"))
        .unwrap()
}

#[extendr]
fn expand_multilinestring(x: Robj) -> Robj {
    let x = Geom::from(x).geom;
    MultiLineString::try_from(x)
        .unwrap()
        .0
        .into_iter()
        .map(|pnt| to_pntr(Geom::from(pnt)))
        .collect::<List>()
        .into_robj()
        .set_attrib("class", geom_class("linestring"))
        .unwrap()
}

// primitives to components
// polygon to linestrings

#[extendr]
fn expand_polygon(x: Robj) -> Robj {
    let x = Geom::from(x).geom;
    let poly = Polygon::try_from(x).unwrap();
    
    let ext = poly.exterior().to_owned();
    let interior = poly.interiors().to_owned();
    
    let mut res: Vec<Robj> = Vec::with_capacity(interior.len() + 1);

    res.push(to_pntr(Geom::from(ext)));

    for line in interior.into_iter() {
        res.push(to_pntr(Geom::from(line)))
    }

    List::from_values(res)
        .set_attrib("class", geom_class("linestring"))
        .unwrap()
}

#[extendr]
fn expand_linestring(x: Robj) -> Robj {
    let x = Geom::from(x).geom;
    let line = LineString::try_from(x).unwrap();
    let res = line.0
        .into_iter()
        .map(|x| to_pntr(Geom::from(Point::try_from(x).unwrap())))
        .collect::<List>()
        .into_robj();

    res.set_attrib("class", geom_class("point")).unwrap()
}


// building primitives up
// vec points -> Line
// vec points -> multipoint
#[extendr]
fn cast_points_line(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| Point::try_from(x.geom).unwrap())
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(LineString::from(x)))
}

#[extendr]
fn cast_points_multipoint(x: List) -> Robj {
    let x = from_list(x)
    .into_iter()
    .map(|x| Point::try_from(x.geom).unwrap())
    .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::from(x)))
}

// vec lines -> polygon
// vec lines -> multiline
#[extendr]
fn cast_lines_polygon(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| LineString::try_from(x.geom).unwrap())
        .collect::<Vec<LineString>>();

        // shoot do i need to handle winding here? 
        // assuming the user has the widning correct here
    let n = x.len();
    let exterior = x[0].clone();
    let interior = if n > 1 {
        x[1..n].to_owned()
    } else {
        vec![]
    };

    to_pntr(Geom::from(Polygon::new(exterior, interior)))
}

#[extendr]
fn cast_lines_multilinestring(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| LineString::try_from(x.geom).unwrap())
        .collect::<Vec<LineString>>();

        to_pntr(Geom::from(MultiLineString::new(x)))
}

#[extendr]
fn cast_multiline_polygon(x: Robj) -> Robj {
    let x = MultiLineString::try_from(Geom::from(x).geom).unwrap().0;
    
    let n = x.len();
    let exterior = x[0].clone();
    let interior = if n > 1 {
        x[1..n].to_owned()
    } else {
        vec![]
    };

    to_pntr(Geom::from(Polygon::new(exterior, interior)))
}

// multiline -> polygon
// polygons -> multipolygon


extendr_module! {
    mod casting;
    fn expand_multipolygon;
    fn expand_multilinestring;
    fn expand_multipoint;
    fn expand_polygon;
    fn expand_linestring;
    fn cast_points_line;
    fn cast_points_multipoint;
    fn cast_lines_multilinestring;
    fn cast_lines_polygon;
}