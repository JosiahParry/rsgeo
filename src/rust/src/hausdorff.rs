use extendr_api::prelude::*;
use geo::CoordsIter;
use geo_types::*;


use geo::EuclideanDistance;

use geo::GeoFloat;
use geo::Coord;

// This is a test implementation of haussdorf distance that requires 
// https://github.com/georust/geo/pull/1029/ to be merged
// it is not complete yet. 
// will create PR to geo when figured out. 
// also a great way to learn how to make a rust macro

pub trait HausdorffDistance<T, Rhs = Self> {
    fn hausdorff_distance(&self, rhs: &Rhs) -> T;
}

// ┌───────────────────────────┐
// │ Implementations for Coord │
// └───────────────────────────┘

// coord to coord
impl<T> HausdorffDistance<T, Coord<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, c: &Coord<T>) -> T {
        self.euclidean_distance(c)
    }
}

// coord to point
impl<T> HausdorffDistance<T, Point<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, c: &Point<T>) -> T {
        self.euclidean_distance(&c.0)
    }
}
// coord to Line
use num_traits::Bounded; // used to have T as generic type in folding
use num_traits::FloatConst;
impl<T> HausdorffDistance<T, Line<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, l: &Line<T>) -> T {
        l
            .coords_iter()
            .map(|p| self.euclidean_distance(&p))
            .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
    }
}

// coord to linestring
impl<T> HausdorffDistance<T, LineString<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, l: &LineString<T>) -> T {
        l
            .coords_iter()
            .map(|p| self.euclidean_distance(&p))
            .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
    }
}

// coord to polygon
impl<T> HausdorffDistance<T, Polygon<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, p: &Polygon<T>) -> T {
        p
            .exterior_coords_iter()
            .map(|p| self.euclidean_distance(&p))
            .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
    }
}


// coord to multi macro
macro_rules! impl_haussdorf_distance_coord_to_multi {
    ($($for:ident),*) => {
        $(
            impl<T> HausdorffDistance<T, $for<T>> for Coord<T>
            where
                T: GeoFloat 
            {
                fn hausdorff_distance(&self, geom: &$for<T>) -> T {
                    geom
                        .coords_iter()
                        .map(|p| self.euclidean_distance(&p))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
                }
            }
        )*
    };
}

// implement for the multi's
impl_haussdorf_distance_coord_to_multi!(MultiPoint, MultiLineString, MultiPolygon, Triangle, Rect);

// ┌───────────────────────────┐
// │ Implementations for Point │
// └───────────────────────────┘
macro_rules! impl_haussdorf_distance_point {
    ($($for:ident),*) => {
        $(
            impl<T> HausdorffDistance<T, $for<T>> for Point<T>
            where
                T: GeoFloat + FloatConst,
            {
                fn hausdorff_distance(&self, geom: &$for<T>) -> T {
                    self.0.hausdorff_distance(geom)
                }
            }
        )*
    };
}

impl_haussdorf_distance_point!(
    Coord, Line, Rect, Triangle,
    Point, LineString, Polygon, 
    MultiPoint, MultiLineString, MultiPolygon
);


macro_rules! impl_hausdorff_distance_from_iterables {
    ($to:ident, [$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for $to<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                    .iter()
                    .map(|c| c.euclidean_distance(geom))
                    .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
                }
            }
        )*
    };
}


// TODO do an implementation for each from and not try to macro it out. 
// its not THAT bad

// impl_hausdorff_distance_from_iterables!(MultiPoint, [Line, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon]);
// impl_hausdorff_distance_from_iterables!(LineString, [Line, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon]);


// types that iterate through multiple coords
// line, triangle, rect, linestring, polygon, multipoint
// then multilinestring, muiltipolygon are just the max of each of those

// # we find the minimum distance for each point in x to the point set in y
// # then we take all of the minima and find the maximum
impl<T> HausdorffDistance<T, MultiPoint<T>> for MultiPoint<T> 
where
T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, rhs: &MultiPoint<T>) -> T {
        self
            .iter()
            .map(|c| c.euclidean_distance(rhs))
            .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
    }
}

impl<T> HausdorffDistance<T, MultiPoint<T>> for LineString<T> 
where
T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, rhs: &MultiPoint<T>) -> T {
        rhs
            .0
            .iter()
            .map(|c| c.euclidean_distance(self))
            .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
    }
}


// impl<T> HausdorffDistance<T, Polygon<T>> for MultiPoint<T> 
// where
// T: GeoFloat + FloatConst,
// {
//     fn hausdorff_distance(&self, rhs: &Polygon<T>) -> T {
//         self
//             .iter()
//             .map(|c| c.euclidean_distance(rhs))
//             .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
//     }
// }


extendr_module! {
    mod hausdorff;
}