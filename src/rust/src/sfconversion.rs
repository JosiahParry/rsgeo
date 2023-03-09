use extendr_api::prelude::*;
use crate::types::Geom;
use crate::mat_to_rs; 
use crate::matrix_to_coords;
use crate::geoms::polygon_inner;
use geo_types::{Geometry, Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon};
use ndarray::Axis;

pub fn sfg_to_geometry(x: Robj) -> Geom {
    
    let cls = x.class().unwrap().next().unwrap();
    
    match cls {
        "POINT" => {
            let x = Doubles::try_from(x).unwrap();
            Geom::from(Point::new(x[0].0, x[1].0))
        },
        "MULTIPOINT" => {
            let x = RMatrix::from_robj(&x).unwrap();
            let arr = mat_to_rs(x);

            let mpnt = MultiPoint::new(
                arr.axis_iter(Axis(0))
                    .map(|x| Point::new(x[0], x[1]))
                    .collect::<Vec<Point>>(),
            );

            Geom::from(mpnt)
        },
        "LINESTRING" => {
            let x = RMatrix::from_robj(&x).unwrap();
            let coords = matrix_to_coords(x);
            let lns = LineString::new(coords);
            Geom::from(lns)
        },

        "MULTILINESTRING" => {
            let x = List::try_from(x).unwrap();
            let vec_lns = x
            .into_iter()
            .map(|(_, x)| LineString::new(matrix_to_coords(RMatrix::try_from(x).unwrap())))
            .collect::<Vec<LineString>>();
    
            Geom::from(MultiLineString::new(vec_lns))
        },
        "POLYGON" => {
            let x = List::try_from(x).unwrap();
            let n = x.len();
            let mut linestrings: Vec<LineString> = Vec::with_capacity(n);
        
            let exterior = matrix_to_coords(x[0].as_matrix().unwrap());
            let exterior = LineString::new(exterior);
        
            if n > 1 {
                for i in 1..n {
                    let xi: RMatrix<f64> = x[i].to_owned().try_into().unwrap();
                    let coords = matrix_to_coords(xi);
                    let line = LineString::new(coords);
                    linestrings.push(line);
                }
            }
        
            let polygon = Polygon::new(exterior, linestrings);
            polygon.into()  
        },

        "MULTIPOLYGON" => {
            let x = List::try_from(x).unwrap();
            let res = MultiPolygon::new(
                x.into_iter()
                    .map(|(_, x)| polygon_inner(List::try_from(x).unwrap()))
                    .collect::<Vec<Polygon>>(),
            );
        
            res.into()
        },

        &_ => Geom::from(Point::new(0.0, 0.0))
    }
}

extendr_module! {
    mod sfconversion;
}