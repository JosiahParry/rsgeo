

use geo::{Polygon, polygon};
// use geo_types::{Polygon, Geometry, coord, Point};


// use std::fs::File;
// use std::io::{BufReader, BufRead};
// use wkt::{TryFromWkt};




// fn to_point(arr: [f64;2]) -> Point{
//     point! { x: arr[0], y: arr[1]}
// }

mod intersects;
//use crate::intersects::*;

mod types;
use crate::types::*;

fn main() {


    // // read geometries from a text file
    // let f = File::open("geoms.txt").expect("this shit to work"); 
    // let f = BufReader::new(f);

    // // creater a vector of polygons
    // let mut all_polys: Vec<Polygon> = Vec::new();

    // for line in f.lines() {
    //     let line = line.expect("Unable to read line");
    //     let ply: Polygon<f64> = Polygon::try_from_wkt_str(line.as_str()).unwrap();
    //     all_polys.push(ply);
    
    // }

    // let geom = all_polys[0].clone();
    // let geoms: Vec<Geometry> = all_polys
    //  //   .clone()
    //     .into_iter()
    //     .map(| x | x.into()).collect();

    // let x = poly_geoms(geom, geoms);  

    // println!("{:?}", x);

    // testing structs 
    let coord1 = geo_types::coord! {
        x: -21.95156,
        y: 64.1446,
    };
    let coord2 = geo_types::coord! {
        x: -21.951,
        y: 64.14479,
    };
    let coord3 = geo_types::coord! {
        x: -21.95044,
        y: 64.14527,
    };
    let coord4 = geo_types::coord! {
        x: -21.951445,
        y: 64.145508,
    };

    let ls = polygon![coord1, coord2, coord3, coord4];
    //let g: Geometry = ls.into();


    let x: Geom = ls.into();

    println!("{}", x);

    let x: Polygon = x.into();

    println!("{:?}", x);


}