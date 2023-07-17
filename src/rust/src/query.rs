use sfconversions::{Geom, vctrs::geom_class};
use extendr_api::prelude::*;

use geo::{
    HaversineBearing, GeodesicBearing, 
    HaversineClosestPoint, ClosestPoint, Closest,
    IsConvex,
    LineInterpolatePoint, LineLocatePoint
};
use geo_types::{Point, LineString};



// /// Calculate Bearing
// ///
// /// @param x an object of class `point`
// /// @param y for `bearing()` an object of class `point`. For `bearings()` an object of class `rs_POINT`
// ///
// /// @returns
// /// A vector of doubles of the calculated bearing for between x and y
// ///
// /// @export



#[extendr]
fn bearing_haversine(x: List, y: List) -> Doubles {
    let x_cls = x.class().unwrap().next().unwrap();
    let y_cls= y.class().unwrap().next().unwrap();

    if (x_cls != "rs_POINT") || (y_cls != "rs_POINT") {
        panic!("`x` and `y` must be point geometries of class `rs_POINT`");
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let p1: Point = Geom::try_from(xi)
                    .unwrap()
                    .geom
                    .try_into()
                    .unwrap();
                
                let p2: Point = Geom::try_from(yi)
                    .unwrap()
                    .geom
                    .try_into()
                    .unwrap();

                p1.haversine_bearing(p2).into()
            }
        }).collect::<Doubles>()
}


#[extendr]
fn bearing_geodesic(x: List, y: List) -> Doubles {
    let x_cls = x.class().unwrap().next().unwrap();
    let y_cls= y.class().unwrap().next().unwrap();

    if (x_cls != "rs_POINT") || (y_cls != "rs_POINT") {
        panic!("`x` and `y` must be point geometries of class `rs_POINT`");
    }

    x
        .iter()
        .zip(y.iter())
        .map(|((_, xi), (_, yi))| {
            if xi.is_null() || yi.is_null() {
                Rfloat::na()
            } else {
                let p1: Point = Geom::try_from(xi)
                    .unwrap()
                    .geom
                    .try_into()
                    .unwrap();
                
                let p2: Point = Geom::try_from(yi)
                    .unwrap()
                    .geom
                    .try_into()
                    .unwrap();

                p1.geodesic_bearing(p2).into()
            }
        }).collect::<Doubles>()
}



// #[extendr]
// /// Find the closest point
// ///
// /// @param x a single geometry
// /// @param y a `point`
// ///
// /// @export
// fn closest_point(x: Robj, y: Robj) -> Robj {
//     let res = Geom::from(x)
//         .geom
//         .closest_point(&Geom::from(y).geom.try_into().unwrap());

//     match res {
//         Closest::SinglePoint(res) => to_pntr(Geom::from(res)),
//         Closest::Intersection(res) => to_pntr(Geom::from(res)),
//         // id like a better approach here
//         Closest::Indeterminate => Robj::from(extendr_api::NA_LOGICAL),
//     }
// }

#[extendr]
fn closest_point(x: List, y: List) -> Robj {
    // check that y is a point
    let y_cls= y.class().unwrap().next().unwrap();
    if y_cls != "rs_POINT" {
        panic!("`y` must be point geometries of class `rs_POINT`");
    }

    x
    .iter()
    .zip(y.iter())
    .map(|((_, xi), (_, yi))| {
        if xi.is_null() || yi.is_null() {
            NULL.into_robj()
        } else {
            let p: Point = Geom::try_from(yi)
                .unwrap()
                .geom
                .try_into()
                .unwrap();
            

            let closest = Geom::try_from(xi)
                .unwrap()
                .geom
                .closest_point(&p);

            match closest {
                Closest::SinglePoint(pnt) => Geom::from(pnt).into(),
                Closest::Intersection(pnt) => Geom::from(pnt).into(),
                Closest::Indeterminate => NULL.into_robj()
            }
        }
    }).collect::<List>()
    .set_class(sfconversions::vctrs::geom_class("point"))
    .unwrap()

}

#[extendr]
fn closest_point_haversine(x: List, y: List) -> Robj {
    // check that y is a point
    let y_cls= y.class().unwrap().next().unwrap();
    if y_cls != "rs_POINT" {
        panic!("`y` must be point geometries of class `rs_POINT`");
    }

    x
    .iter()
    .zip(y.iter())
    .map(|((_, xi), (_, yi))| {
        if xi.is_null() || yi.is_null() {
            NULL.into_robj()
        } else {
            let p: Point = Geom::try_from(yi)
                .unwrap()
                .geom
                .try_into()
                .unwrap();
            

            let closest = Geom::try_from(xi)
                .unwrap()
                .geom
                .haversine_closest_point(&p);

            match closest {
                Closest::SinglePoint(pnt) => Geom::from(pnt).into(),
                Closest::Intersection(pnt) => Geom::from(pnt).into(),
                Closest::Indeterminate => NULL.into_robj()
            }
        }
    }).collect::<List>()
    .set_class(sfconversions::vctrs::geom_class("point"))
    .unwrap()

}


#[extendr]
fn is_convex(x: List) -> Logicals {
    // check that y is a point
    let x_cls= x.class().unwrap().next().unwrap();
    if x_cls != "rs_LINESTRING" {
        panic!("`y` must be LineString geometries of class `rs_LINESTRING`");
    }

    x
        .iter()
        .map(|(_, xi)| {
            if xi.is_null() {
                Rbool::na()
            } else {
                LineString::try_from(
                    Geom::try_from(xi).unwrap()
                ).unwrap()
                .is_convex()
                .into()
            }
        }).collect::<Logicals>()
}

#[extendr]
fn line_interpolate_point(x: List, fraction: Doubles) -> Robj {
    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be a `rs_LINESTRING`")
    }

    x
        .iter()
        .zip(fraction.into_iter())
        .map(|((_, xi), fi)| {

            if xi.is_null() || fi.is_na() || fi.is_infinite() || fi.is_nan() {
                NULL.into_robj()
            } else {
                let l: LineString = Geom::try_from(xi)
                    .unwrap()
                    .try_into()
                    .unwrap();

                let res = l.line_interpolate_point(fi.inner());

                match res {
                    Some(res) => Geom::from(res).into(),
                    None => NULL.into_robj()
                }
            }

        }).collect::<List>()
        .set_class(geom_class("point"))
        .unwrap()

}

#[extendr]
fn locate_point_on_line(x: List, y: List) -> Doubles {

    if !x.inherits("rs_LINESTRING") {
        panic!("`x` must be an `rs_LINESTRING`")
    } else if !y.inherits("rs_POINT") {
        panic!("`y` must be an `rs_POINT")
    }

    x
    .iter()
    .zip(y.iter())
    .map(|((_, xi), (_, yi))| {
        if xi.is_null() || yi.is_null() {
            Rfloat::na()
        } else {

            let l: LineString = Geom::try_from(xi)
                .unwrap()
                .geom
                .try_into()
                .unwrap();

            let p: Point = Geom::try_from(yi)
                .unwrap()
                .geom
                .try_into()
                .unwrap();
            
            l
                .line_locate_point(&p)
                .into()

        }
    }).collect::<Doubles>()


}

extendr_module! {
    mod query;
    fn bearing_geodesic;
    fn bearing_haversine;
    fn closest_point;
    fn closest_point_haversine;
    fn is_convex;
    fn line_interpolate_point;
    fn locate_point_on_line;
}
