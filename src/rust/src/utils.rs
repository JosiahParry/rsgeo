use extendr_api::prelude::*;

pub fn geom_class(cls: &str) -> [String; 3] {
    let cls = cls.to_uppercase();
    let geom_class = "rs_".to_owned() + cls.as_str();

    [geom_class, String::from("vctrs_vctr"), String::from("list")]
}

pub fn determine_geoms_class(x: &Robj) -> [String; 3] {
    let x: List = x.try_into().unwrap();

    let class = x[0].class().unwrap().nth(0).unwrap();

    let all_identical = x
        .iter()
        .all(|(_, robj)| robj.class().unwrap().nth(0).unwrap() == class);

    let class = if all_identical {
        x[0].class().unwrap().nth(0).unwrap()
    } else {
        "geometry"
    };

    geom_class(class)
}

// Create a blank pointer to be used in ptype casting
#[extendr]
fn null_pntr() -> Robj {
    ExternalPtr::new(0).into_robj()
}

//
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

use extendr_api::scalar::Scalar; 