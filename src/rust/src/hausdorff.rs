use extendr_api::prelude::*;
use geo::CoordsIter;
use geo_types::*;


use geo::EuclideanDistance;

use geo::GeoFloat;
use geo::Coord;
use num_traits::Bounded; // used to have T as generic type in folding
use num_traits::FloatConst;
use wkt::types::GeometryCollection;

// This is a test implementation of haussdorf distance that requires 
// https://github.com/georust/geo/pull/1029/ to be merged
// it is not complete yet. 
// will create PR to geo when figured out. 
// also a great way to learn how to make a rust macro

// define public trait
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



impl<T> HausdorffDistance<T, Geometry<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Line(rhs) => self.hausdorff_distance(rhs),
            Geometry::Rect(rhs) => self.hausdorff_distance(rhs),
            Geometry::Triangle(rhs) => self.hausdorff_distance(rhs),
            Geometry::Point(rhs) => self.hausdorff_distance(rhs),
            Geometry::MultiPoint(rhs) => self.hausdorff_distance(rhs),
            Geometry::LineString(rhs) => self.hausdorff_distance(rhs),
            Geometry::MultiLineString(rhs) => self.hausdorff_distance(rhs),
            Geometry::Polygon(rhs) => self.hausdorff_distance(rhs),
            Geometry::MultiPolygon(rhs) => self.hausdorff_distance(rhs),
            Geometry::GeometryCollection(rhs) => self.hausdorff_distance(rhs),
        }
    }
}

// geometry and geometry collection 
impl<T> HausdorffDistance<T, GeometryCollection<T>> for Coord<T>
where
    T: GeoFloat + FloatConst,
{
    fn hausdorff_distance(&self, rhs: &GeometryCollection<T>) -> T {
        rhs
            .0
            .iter()
            .map(|c2| self.hausdorff_distance(&c2))
            .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
    }
}


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
    MultiPoint, MultiLineString, MultiPolygon,
    Geometry
);


// ┌────────────────────────────────┐
// │ Implementations for MultiPoint │
// └────────────────────────────────┘

// macro_rules! impl_hausdorff_distance_from_iterables {
//     ($to:ident, [$($from:ident),*]) => {
//         $(
//             impl<T> HausdorffDistance<T, $from<T>> for $to<T>
//             where
//                 T: GeoFloat + FloatConst
//             {
//                 fn hausdorff_distance(&self, geom: &$from<T>) -> T {
//                     self
//                     .iter()
//                     .map(|c| c.euclidean_distance(geom))
//                     .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
//                 }
//             }
//         )*
//     };
// }


macro_rules! impl_hausdorff_distance_for_mpnt {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for MultiPoint<T>
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


// TODO for coord
impl_hausdorff_distance_for_mpnt!([Line, Rect, Triangle, Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, Geometry, GeometryCollection]);

// ┌────────────────────────────────┐
// │ Implementations for LineString │
// └────────────────────────────────┘

// impl<T> HausdorffDistance<T, MultiPoint<T>> for LineString<T> 
// where
// T: GeoFloat + FloatConst,
// {
//     fn hausdorff_distance(&self, rhs: &MultiPoint<T>) -> T {
//         // rhs
//         //     .0
//         //     .iter()
//         //     .map(|c| c.euclidean_distance(self))
//         //     .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
//         self
//             .coords_iter()
//             .map(|c| Point::from(c).euclidean_distance(&rhs))
//             .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))

//     }
// }

macro_rules! impl_hausdorff_distance_for_linestring {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for LineString<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .coords_iter()
                        .map(|c| Point::from(c).euclidean_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))

                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_linestring!([Line, Rect, Triangle, Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon]);


// ┌─────────────────────────────────────┐
// │ Implementations for MultiLineString │
// └─────────────────────────────────────┘

macro_rules! impl_hausdorff_distance_for_multilinestring {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for MultiLineString<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .iter()
                        .map(|l| l.hausdorff_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))

                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_multilinestring!([Line, Rect, Triangle, Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon]);


// ┌─────────────────────────────┐
// │ Implementations for Polygon │
// └─────────────────────────────┘

macro_rules! impl_hausdorff_distance_for_polygon {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for Polygon<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .coords_iter()
                        .map(|c| Point::from(c).euclidean_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))

                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_polygon!([Line, Rect, Triangle, Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon]);

// ┌──────────────────────────────────┐
// │ Implementations for MultiPolygon │
// └──────────────────────────────────┘


macro_rules! impl_hausdorff_distance_for_multipolygon {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for MultiPolygon<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .iter()
                        .map(|p| p.hausdorff_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_multipolygon!([Line, Rect, Triangle, Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon]);



// ┌──────────────────────────┐
// │ Implementations for Line │
// └──────────────────────────┘

macro_rules! impl_hausdorff_distance_for_line {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for Line<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .coords_iter()
                        .map(|p| Point::from(p).euclidean_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_line!([
    Line, Triangle, Rect, 
    Point, MultiPoint,
    LineString, MultiLineString,
    Polygon, MultiPolygon
]);



// ┌──────────────────────────┐
// │ Implementations for Rect │
// └──────────────────────────┘

macro_rules! impl_hausdorff_distance_for_rect {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for Rect<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .coords_iter()
                        .map(|p| Point::from(p).euclidean_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_rect!([
    Line, Triangle, Rect, 
    Point, MultiPoint,
    LineString, MultiLineString,
    Polygon, MultiPolygon
]);


// ┌──────────────────────────────┐
// │ Implementations for Triangle │
// └──────────────────────────────┘

macro_rules! impl_hausdorff_distance_for_tri {
    ([$($from:ident),*]) => {
        $(
            impl<T> HausdorffDistance<T, $from<T>> for Triangle<T>
            where
                T: GeoFloat + FloatConst
            {
                fn hausdorff_distance(&self, geom: &$from<T>) -> T {
                    self
                        .coords_iter()
                        .map(|p| Point::from(p).euclidean_distance(geom))
                        .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
                }
            }
        )*
    };
}

// TODO coord, GeometryCollection, Geometry
impl_hausdorff_distance_for_tri!([
    Line, Triangle, Rect, 
    Point, MultiPoint,
    LineString, MultiLineString,
    Polygon, MultiPolygon
]);



// types that iterate through multiple coords
// line, triangle, rect, linestring, polygon, multipoint
// then multilinestring, muiltipolygon are just the max of each of those

// # we find the minimum distance for each point in x to the point set in y
// # then we take all of the minima and find the maximum
// impl<T> HausdorffDistance<T, MultiPoint<T>> for MultiPoint<T> 
// where
// T: GeoFloat + FloatConst,
// {
//     fn hausdorff_distance(&self, rhs: &MultiPoint<T>) -> T {
//         self
//             .iter()
//             .map(|c| c.euclidean_distance(rhs))
//             .fold(<T as Bounded>::min_value(), |accum, val| accum.max(val))
//     }
// }


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