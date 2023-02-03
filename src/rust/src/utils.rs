// use crate::types::Geom;
// use extendr_api::prelude::*;
// use geo::BooleanOps;
// use geo_types::geometry::*;
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




// #[extendr]
// fn euclidean_distance_matrix(x: List, y:List) -> RMatrix<f64> {
//     let nr = x.len();
//     let nc = y.len();

//     let xg = from_list(x);
//     let yg = from_list(y);

//     let res_vec = xg.into_iter()
//         .map(|x| yg.to_owned().into_iter()
//             .map(|y| euclidean_distance_impl(x.geom.clone(), &y.geom))
//             .collect::<Vec<f64>>())
//         .collect::<Vec<Vec<f64>>>();
    

//     RMatrix::new_matrix(
//         nr, nc,
//         | r, c | res_vec[r][c]
//     )
// }


