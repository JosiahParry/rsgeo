use crate::types::Geom;
use extendr_api::prelude::*;
use geo::BooleanOps;
use geo_types::geometry::*;
//use extendr_api::wrapper::{};

// pub trait Geometrize {
//     //https://www.dictionary.com/browse/geometrize
//     fn as_geom(self) -> Geom; 
//     //fn as_points_from_mat(self) -> Vec<Geom>;
// }


// impl Geometrize for RMatrix<f64>  {
//     fn as_points_from_mat(self) -> Vec<Geom> {
//         //crate::geoms::geom_points_matrix(self)
//     }
// }

// extendr_module! {
//     mod utils;
// }

// pub trait Aggregate {
//     fn aggregate(self) -> ();
// }

// impl Aggregate for Vec<Geometry> {
//     fn aggregate(self) -> () {
//         let x = self; 
//         match x {
//             Vec<Point> => MultiPoint::new(x)
//         };
//     }
// }


