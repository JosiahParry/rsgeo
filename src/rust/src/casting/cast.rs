use extendr_api::prelude::*;
use geo::CoordsIter;
use geo_types::{MultiPolygon, MultiPoint, MultiLineString, Polygon, LineString, Point};
use crate::types::Geom;
use crate::to_pntr;
use crate::geoms::from_list;



//# cast 1 : 1 
//# expand 1 : many
//# combine many : 1 


// CASTING ------------------------------------------------------------------------
//                      to_point to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// from_point               TRUE          TRUE      FALSE           FALSE         FALSE              FALSE
// from_multipoint            NA          TRUE       TRUE            TRUE          TRUE               TRUE
// from_polygon               NA          TRUE       TRUE            TRUE          TRUE               TRUE
// from_multipolygon          NA          TRUE         NA            TRUE            NA               TRUE
// from_linestring            NA          TRUE      FALSE           FALSE          TRUE               TRUE
// from_multilinestring       NA          TRUE      FALSE           FALSE            NA               TRUE


// from_point               TRUE          TRUE      FALSE           FALSE         FALSE              FALSE
#[extendr]
fn cast_point_point(x: Robj) -> Robj {
    x
}

#[extendr]
fn cast_point_multipoint(x: Robj) -> Robj {
    let res = MultiPoint::new(vec![Point::try_from(Geom::from(x).geom).unwrap()]);
    to_pntr(res.into())
}

// from_multipoint            NA          TRUE       TRUE            TRUE          TRUE               TRUE
#[extendr]
fn cast_multipoint_multipoint(x: Robj) -> Robj {
    x
}

#[extendr]
fn cast_multipoint_polygon(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let mut crds = x.0;
    crds.push(crds[0]);
    let ln = LineString::from(crds);
    to_pntr(Geom::from(Polygon::new(ln, vec![])))
}

#[extendr]
fn cast_multipoint_multipolygon(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let mut crds = x.0;
    crds.push(crds[0]);
    let ln = LineString::from(crds);
    to_pntr(Geom::from(MultiPolygon::new(vec![Polygon::new(ln, vec![])])))
}

#[extendr]
fn cast_multipoint_linestring(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let crds = x.0;
    let ln = LineString::from(crds);
    to_pntr(Geom::from(ln))
}


#[extendr]
fn cast_multipoint_multilinestring(x: Robj) -> Robj {
    let x = MultiPoint::try_from(Geom::from(x).geom).unwrap();
    let crds = x.0;
    let ln = LineString::from(crds);
    to_pntr(Geom::from(MultiLineString::new(vec![ln])))
}

//                      to_point to_multipoint to_polygon to_multipolygon to_linestring to_multilinestring
// from_polygon               NA          TRUE       TRUE            TRUE          TRUE               TRUE
#[extendr]
fn cast_polygon_multipoint(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();
    let pnts = x
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::new(pnts)))

}

#[extendr]
fn cast_polygon_polygon(x: Robj) -> Robj {
    x
}

#[extendr]
fn cast_polygon_multipolygon(x: Robj) -> Robj {
    to_pntr(Geom::from(MultiPolygon::try_from(Geom::from(x).geom).unwrap()))
}


#[extendr]
fn cast_polygon_linestring(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();
    let pnts = x
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(LineString::from(pnts)))
}

#[extendr]
fn cast_polygon_multilinestring(x: Robj) -> Robj {
    let x = Polygon::try_from(Geom::from(x).geom).unwrap();
    let (interrior, holes) = x.into_inner();
    let mut interrior = vec![interrior];
    interrior.extend(holes.into_iter());
    
    to_pntr(Geom::from(MultiLineString::new(interrior)))
}

//                       to_multipoint  to_multipolygon  to_multilinestring
// from_multipolygon              TRUE             TRUE                TRUE
#[extendr]
fn cast_multipolygon_multipoint(x: Robj) -> Robj {
    let mply = MultiPolygon::try_from(Geom::from(x).geom).unwrap();

    let pnts = mply
        .coords_iter()
        .map(|x| Point::from(x))
        .collect::<Vec<Point>>();

    let res = MultiPoint::new(pnts);
    to_pntr(Geom::from(res))
}

#[extendr]
fn cast_multipolygon_multilinestring(x: Robj) -> Robj {
    let mply = MultiPolygon::try_from(Geom::from(x).geom).unwrap();
    let linestrings = mply.0
        .into_iter()
        .map(|x| LineString::from_iter(x.coords_iter()))
        .collect::<Vec<LineString>>();

    to_pntr(Geom::from(MultiLineString::new(linestrings)))
}

