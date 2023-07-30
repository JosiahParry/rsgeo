use extendr_api::prelude::*;
use std::collections::HashMap;
use geo_types::{point, Point, MultiPoint, LineString, coord, Coord};
use sfconversions::{vctrs::geom_class, Geom};

trait IsReal {
    fn is_real(&self) -> bool;
}

 impl IsReal for Rfloat {
    fn is_real(&self) -> bool {
        !(self.is_na() || self.is_infinite() || self.is_nan())
    }
 }

#[extendr]
fn geom_point(x: Doubles, y: Doubles) -> Robj {
    let mut res = List::new(x.len());
    
    for (i, (xi, yi)) in x.iter().zip(y.iter()).enumerate() {
        if xi.is_real() && yi.is_real() {
            res.set_elt(i, sfconversions::constructors::geom_point(xi.inner(), yi.inner())).unwrap();
        }
    }

    res
        .set_attrib("class", geom_class("point"))
        .unwrap()
}


#[extendr]
fn geom_multipoint(x: Doubles, y: Doubles, id: Integers) -> Robj {

    // create empty hash map to store unique vectors of points 
    let mut map_mpnts: HashMap<i32, Vec<Point>> = HashMap::new();

    // iterate through everything and create points
    for ((xi, yi), idx) in x.iter().zip(y.iter()).zip(id.iter()) {
        // check to see if xi and yi are real values / non-missing
        if xi.is_real() && yi.is_real() {
            let pnt = point!(x: xi.inner(), y: yi.inner());

            map_mpnts
                .entry(idx.inner())
                .or_insert(Vec::new())
                .push(pnt);
        }
    } 
    
    // iterate through the hash map to create a new multipoint from each
    let res_vec = map_mpnts
        .into_iter()
        .map(|(_, pts)| {
            Geom::from(MultiPoint::new(pts))
        })
        .collect::<Vec<Geom>>();

    // create multipoint vector
    List::from_values(res_vec)
        .set_class(geom_class("multipoint"))
        .unwrap()
        

}


#[extendr]
fn geom_linestring(x: Doubles, y: Doubles, id: Integers) -> Robj {

    // create empty hash map to store unique vectors of points 
    let mut map_mpnts: HashMap<i32, Vec<Coord>> = HashMap::new();

    // iterate through everything and create points
    for ((xi, yi), idx) in x.iter().zip(y.iter()).zip(id.iter()) {
        // check to see if xi and yi are real values / non-missing
        if xi.is_real() && yi.is_real() {
            let pnt = coord!(x: xi.inner(), y: yi.inner());

            map_mpnts
                .entry(idx.inner())
                .or_insert(Vec::new())
                .push(pnt);
        }
    } 
    
    // iterate through the hash map to create a new multipoint from each
    let res_vec = map_mpnts
        .into_iter()
        .map(|(_, pts)| {
            Geom::from(LineString::new(pts))
        })
        .collect::<Vec<Geom>>();

    // create multipoint vector
    List::from_values(res_vec)
        .set_class(geom_class("multipoint"))
        .unwrap()
        

}




extendr_module! {
    mod construction;
    fn geom_point;
    fn geom_multipoint;
    fn geom_linestring;
}
