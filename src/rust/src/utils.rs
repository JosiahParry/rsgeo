use extendr_api::prelude::*;
use sfconversions::Geom;
// Create a blank pointer to be used in ptype casting

#[extendr]
fn null_pntr() -> Robj {
    ExternalPtr::new(0).into_robj()
}

#[extendr]
fn print_geom(x: Robj) -> String {
    Geom::from(x).print().into()
}

extendr_module! {
    mod utils;
    fn null_pntr;
    fn print_geom;
    // fn restore_geoms;
}