#[extendr]
fn cast_multipolygon_multipolygon(x: Robj) -> Robj { x }

//                      to_multipoint to_linestring to_multilinestring
// from_linestring               TRUE          TRUE               TRUE
#[extendr]
fn cast_linestring_multipoint(x: Robj) -> Robj {
    to_pntr(
        Geom::from(
            LineString::try_from(Geom::from(x).geom).unwrap()
                .coords_iter()
                .collect::<MultiPoint>()
            )
        )
}

#[extendr]
fn cast_linestring_linestring(x: Robj) -> Robj { x }

#[extendr]
fn cast_linestring_multilinestring(x: Robj) -> Robj {
    to_pntr(Geom::from(MultiLineString::new(vec![LineString::try_from(Geom::from(x).geom).unwrap()])))
}

//                      to_multipoint to_multilinestring
// from_multilinestring          TRUE               TRUE
#[extendr]
fn cast_multilinestring_multipoint(x: Robj) -> Robj {
    let res: Geom = MultiPoint::from_iter(MultiLineString::try_from(
        Geom::from(x)
        .geom
    )
    .unwrap()
    .coords_iter())
    .into();

    to_pntr(res)
    
}

#[extendr]
fn cast_multilinestring_multilinestring(x: Robj) -> Robj { x }

// COMBINE ------------------------------------------------------------------------

// building primitives up
// vec points -> Line
// vec points -> multipoint
#[extendr]
fn combine_points_line(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| Point::try_from(x.geom).unwrap())
        .collect::<Vec<Point>>();

    to_pntr(Geom::from(LineString::from(x)))
}

#[extendr]
fn combine_points_multipoint(x: List) -> Robj {
    let x = from_list(x)
    .into_iter()
    .map(|x| Point::try_from(x.geom).unwrap())
    .collect::<Vec<Point>>();

    to_pntr(Geom::from(MultiPoint::from(x)))
}

// vec lines -> polygon
// vec lines -> multiline
#[extendr]
fn combine_lines_polygon(x: List) -> Robj {
    let x = from_list(x)
        .into_iter()
        .map(|x| LineString::try_from(x.geom).unwrap())
        .collect::<Vec<LineString>>();

        // shoot do i need to handle winding here? 
        // assuming the user has the widning correct here
    let n = x.len();
    let exterior = x[0].clone();
    let interior = if n > 1 {
        x[1..n].to_owned()
    } else {
        vec![]
    };

    to_pntr(Geom::from(Polygon::new(exterior, interior)))
}

// #[extendr]
// fn cast_lines_multilinestring(x: List) -> Robj {
//     let x = from_list(x)
//         .into_iter()
//         .map(|x| LineString::try_from(x.geom).unwrap())
//         .collect::<Vec<LineString>>();

//         to_pntr(Geom::from(MultiLineString::new(x)))
// }

// #[extendr]
// fn cast_multiline_polygon(x: Robj) -> Robj {
//     let x = MultiLineString::try_from(Geom::from(x).geom).unwrap().0;
    
//     let n = x.len();
//     let exterior = x[0].clone();
//     let interior = if n > 1 {
//         x[1..n].to_owned()
//     } else {
//         vec![]
//     };

//     to_pntr(Geom::from(Polygon::new(exterior, interior)))
// }

// multiline -> polygon
// polygons -> multipolygon


extendr_module! {
    mod cast;
    fn cast_point_point; // point
    fn cast_point_multipoint;
    fn cast_multipoint_multipoint; // multipoint
    fn cast_multipoint_linestring;
    fn cast_multipoint_multilinestring;
    fn cast_multipoint_polygon;
    fn cast_multipoint_multipolygon;
    fn cast_linestring_linestring; // linestring
    fn cast_linestring_multilinestring;
    fn cast_linestring_multipoint;
    fn cast_multilinestring_multilinestring; // multilinestring
    fn cast_multilinestring_multipoint;
    fn cast_polygon_polygon; // polygon
    fn cast_polygon_multipolygon;
    fn cast_polygon_multipoint;
    fn cast_polygon_linestring;
    fn cast_polygon_multilinestring;
    fn cast_multipolygon_multipolygon; // multipolygon 
    fn cast_multipolygon_multilinestring;
    fn cast_multipolygon_multipoint;

}