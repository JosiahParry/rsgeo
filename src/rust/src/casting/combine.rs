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
