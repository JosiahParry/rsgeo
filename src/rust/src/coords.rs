use extendr_api::prelude::*;
use sfconversions::Geom;
use geo_types::*;

use geo::{CoordsIter};

// TODO - missing value handling for coordinate conversions
#[extendr]
fn point_to_coords(x: List) -> Robj {
    let (x, y): (Vec<f64>, Vec<f64>) = x
        .into_iter()
        .filter_map(|(_, robj)| {
            let p = <&Geom>::from_robj(&robj).unwrap();
            let crds = match p.geom {
                Geometry::Point(p) => Some(p.x_y()),
                _ => None
            };
            crds
        }).unzip();
    data_frame!(x = x, y = y)
}

#[extendr]
fn multipoint_to_coords(x: List) -> Robj {
    let res = x   
        .into_iter()
        .enumerate()
        .filter_map(|(i, (_, robj))| {
            let g = <&Geom>::from_robj(&robj).unwrap();
            let crds = match &g.geom {
                Geometry::MultiPoint(mp) => {
                    let n = mp.coords_count();
                    let crds = mp
                        .coords_iter()
                        .map(|c| c.x_y())
                        .zip(vec![(i as i32) + 1; n])
                        .collect::<Vec<((f64, f64), i32)>>();
                    Some(crds)
                },
                _ => None
            };
            crds
        })
        .flat_map(|x| x)
        .collect::<Vec<((f64, f64), i32)>>();

    let (xy, id): (Vec<(f64, f64)>, Vec<i32>) = res.into_iter().unzip();
    let (x, y): (Vec<f64>, Vec<f64>) = xy.into_iter().unzip();

    data_frame!(x = x, y = y, id = id)
}

#[extendr]
fn multilinestring_to_coords(x: List) -> Robj {
    let all_coords = x
        .into_iter()
        .enumerate()
        .flat_map(|(i, (_, robj))| {
            let mln = MultiLineString::try_from(Geom::from(robj).geom).unwrap();
            let coords = multilinestring_coords(mln);
            let n = coords.len();
        coords
            .into_iter()
            .zip(vec![(i as i32) + 1; n])
        .collect::<Vec<(((f64, f64), i32), i32)>>()
    })
    .collect::<Vec<(((f64, f64), i32), i32)>>();

    let (xyid, mlns_id): (Vec<((f64, f64), i32)>, Vec<i32>) = all_coords.into_iter().unzip();
    let (xy, line_id): (Vec<(f64, f64)>, Vec<i32>) = xyid.into_iter().unzip();
    let (x, y): (Vec<f64>, Vec<f64>) = xy.into_iter().unzip();
    data_frame!(x = x, y = y, line_id = line_id, multilinestring_id = mlns_id)
}

fn multilinestring_coords(x: MultiLineString) -> Vec<((f64, f64), i32)> {
    x
        .into_iter()
        .enumerate()
        .flat_map(|(i, ln)| {
            ln
                .coords_iter()
                .map(|c| (c.x_y(), (i + 1) as i32))
                .collect::<Vec<((f64, f64), i32)>>()
        })
        .collect::<Vec<((f64, f64), i32)>>()
}

fn linestring_coords(x: &LineString) -> Vec<(f64, f64)> {
    x   
        .coords_iter()
        .map(|c| c.x_y())
        .collect::<Vec<(f64, f64)>>()
}

#[extendr]
fn linestring_to_coords(x: List) -> Robj {
    let all_coords = x
        .into_iter()
        .enumerate()
        .flat_map(|(i, (_, robj))| {
            let lns = LineString::try_from(Geom::from(robj).geom).unwrap();
            let coords = linestring_coords(&lns);
            let n = coords.len();
            coords
                .into_iter()
                .zip(vec![(i as i32) + 1; n])
                .collect::<Vec<((f64, f64), i32)>>()
        })
        .collect::<Vec<((f64, f64), i32)>>();

    let (xy, id): (Vec<(f64, f64)>, Vec<i32>) = all_coords.into_iter().unzip();
    let (x, y): (Vec<f64>, Vec<f64>) = xy.into_iter().unzip();
    data_frame!(x = x, y = y, line_id = id)
}

