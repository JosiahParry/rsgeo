use extendr_api::{ExternalPtr, Robj};
// create an enum of geo-types
use geo_types::{Line, Rect, Point, Polygon, LineString, Geometry, MultiPoint, MultiPolygon, MultiLineString};
use std::fmt;


#[derive(Debug, Clone)]
pub struct Geom {
    pub geom: Geometry
}



impl fmt::Display for Geom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.geom)
    }
}

// FROM geo-types to Geom
impl From<Point> for Geom {
    fn from(pnt: Point) -> Self {
        let x: Geometry = pnt.into();
        Geom { geom: x }
    }
}

impl From<MultiPoint> for Geom {
    fn from(pnt: MultiPoint) -> Self {
        let x: Geometry = pnt.into();
        Geom { geom: x }
    }
}


impl From<Polygon> for Geom {
    fn from(poly: Polygon) -> Self {
        let x: Geometry = poly.into();
        Geom { geom: x }
    }
}


impl From<MultiPolygon> for Geom {
    fn from(poly: MultiPolygon) -> Self {
        let x: Geometry = poly.into();
        Geom { geom: x }
    }
}

impl From<LineString> for Geom {
    fn from(lns: LineString) -> Self {
        let x: Geometry = lns.into();
        Geom { geom: x }
    }
}

impl From<MultiLineString> for Geom {
    fn from(lns: MultiLineString) -> Self {
        let x: Geometry = lns.into();
        Geom { geom: x }
    }
}

impl From<Rect> for Geom {
    fn from(r: Rect) -> Self {
        let x: Geometry = r.into();
        Geom { geom: x }
    }
}

impl From<Line> for Geom {
    fn from(l: Line) -> Self {
        let x: Geometry = l.into();
        Geom { geom: x }
    }
}


impl From<ExternalPtr<Geom>> for Geom {
    fn from(pntr: ExternalPtr<Geom>) -> Self {
        let geo = pntr.geom.clone();
        Geom {
            geom: geo
        }
    }
}

impl From<Robj> for Geom {
    fn from(robj: Robj) -> Self {
        let robj: ExternalPtr<Geom> = robj
            .try_into()
            .unwrap();
        let robj: Geom = robj.into();
        robj
    }
}


// impl From<&Robj> for &Geom {
//     fn from(robj: &Robj) -> Self {
//         let robj: &ExternalPtr<Geom> = robj.
//             .try_into()
//             .unwrap();

//         robj
//     }
// }

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

// enum Geoms {
//     RPoints(Vec<Geom>),
//     RLines(Vec<Geom>),
//     RPolys(Vec<Geom>)
// }