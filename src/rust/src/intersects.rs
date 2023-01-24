//mod spatialindex;
//use crate::spatialindex::*;
use geo::{Intersects};
use geo_types::{Polygon, Geometry, Point};


// need formats of 
//    single:many
//       - should be niave approach?
//    many:many
//    {polygon}:{polygons}
//    {point}:{points}


// note these are 1 indexed
// polygon to polygons
pub fn poly_polys(x: Polygon, y: Vec<Polygon>) -> Vec<usize> {

   let res: Vec<usize> =  y.into_iter().
        enumerate().
        filter(| (_, geom) | 
        x.intersects(geom) 
    ).map(| (index, _) | index + 1 ).collect();

    res    
}


pub fn poly_geoms(x: Polygon, y: Vec<Geometry>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(y.len());
    for (index, geom) in y.into_iter().enumerate() {
        let hit = x.intersects(&geom);
        if hit { res.push(index)}
    }
    res
 }


 pub fn point_geoms(x: Point, y: Vec<Geometry>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::with_capacity(y.len());
    for (index, geom) in y.into_iter().enumerate() {
        let hit = x.intersects(&geom);
        if hit { res.push(index)}
    }
    res
 }
