use core::ops::Range;
use extendr_api::prelude::*;
use geo::{BoundingRect, EuclideanDistance};
use geo_types::{LineString, Rect};
use rstar::primitives::{CachedEnvelope, GeomWithData};
use rstar::{RTree, RTreeObject, AABB};
use sfconversions::{Geom, IntoGeom};
use std::collections::BTreeMap;

fn create_line_segment_tree(x: List) -> RTree<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>> {
    // for each geometry in the vector x
    let x_segments = x
        .iter()
        // we get the index position
        .enumerate()
        .filter_map(|(i, (_, xi))| {
            // if missing skip it
            if xi.is_null() {
                None
            } else {
                // get the Geom struct
                let geo = Geom::try_from(xi).unwrap();
                // get the linestring -- will panic if not a linstring
                let lines = LineString::try_from(geo.geom)
                    .unwrap()
                    // iterate through each component linestring
                    .lines()
                    // for each component linestring we calculate the slope, create a bounding box
                    // and createa `GeomWithData` that stores the index position and slope
                    // we don't want to calculate the slope multiple times for the same
                    // geometry so this will store it
                    .map(|li| {
                        let slope = li.slope();
                        let env = li.cached_envelope();
                        GeomWithData::new(env, (i, slope))
                    })
                    .collect::<Vec<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>>>();

                // return an option here so null geomerties are excluded from the tree
                Some(lines)
            }
        })
        .flatten()
        .collect::<Vec<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>>>();

    // insert into a new tree
    RTree::bulk_load(x_segments)
}

fn x_range(rect: &Rect) -> Range<f64> {
    rect.min().x..rect.max().x
}

fn y_range(rect: &Rect) -> Range<f64> {
    rect.min().y..rect.max().y
}

fn range_contains(r1: Range<f64>, r2: Range<f64>) -> bool {
    r1.contains(&r2.start) || r1.contains(&r2.end)
}


// Given the overlap in the domain and range
// we can calculate the segment lenth of the line that is provided
// we use the bounding box as a &Rect to determine the width or height
// of the triangle
fn solve_segment_length(
    x_overlap: Option<Range<f64>>, 
    y_overlap: Option<Range<f64>>, 
    bbox: &Rect
) -> f64 {
    if x_overlap.is_some() && y_overlap.is_some() {
        let (base_w, base_h) = wh(&bbox);
        let dy = solve_dy(y_overlap.unwrap());
        let dx = solve_dx(dy, base_w, base_h);
        solve_h(dx, dy)
    } else if x_overlap.is_some() {
        let x_over = x_overlap.unwrap();
        x_over.end - x_over.start 
    } else if y_overlap.is_some() {
        let y_over = y_overlap.unwrap();
        y_over.end - y_over.start
    } else {
        unreachable!() // this should never happen 
    }
} 


// get height and width from a Line
// do this by passing in the bounding rectangle
// (width, height)
fn wh(x: &Rect) -> (f64, f64) {
    let (x1, y1) = x.min().x_y();
    let (x2, y2) = x.max().x_y();
    (x2 - x1, y2 - y1)
}

// Solve for dy:
// This is the height of the range of Y values
fn solve_dy(y_range: Range<f64>) -> f64 {
    y_range.end - y_range.start
}

// base_w is dx2 (the )
fn solve_dx(dy: f64, base_w: f64, base_h: f64) -> f64 {
    dy * base_w / base_h
}

fn solve_h(dx: f64, dy: f64) -> f64 {
    (dx.powi(2)+ dy.powi(2)).sqrt()
}