#[extendr]
fn polygon_to_coords(x: List) -> Robj {
    let all_coords = x
        .into_iter()
        .enumerate()
        .flat_map(|(i, (_, robj))| {
            let poly = Polygon::try_from(Geom::from(robj).geom).unwrap();
            let coords = polygon_coords(poly);
            let n = coords.len();
            coords
                .into_iter()
                .zip(vec![(i as i32) + 1; n])
            .collect::<Vec<(((f64, f64), i32), i32)>>()
        })
        .collect::<Vec<(((f64, f64), i32), i32)>>();

    let (xyid, poly_id): (Vec<((f64, f64), i32)>, Vec<i32>) = all_coords.into_iter().unzip();
    let (xy, line_id): (Vec<(f64, f64)>, Vec<i32>) = xyid.into_iter().unzip();
    let (x, y): (Vec<f64>, Vec<f64>) = xy.into_iter().unzip();
    data_frame!(x = x, y = y, line_id = line_id, polygon_id = poly_id)
}

fn polygon_coords(x: Polygon) -> Vec<((f64, f64), i32)> {
    let mut exterior = x
        .exterior_coords_iter()
        .map(|c| (c.x_y(), 1 as i32))
        .collect::<Vec<((f64, f64), i32)>>();

    let interior = x 
        .interiors()
        .iter()
        .enumerate()
        .flat_map(|(i, ring)| {
            ring
                .coords_iter()
                .map(|c| (c.x_y(), (i + 1) as i32))
                .collect::<Vec<((f64, f64), i32)>>()
        })
        .collect::<Vec<((f64, f64), i32)>>();
    
    exterior.extend(interior.into_iter());

    // let (xy, ids): (Vec<(f64, f64)>, Vec<i32>) = exterior.into_iter().unzip();
    // let (x, y): (Vec<f64>, Vec<f64>) = xy.into_iter().unzip();
    exterior
    // data_frame!(x = x, y = y, ring_id = ids)
}



#[extendr]
fn multipolygon_to_coords(x: List) -> Robj {
    let all_coords = x
        .into_iter()
        .enumerate()
        .flat_map(|(i, (_, robj))| {
            let poly = MultiPolygon::try_from(Geom::from(robj).geom).unwrap();
            let coords = multipolygon_coords(poly);
            let n = coords.len();
            coords
                .into_iter()
                .zip(vec![(i as i32) + 1; n])
            .collect::<Vec<((((f64, f64), i32), i32), i32)>>()
        })
        .collect::<Vec<((((f64, f64), i32), i32), i32)>>();

    let (xyididid, multipoly_id): (Vec<(((f64, f64), i32), i32)>, Vec<i32>) = all_coords.into_iter().unzip();
    let (xyidid, poly_id): (Vec<((f64, f64), i32)>, Vec<i32>) = xyididid.into_iter().unzip();
    let (xyid, line_id): (Vec<(f64, f64)>, Vec<i32>) = xyidid.into_iter().unzip();
    let (x, y): (Vec<f64>, Vec<f64>) = xyid.into_iter().unzip();
    data_frame!(x = x, y = y, line_id = line_id, polygon_id = poly_id, multipolygon_id = multipoly_id)
}


fn multipolygon_coords(x: MultiPolygon) -> Vec<(((f64, f64), i32), i32)> {
    x
        .0
        .into_iter()
        .enumerate()   
        .flat_map(|(i, poly)| {
            let pi = polygon_coords(poly);
            let index = vec![(i as i32) + 1; pi.len()];
            pi
                .into_iter()
                .zip(index.into_iter())
                .collect::<Vec<(((f64, f64), i32), i32)>>()
        })
        .collect::<Vec<(((f64, f64), i32), i32)>>()
}


extendr_module! {
    mod coords;
    fn point_to_coords;
    fn multipoint_to_coords;
    fn linestring_to_coords;
    fn multilinestring_to_coords;
    fn polygon_to_coords;
    fn multipolygon_to_coords;
}

