use geo_types::Geometry;
use wkt::ToWkt;
use wkt::TryFromWkt;
//use std::io::Read;
use extendr_api::prelude::*;
use extendr_api::{extendr, Robj, Strings};

use crate::to_pntr;
use crate::types::Geom;

#[extendr]
pub fn wkt_to_geom(x: &str) -> Robj {
    let x = Geometry::try_from_wkt_str(x).unwrap();
    to_pntr(Geom::from(x))
}

#[extendr]
pub fn wkt_to_geoms(x: Strings) -> List {
    x.iter().map(|x| wkt_to_geom(x.as_str())).collect::<List>()
}
#[extendr]
pub fn wkt_from_geom(x: Robj) -> Strings {
    let res = Geom::from(x).geom.to_wkt().to_string();
    Strings::from(res)
}

#[extendr]
pub fn wkt_from_geoms(x: List) -> Strings {
    let all_wkt = x
        .into_iter()
        .map(|(_, robj)| Geom::from(robj).geom.to_wkt().to_string());

    Strings::from_iter(all_wkt)
}

extendr_module! {
    mod wktimpl;
    fn wkt_to_geom;
    fn wkt_from_geom;
    fn wkt_to_geoms;
    fn wkt_from_geoms;
}
