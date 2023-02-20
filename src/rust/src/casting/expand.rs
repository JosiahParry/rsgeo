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


extendr_module! {
    mod expand;
    fn expand_linestring;
    
    fn expand_multipolygon;
    fn expand_multilinestring;
    fn expand_multipoint;
    fn expand_polygon;
}