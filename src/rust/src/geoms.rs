use crate::types::*;
use extendr_api::prelude::*;
use extendr_api::wrapper::{ExternalPtr, RMatrix};
use geo_types::{LineString, Polygon, Point};
use crate::matrix_to_coords;
use crate::mat_to_rs;
use ndarray::{Axis};



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

/// Create a single point
/// 
/// For a single x, y coordinate create a point
/// @export
#[extendr]
fn geom_point(x: f64, y: f64) -> Robj {
    let pnt = Point::new(x, y);
    let pnt: Geom = pnt.try_into().unwrap();
    let res = ExternalPtr::new(pnt);
    r![res].set_attrib("class", "point").unwrap()
}


/// Create a list of points 
/// Given a matrix of x, y coordinates, create a list of points
///@export
#[extendr]
pub fn geom_points_matrix(x: RMatrix<f64>) -> Robj {

    let arr = mat_to_rs(x);
    let n = arr.nrows();
    let mut res: Vec<Robj> = Vec::with_capacity(n);

    for row in arr.axis_iter(Axis(0)) {
        res.push(geom_point(row[0], row[1]));
    }

    let res = List::from_values(res);
    let res = res.set_attrib("class", "rs_POINT").unwrap();
    res
}

// POLYGONS 


#[extendr]
///@export
pub fn geom_polygon(x: List) -> Robj {
    let n = x.len();
    let mut linestrings: Vec<LineString> = Vec::with_capacity(n);

    let exterior = matrix_to_coords(x[0].as_matrix().unwrap());
    let exterior = LineString::new(exterior);

    if n > 1 {
        for i in 1..(n - 1) {
            let xi: RMatrix<f64> = x[i].to_owned().try_into().unwrap();
            let coords = matrix_to_coords(xi);
            let line = LineString::new(coords);
            linestrings.push(line);
        }
    }
    
    let polygon = Polygon::new(exterior, linestrings);
    let polygon: Geom = polygon.into();

    r![ExternalPtr::new(polygon)].set_attrib("class", "polygon").unwrap()
}

// List of polygons 
// a list of polygons 
///@export
#[extendr]
fn geom_polygons(x: List) -> Robj {
    let n = x.len();
    let mut polygons: Vec<Robj> = Vec::with_capacity(n);

    for (_, robj) in x.into_iter() {
        let robj: List = robj.try_into().unwrap();
        polygons.push(geom_polygon(robj));
    }

    let res = List::from_values(polygons);
    let res = res.set_attrib("class", "rs_POLYGON").unwrap();
    res

}


// utility function to extract a Vec of Geoms from a list 
pub fn from_list(x: List) -> Vec<Geom> {
    x.into_iter()
        .map(|(_, robj)| Geom::try_from(robj).unwrap())
        .collect::<Vec<_>>()
}


// Macro to generate exports
extendr_module! {
    mod geoms;
    fn geom_point;
    fn geom_points_matrix;
    fn geom_polygon;
    fn geom_polygons;
    fn print_geom;
    fn print_geoms;
}