fn identify_candidates(
    x: RTree<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>>,
    y: Vec<LineString>,
    dist: f64,
    slope_tolerance: f64,
) -> BTreeMap<i32, Vec<i32>> {
    let half_d = dist / 2.0;
    let mut matches: BTreeMap<i32, Vec<i32>> = BTreeMap::new();

    let _ = y.into_iter().enumerate().for_each(|(j, lns)| {
        lns.lines().for_each(|li| {
            // increase the size of our bounding rectangle to ensure that our padding distance
            // is respected.
            let bb = li.bounding_rect();
            let (ll_x, ll_y) = bb.min().x_y();
            let (ur_x, ur_y) = bb.max().x_y();

            // create the AABB
            let aabb = AABB::from_corners(
                [ll_x - half_d, ll_y - half_d],
                [ur_x + half_d, ur_y + half_d],
            );

            // get the intersection candidates based on the AABB
            let cands = x.locate_in_envelope_intersecting(&aabb);

            // for each possible candidate
            cands.for_each(|ci| {
                let (i, slope) = ci.data;
                // check if the they are within a tolerance

                // is the difference between slopes within our pre-defined tolerance?
                let is_tolerant = (slope - li.slope()).abs() < slope_tolerance;

                // if the slopes are sufficiently similar we can check if they're close enough
                if is_tolerant {
                    // next we check to see if there is overlap in the domain and range
                    let bbox_1 = ci.geom().geom.bounding_rect().unwrap();
                    let bbox_2 = li.bounding_rect();

                    // if theres overlap then we do a distance based check
                    if range_contains(x_range(&bbox_2), x_range(&bbox_1))
                        || range_contains(y_range(&bbox_2), y_range(&bbox_1))
                    {
                        // calculate the distance from the line segment
                        // if its within our threshold we include it;
                        let d = ci.geom().geom.euclidean_distance(&li);
                        // if distance is less than or equal to tolerance, add the key
                        if d <= dist {
                            // add 1 for R indexing
                            // ensures that no duplicates are inserted. Creates a new empty vector is needed
                            let entry = matches.entry((i + 1) as i32).or_insert_with(Vec::new);
                            if !entry.contains(&((j + 1) as i32)) {
                                entry.push((j + 1) as i32);
                            }
                        }
                    }
                }
            });
        })
    });

    matches
}

#[extendr]
fn rnet_merge(x: List, y: List, dist: f64, slope_tolerance: f64) -> List {
    let x_tree = create_line_segment_tree(x);
    // from rsgeo vector create vector of linestrings. will not handle missingvalues
    let y = y
        .into_iter()
        .map(|(_, lns)| LineString::try_from(Geom::from(lns).geom).unwrap())
        .collect::<Vec<_>>();

    let res = identify_candidates(x_tree, y, dist, slope_tolerance);

    let (keys, vals): (Vec<_>, Vec<_>) = res.into_iter().map(|(k, v)| (k, v)).unzip();

    list!(
        from = Integers::from_values(keys),
        to = List::from_values(vals)
    )
}

// For each linestring in X, expand to a vector of component lines
// For each Line in Xi, insert it into an RTree with the index of the linestring and it's slope
// For each linestring in Y, expand to a vector of component lines.
// We define a distance in euclidean units to search within, D.
// For each line in Yj, we create an AABB based on the lines envelope.
// We subtract D/2 from the LL and add D/2 to the UR
// We search the RTree for all lines that intersect the AABB
// We compare the stored slope with the slope or our line and check if they are within a tolerance
extendr_module! {
    mod rnetmerge;
    fn rnet_merge;
}

// Notes:
// after geting candidates the bounding boxes might intersect but still will be
// far away
// after comparing slopes, the distance should be captured and checked so that
// we can make sure they're within a distance of eachother
// how much of x is in y??

// fn create_target_rtree(y: Vec<LineString>, D: f64) {
//     let half_d = D/2.0;

//     let _ = y
//         .iter()
//         .enumerate()
//         .map(|(i, yi)| {
//             yi.lines()
//                 .map(|yij| {

//                     let bb = yij.bounding_rect();
//                     let (ll_x, ll_y) = bb.min().x_y();
//                     let (ur_x, ur_y) = bb.max().x_y();
        
//                     // create the AABB
//                     let aabb = AABB::from_corners(
//                         [ll_x - half_d, ll_y - half_d],
//                         [ur_x + half_d, ur_y + half_d],
//                     );

                    
//                 })
//         });

// }