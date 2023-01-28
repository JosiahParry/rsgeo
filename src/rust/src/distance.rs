use crate::types::Geom;
use extendr_api::prelude::*;
use extendr_api::Robj;
use crate::geoms::from_list;

use geo_types::{
    Geometry, Point, Polygon, LineString, MultiLineString, MultiPoint, MultiPolygon
};
use geo::{EuclideanDistance, HaversineDistance, GeodesicDistance, VincentyDistance};


#[extendr]
fn euclidean_distance_matrix(x: List, y:List) -> RMatrix<f64> {
    let nr = x.len();
    let nc = y.len();

    let xg = from_list(x);
    let yg = from_list(y);

    let res_vec = xg.into_iter()
        .map(|x| yg.to_owned().into_iter()
            .map(|y| euclidean_distance_impl(x.geom.clone(), &y.geom))
            .collect::<Vec<f64>>())
        .collect::<Vec<Vec<f64>>>();
    

    RMatrix::new_matrix(
        nr, nc,
        | r, c | res_vec[r][c]
    )

}

fn euclidean_distance_impl(x: Geometry, y: &Geometry) -> f64 {


    match x {
        Geometry::Point(x) => e_dist_pnt(x, y),
        Geometry::MultiPoint(x) => e_dist_mpnt(x, y),
        Geometry::LineString(x) => e_dist_linestring(x, y),
        Geometry::MultiLineString(x) => e_dist_mlinestring(x, y),
        Geometry::Polygon(x) => e_dist_poly(x, y),
        Geometry::MultiPolygon(x) => e_dist_mpoly(x, y),
        _ => 0.,
    }

}

#[extendr]
fn euclidean_distance_pairwise(x: List, y: List) -> Vec<f64> {

    x.into_iter()
        .enumerate()
        .map(|(i, x)| euclidean_distance(x.1, y[i].to_owned()))
        .collect::<Vec<f64>>()

}

// TODO have functions return Rfloat object, 
// underlying functions can return NA instead of 0 for types without the impl
#[extendr]
fn euclidean_distances(x: Robj, y: List) -> Vec<f64> {

    y.into_iter()
        .map(| (_, y) | euclidean_distance(x.clone(), y))
        .collect::<Vec<f64>>()
}

#[extendr]
fn euclidean_distance(x: Robj, y: Robj) -> f64 {

    let x: Geom = x.into(); 
    let y: Geom = y.into(); 

    let y = &y.geom;
    let x = x.geom;

    match x {
        Geometry::Point(x) => e_dist_pnt(x, y),
        Geometry::MultiPoint(x) => e_dist_mpnt(x, y),
        Geometry::LineString(x) => e_dist_linestring(x, y),
        Geometry::MultiLineString(x) => e_dist_mlinestring(x, y),
        Geometry::Polygon(x) => e_dist_poly(x, y),
        Geometry::MultiPolygon(x) => e_dist_mpoly(x, y),
        _ => 0.,
    }

}

fn e_dist_pnt(x: Point, y: &Geometry) -> f64 {
    match y {
        Geometry::LineString(y) => x.euclidean_distance(y),
        Geometry::MultiLineString(y) => x.euclidean_distance(y),
        Geometry::Point(y) => x.euclidean_distance(y),
        Geometry::MultiPoint(y) => x.euclidean_distance(y),
        Geometry::Polygon(y) => x.euclidean_distance(y),
        Geometry::MultiPolygon(y) => x.euclidean_distance(y),
        Geometry::Line(y) => x.euclidean_distance(y),
        _ => 0.0
    }
}

fn e_dist_mpnt(x: MultiPoint, y: &Geometry) -> f64 {
    match y {
        Geometry::Point(y) => x.euclidean_distance(y),
        _ => 0.0
    }
}

fn e_dist_linestring(x: LineString, y: &Geometry) -> f64 {
    match y {
        Geometry::LineString(y) => x.euclidean_distance(y),
        Geometry::Point(y) => x.euclidean_distance(y),
        Geometry::Polygon(y) => x.euclidean_distance(y),
        Geometry::Line(y) => x.euclidean_distance(y),
        _ => 0.0
    }
}

fn e_dist_mlinestring(x: MultiLineString, y: &Geometry) -> f64 {
    match y {
        Geometry::Point(y) => x.euclidean_distance(y),
        _ => 0.0
    }
}

fn e_dist_poly(x: Polygon, y: &Geometry) -> f64 {
    match y {
        Geometry::LineString(y) => x.euclidean_distance(y),
        //Geometry::MultiLineString(y) => x.euclidean_distance(y),
        Geometry::Point(y) => x.euclidean_distance(y),
        //Geometry::MultiPoint(y) => x.euclidean_distance(y),
        Geometry::Polygon(y) => x.euclidean_distance(y),
        //Geometry::MultiPolygon(y) => x.euclidean_distance(y),
        Geometry::Line(y) => x.euclidean_distance(y),
        _ => 0.0
    }
}

fn e_dist_mpoly(x: MultiPolygon, y: &Geometry) -> f64 {
    match y {
        Geometry::Point(y) => x.euclidean_distance(y),
        _ => 0.0
    }
}



