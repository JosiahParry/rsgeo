use sfconversions::Geom;
use extendr_api::prelude::*;

use geo::{HaversineBearing, GeodesicBearing};
use geo_types::Point;



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

// /// Calculate Bearing
// ///
// /// @param x an object of class `point`
// /// @param y for `bearing()` an object of class `point`. For `bearings()` an object of class `rs_POINT`
// ///
// /// @returns
// /// A vector of doubles of the calculated bearing for between x and y
// ///
// /// @export
// #[extendr]
// fn bearing(x: Robj, y: Robj) -> f64 {
//     let x: Geom = x.into();
//     let x: Point = x.into();

//     let y: Geom = y.into();
//     let y: Point = y.into();

//     x.bearing(y)
// }

// #[extendr]
// ///@rdname bearing
// ///@export
// fn bearings(x: Robj, y: List) -> Vec<f64> {
//     let points = from_list(y);

//     let x: Geom = x.into();
//     let x: Point = x.into();

//     points
//         .into_iter()
//         .map(|pnt| x.bearing(pnt.geom.try_into().unwrap()))
//         .collect::<Vec<f64>>()
// }

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

extendr_module! {
    mod query;
    // fn bearing;
    // fn bearings;
    // fn closest_point;
}
