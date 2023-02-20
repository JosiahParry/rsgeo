use extendr_api::prelude::*;
use geo_types::*;
use crate::utils::geom_class;
use crate::types::Geom;
use crate::to_pntr;


// EXPAND -------------------------------------------------------------------------
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
// polygon to linestring

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


// Expansion Hierarchy
// MultiPolygon -> Polygon
// (Polygon -> LineString) OR (MultiLineString -> LineString)
// LineString -> Point

#[extendr]
fn expand_linestrings(x: List) -> List {
    x
        .into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                Robj::from(extendr_api::NULL)
            } else {
                expand_linestring(robj)
            }
        })
        .collect::<List>()
}


#[extendr]
fn expand_multipolygons(x: List) -> List {
    x
        .into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                Robj::from(extendr_api::NULL)
            } else {
                expand_multipolygon(robj)
            }
        })
        .collect::<List>()
}


#[extendr]
fn expand_multilinestrings(x: List) -> List {
    x
        .into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                Robj::from(extendr_api::NULL)
            } else {
                expand_multilinestring(robj)
            }
        })
        .collect::<List>()
}


#[extendr]
fn expand_multipoints(x: List) -> List {
    x
        .into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                Robj::from(extendr_api::NULL)
            } else {
                expand_multipoint(robj)
            }
        })
        .collect::<List>()
}


#[extendr]
fn expand_polygons(x: List) -> List {
    x
        .into_iter()
        .map(|(_, robj)| {
            if robj.is_null() {
                Robj::from(extendr_api::NULL)
            } else {
                expand_polygon(robj)
            }
        })
        .collect::<List>()
}

extendr_module! {
    mod expand;
    fn expand_linestring;
    fn expand_linestrings;
    fn expand_multipolygon;
    fn expand_multipolygons;
    fn expand_multilinestring;
    fn expand_multilinestrings;
    fn expand_multipoint;
    fn expand_multipoints;
    fn expand_polygon;
    fn expand_polygons;
}