//// Haversine distance 
#[extendr]
fn haversine_distances(x: Robj, y: List) -> Vec<f64> {

    let x: Geom = x.try_into().unwrap();
    let x: Point = x.try_into().unwrap();
    let y = from_list(y);

    y.into_iter()
        .map(| y | Point::try_from(y.geom).unwrap())
        .map(|pnt| x.haversine_distance(&pnt))
        .collect::<Vec<f64>>()
}

#[extendr]
fn haversine_distance(x: Robj, y: Robj) -> f64 {
    let x: Geom = x.into(); 
    let y: Geom = y.into(); 

    let x: Point = x.geom.try_into().unwrap();
    let y: Point = y.geom.try_into().unwrap();

    x.haversine_distance(&y)

}

#[extendr]
fn haversine_distance_matrix(x: List, y: List) -> RMatrix<f64> {
    let nr = x.len();
    let nc = y.len();

    let xg = from_list(x);
    let yg = from_list(y);

    let res_vec = xg.into_iter()
        .map(|x| Point::try_from(x).unwrap())
        .map(|x| 
            yg.to_owned().into_iter()
                .map(|y| Point::try_from(y).unwrap())
                .map(|y| x.haversine_distance(&y))
                .collect::<Vec<f64>>()
        )
        .collect::<Vec<Vec<f64>>>();
    
        RMatrix::new_matrix(
            nr, nc,
            | r, c | res_vec[r][c]
        )
}


//// Geodesic distance 
#[extendr]
fn geodesic_distances(x: Robj, y: List) -> Vec<f64> {

    let x: Geom = x.try_into().unwrap();
    let x: Point = x.try_into().unwrap();
    let y = from_list(y);

    y.into_iter()
        .map(| y | Point::try_from(y.geom).unwrap())
        .map(|pnt| x.geodesic_distance(&pnt))
        .collect::<Vec<f64>>()
}

#[extendr]
fn geodesic_distance(x: Robj, y: Robj) -> f64 {
    let x: Geom = x.into(); 
    let y: Geom = y.into(); 

    let x: Point = x.geom.try_into().unwrap();
    let y: Point = y.geom.try_into().unwrap();

    x.geodesic_distance(&y)

}

#[extendr]
fn geodesic_distance_matrix(x: List, y: List) -> RMatrix<f64> {
    let nr = x.len();
    let nc = y.len();

    let xg = from_list(x);
    let yg = from_list(y);

    let res_vec = xg.into_iter()
        .map(|x| Point::try_from(x).unwrap())
        .map(|x| 
            yg.to_owned().into_iter()
                .map(|y| Point::try_from(y).unwrap())
                .map(|y| x.geodesic_distance(&y))
                .collect::<Vec<f64>>()
        )
        .collect::<Vec<Vec<f64>>>();
    
        RMatrix::new_matrix(
            nr, nc,
            | r, c | res_vec[r][c]
        )
}


//// Haversine distance 
#[extendr]
fn vincenty_distances(x: Robj, y: List) -> Vec<f64> {

    let x: Geom = x.try_into().unwrap();
    let x: Point = x.try_into().unwrap();
    let y = from_list(y);

    y.into_iter()
        .map(| y | Point::try_from(y.geom).unwrap())
        .map(|pnt| x.vincenty_distance(&pnt).unwrap())
        .collect::<Vec<f64>>()
}

#[extendr]
fn vincenty_distance(x: Robj, y: Robj) -> f64 {
    let x: Geom = x.into(); 
    let y: Geom = y.into(); 

    let x: Point = x.geom.try_into().unwrap();
    let y: Point = y.geom.try_into().unwrap();

    x.vincenty_distance(&y).unwrap()

}

#[extendr]
fn vincenty_distance_matrix(x: List, y: List) -> RMatrix<f64> {
    let nr = x.len();
    let nc = y.len();

    let xg = from_list(x);
    let yg = from_list(y);

    let res_vec = xg.into_iter()
        .map(|x| Point::try_from(x).unwrap())
        .map(|x| 
            yg.to_owned().into_iter()
                .map(|y| Point::try_from(y).unwrap())
                .map(|y| x.vincenty_distance(&y).unwrap())
                .collect::<Vec<f64>>()
        )
        .collect::<Vec<Vec<f64>>>();
    
        RMatrix::new_matrix(
            nr, nc,
            | r, c | res_vec[r][c]
        )
}

// Exporting   
extendr_module! {
    mod distance;
    fn euclidean_distance;
    fn euclidean_distances;
    fn euclidean_distance_pairwise;
    fn euclidean_distance_matrix;
    fn haversine_distance;
    fn haversine_distances;
    fn haversine_distance_matrix; 
    fn geodesic_distance;
    fn geodesic_distances;
    fn geodesic_distance_matrix;
    fn vincenty_distance;
    fn vincenty_distances;
    fn vincenty_distance_matrix;
    // fn geodesic_distance;
    // fn haversine_distance;
    // fn vicenty_distance;
}