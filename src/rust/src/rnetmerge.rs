use core::ops::Range;
use extendr_api::prelude::*;
use geo::{BoundingRect, EuclideanDistance};
use geo_types::{LineString, Rect};
use rstar::primitives::{CachedEnvelope, GeomWithData};
use rstar::{RTree, RTreeObject, AABB};
use sfconversions::{Geom, IntoGeom};
use std::collections::BTreeMap;

// use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn create_line_segment_tree(x: Vec<LineString>) -> RTree<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>> {
    // for each geometry in the vector x
    let x_segments = x
        .par_iter()
        // we get the index position
        .enumerate()
        .map(|(i, xi)| {
            xi
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
                .collect::<Vec<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>>>()
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

fn _range_contains(r1: Range<f64>, r2: Range<f64>) -> bool {
    r1.contains(&r2.start) || r1.contains(&r2.end)
}


fn overlap_range(r1: Range<f64>, r2: Range<f64>) -> Option<Range<f64>> {
    if r1.end < r2.start || r2.end < r1.start {
        None
    } else {
        Some(r1.start.max(r2.start)..r1.end.min(r2.end))
    }
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




use itertools::MultiUnzip; 
#[extendr]
fn rnet_merge(x: List, y: List, dist: f64, slope_tolerance: f64) -> Robj {

    let x = x.into_iter().map(|(_, xi)| LineString::try_from(Geom::from(xi).geom).unwrap()).collect::<Vec<_>>();
    let x_tree = create_line_segment_tree(x);
    // from rsgeo vector create vector of linestrings. will not handle missingvalues
    let y = y
        .into_iter()
        .map(|(_, lns)| LineString::try_from(Geom::from(lns).geom).unwrap())
        .collect::<Vec<_>>();

    let res = identify_candidates(x_tree, y, dist, slope_tolerance);

    let (ks, js, shared_lens): (Vec<_>, Vec<_>, Vec<_>) = res
        .into_iter()
        .flat_map(|(k, v)| {
            let (j, shared_len): (Vec<_>, Vec<_>) = v.into_iter().unzip();
            let ks = vec![k; j.len()];
            ks.into_iter()
                .zip(j.into_iter())
                .zip(shared_len.into_iter())
                .map(|((k, j), shared_len)| (k, j, shared_len))
        })
        .multiunzip();

    data_frame!(i = ks, j = js, shared_len = shared_lens)

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


fn identify_candidates(
    x: RTree<GeomWithData<CachedEnvelope<Geom>, (usize, f64)>>,
    y: Vec<LineString>,
    dist: f64,
    slope_tolerance: f64,
) -> BTreeMap<i32, Vec<(i32, f64)>> {
    let half_d = dist / 2.0;
    let mut matches: BTreeMap<i32, Vec<(i32, f64)>> = BTreeMap::new();
    // let match_arc = Arc::new(Mutex::new(matches));

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
                    // let ci_env = ci.envelope();
                    // let bbox_1 = Rect::new(ci_env.lower(), ci_env.upper()); 
                    let bbox_2 = li.bounding_rect();

                    let x_overlap = overlap_range(x_range(&bbox_1), x_range(&bbox_2));
                    let y_overlap = overlap_range(y_range(&bbox_1), y_range(&bbox_2));

                    // if theres overlap then we do a distance based check
                    if x_overlap.is_some() || y_overlap.is_some() {
                        // calculate the distance from the line segment
                        // if its within our threshold we include it;
                        let d = ci.geom().geom.euclidean_distance(&li);
                        // if distance is less than or equal to tolerance, add the key
                        if d <= dist {
                            let shared_len = solve_segment_length(x_overlap, y_overlap, &bbox_1);
                            // add 1 for R indexing
                            // ensures that no duplicates are inserted. Creates a new empty vector is needed
                            // let mut entry = match_arc.lock().unwrap();
                            // let entry = entry.entry((i + 1) as i32).or_insert_with(Vec::new);
                            let entry = matches.entry((i + 1) as i32).or_insert_with(Vec::new);
                            let j_plus_one = (j + 1) as i32;
                            
                            if let Some(tuple) = entry.iter_mut().find(|(x, _)| *x == j_plus_one) {
                                tuple.1 += shared_len;
                            } else {
                                entry.extend(std::iter::once((j_plus_one, shared_len)));
                            }
                        }
                    }
                }
            });
        })
    });

    // this approach using an arc and mutex is slower than single threading
    // let matches = match_arc.lock().unwrap().clone();
    matches
}



// Copilot generated pseudo-code
// 1. Define `half_d` as half of the input `dist`.
// 2. Initialize an empty `SkipMap` called `matches`.
// 3. For each `LineString` `lns` in `y` with its index `j`:
//    1. For each line `li` in `lns`:
//       1. Get the bounding rectangle `bb` of `li`.
//       2. Calculate the lower-left (`ll_x`, `ll_y`) and upper-right (`ur_x`, `ur_y`) coordinates of `bb`.
//       3. Create an `AABB` (Axis-Aligned Bounding Box) `aabb` by expanding `bb` by `half_d` in all directions.
//       4. Find all geometries in `x` (an RTree) that intersect with `aabb`. These are the intersection candidates `cands`.
//       5. For each candidate `ci` in `cands`:
//          1. Extract the index `i` and slope `slope` from `ci`.
//          2. Check if the absolute difference between `slope` and the slope of `li` is less than `slope_tolerance`. This is `is_tolerant`.
//          3. If `is_tolerant` is true:
//             1. Get the bounding rectangles `bbox_1` and `bbox_2` of `ci` and `li` respectively.
//             2. Calculate the overlap in the x and y ranges of `bbox_1` and `bbox_2`. This is `x_overlap` and `y_overlap`.
//             3. If there is overlap in either the x or y range:
//                1. Calculate the Euclidean distance `d` between `ci` and `li`.
//                2. If `d` is less than or equal to the input `dist`:
//                   1. Calculate the shared length `shared_len` of the segments using `x_overlap`, `y_overlap`, and `bbox_1` with `solve_segment_length()`.
//                   2. Get or create an entry in `matches` for the key `(i + 1) as i32`. This is `entry`.
//                   3. If `entry` contains a tuple with the first element equal to `(j + 1) as i32`, increment the second element of the tuple by `shared_len`.
//                   4. Otherwise, add a new tuple `(j + 1 as i32, shared_len)` to `entry`.
// 4. Return `matches`.

