// create an enum of geo-types
use geo_types::{Point, Polygon, LineString, Geometry};


pub struct Geom {
    pub geom: Geometry
}


use std::fmt;
impl fmt::Display for Geom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.geom)
    }
}

// FROM geo-types to Geom
impl From<Point> for Geom {
    fn from(pnt: Point) -> Self {
        let x: Geometry = pnt.into();
        Geom { geom: x}
    }
}

impl From<Polygon> for Geom {
    fn from(poly: Polygon) -> Self {
        let x: Geometry = poly.into();
        Geom { geom: x}
    }
}

impl From<LineString> for Geom {
    fn from(lns: LineString) -> Self {
        let x: Geometry = lns.into();
        Geom { geom: x}
    }

}


// TO geo-types from Geom
impl From<Geom> for Polygon {
    fn from(geom: Geom) -> Self {
        let x = geom.geom;
        let x: Polygon = x.try_into().unwrap();
        x
    }
}


impl From<Geom> for LineString {
    fn from(geom: Geom) -> Self {
        let x = geom.geom;
        let x: LineString = x.try_into().unwrap();
        x
    }
}

impl From<Geom> for Point {
    fn from(geom: Geom) -> Self {
        let x = geom.geom;
        let x: Point = x.try_into().unwrap();
        x
    }
}

enum Geoms {
    RPoints(Vec<Geom>),
    RLines(Vec<Geom>),
    RPolys(Vec<Geom>)
}