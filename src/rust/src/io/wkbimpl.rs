use extendr_api::prelude::*;
use wkb::*;

use crate::to_pntr;
use crate::utils::determine_geoms_class;
use crate::{geoms::from_list, types::Geom};

#[extendr]
/// WKB Translation
///
/// @export
/// @rdname wkb
fn wkb_from_geom(x: Robj) -> Vec<u8> {
    let x = Geom::from(x).geom;
    geom_to_wkb(&x).unwrap()
}

#[extendr]
/// @export
/// @rdname wkb
fn wkb_from_geoms(x: List) -> List {
    let x = from_list(x);
    x.into_iter()
        .map(|x| geom_to_wkb(&x.geom).unwrap())
        .collect::<List>()
}

use std::io::Cursor;

#[extendr(use_try_from = true)]
/// @export
/// @rdname wkb
fn wkb_to_geom(x: Vec<u8>) -> Robj {
    let mut bytes_cursor = Cursor::new(x);
    let p = bytes_cursor.read_wkb().unwrap();
    to_pntr(Geom::from(p))
}

#[extendr(use_try_from = true)]
/// @export
/// @rdname wkb
fn wkb_to_geoms(x: List) -> Robj {
    let res = x
        .into_iter()
        .map(|(_, x)| wkb_to_geom(raw_to_vecu8(x)))
        .collect::<List>()
        .into_robj();

    res.set_attrib("class", determine_geoms_class(&res))
        .unwrap()
}

fn raw_to_vecu8(x: Robj) -> Vec<u8> {
    let x = extendr_api::Raw::try_from(x).unwrap();
    x.as_slice()
        .iter()
        .map(|x| x.to_owned())
        .collect::<Vec<u8>>()
}

//extendr_api::raw
extendr_module! {
    mod wkbimpl;
    fn wkb_from_geom;
    fn wkb_from_geoms;
    fn wkb_to_geom;
    fn wkb_to_geoms;
}
