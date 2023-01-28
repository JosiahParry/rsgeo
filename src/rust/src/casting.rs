use extendr_api::prelude::*;
use geo::{
    MultiPolygon, Polygon,
    MultiPoint, Point,
    MultiLineString, LineString, 
    RemoveRepeatedPoints, BooleanOps,
    Geometry
};

use crate::geoms::from_list;

#[extendr]
fn get_base_vec(x: List) {

    let geom_type = x.class().unwrap();
    let x = from_list(x);
    let x: Vec<Geometry> = x.into_iter().map(|x| x.geom).collect();

    //match geom_type
    let geo_type: String = geom_type.into_iter()
        .filter(|cls| cls.contains("rs_"))
        .collect();
    
    rprintln!("{geo_type}");
    
}

//#[extendr]
fn union_polygon(x: Vec<Polygon> ) -> MultiPolygon {
    //let y= x.into_iter()
    //    .reduce(|target, next| target.add_po(&next));

    x.into_iter()
        .map(|x| MultiPolygon::try_from(x).unwrap())
        .reduce(|init, nxt | init.union(&nxt))
        .unwrap()

}


fn union_multipolygon(x: Vec<MultiPolygon>) -> MultiPolygon {
    x.into_iter()
        .reduce(|init, nxt| init.union(&nxt))
        .unwrap()
}

fn union_point(x: Vec<Point>) -> MultiPoint {

    let res: MultiPoint = x.try_into().unwrap();

    // remove repeated points (similar to sf::st_union())
    res.remove_repeated_points()
}

fn union_multipoint(x: Vec<MultiPoint>) -> MultiPoint {

    let point_vec =  x.into_iter()
    .flat_map(|x| x.into_iter()
            .map(|xi| xi)
        ).collect::<Vec<Point>>();

    MultiPoint::new(point_vec).remove_repeated_points()
    
}


fn union_linestring(x: Vec<LineString>) -> MultiLineString {

    let res = MultiLineString::new(x);

    // remove repeated points (similar to sf::st_union())
    res.remove_repeated_points()
}

fn union_multilinestring(x: Vec<MultiLineString>) -> MultiLineString {

   let line_vecs =  x.into_iter()
        .flat_map(|x| x.into_iter()
                .map(|xi| xi)
            ).collect::<Vec<LineString>>();
    
    MultiLineString::new(line_vecs).remove_repeated_points()
}



extendr_module! {
    mod casting;
    fn get_base_vec;
 //   fn union_polygon;
}