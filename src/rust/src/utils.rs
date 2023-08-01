use extendr_api::prelude::*;
// Create a blank pointer to be used in ptype casting
#[extendr]
fn null_pntr() -> Robj {
    ExternalPtr::new(0).into_robj()
}

#[extendr]
pub fn restore_geoms(x: Robj) -> Robj {
    let class_to_set = determine_geoms_class(&x);
    x.set_attrib("class", class_to_set).unwrap()
}

extendr_module! {
    mod utils;
    fn null_pntr;
    fn restore_geoms;
}
