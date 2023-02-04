use extendr_api::prelude::*;
use extendr_api::Robj;
use geo_types::*;

use crate::types::Geom;


#[extendr]
fn geom_to_r(x: Robj) -> Robj {
    let x = Geom::from(x);
    let x = Robj::from(x);
    x
}

#[extendr]
fn geoms_to_r(x: List) -> List {
    x.into_iter()
        .map(|(_, robj)| geom_to_r(robj))
        .collect::<List>()
}


// from point to Robj
impl From<Geom> for Robj {
    fn from(x: Geom) -> Robj {
        let x = x.geom;
        match x {
            Geometry::Point(x) => from_point(x),
            Geometry::MultiPoint(x) => from_multipoint(x),
            Geometry::LineString(x) => from_linestring(x),
            Geometry::MultiLineString(x) => from_multilinestring(x),
            Geometry::Polygon(x) => from_polygon(x),
            Geometry::MultiPolygon(x) => from_multipolygon(x),
            _ => Robj::from(List::new(0))
        }
    }
}

fn from_coord(x: Coord) -> [f64; 2] {
    [x.x, x.y]
}


fn from_point(x: Point) -> Robj {
    let x = from_coord(x.0);
    Robj::try_from(x).unwrap()
}


fn from_multipoint(x: MultiPoint) -> Robj {
    let x = x.into_iter()
        .map(|p| from_coord(p.into()))
        .collect::<Vec<[f64;2]>>();

    let res = RMatrix::new_matrix(x.len(), 2, |r, c| x[r][c]);
    Robj::from(res)
}

fn from_linestring(x: LineString) -> Robj {
    let x = x.into_iter()
    .map(|x| from_coord(x))
    .collect::<Vec<[f64;2]>>();

    let res = RMatrix::new_matrix(x.len(), 2, |r, c| x[r][c]);
    Robj::from(res)
}

fn from_multilinestring(x: MultiLineString) -> Robj {
    let res = x.0.into_iter()
        .map(|x| from_linestring(x))
        .collect::<List>();

    Robj::from(res)
        
}

fn from_polygon(x: Polygon) -> Robj {
    let exterior = x.exterior().to_owned();
    let interriors = x.interiors().to_owned();

    // combine the exterior ring and interrior rings into 1 vector first
    // then iterate through them. 
    // no method to go from Polygon to multilinestring
    let mut res: Vec<LineString> = Vec::with_capacity(interriors.len() + 1);
    res.push(exterior);
    res.extend(interriors.into_iter());
    
    let res = res.into_iter()
        .map(|i| from_linestring(i))
        .collect::<Vec<Robj>>();

    Robj::from(List::from_values(res))
}

fn from_multipolygon(x: MultiPolygon) -> Robj {
    let res = x.into_iter()
        .map(|p| from_polygon(p))
        .collect::<List>();

    Robj::from(res)
}

 
extendr_module! {
    mod conversion;
    fn geom_to_r;
    fn geoms_to_r;
}