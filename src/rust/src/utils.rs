use extendr_api::prelude::*;

pub fn geom_class(cls: &str) -> [String; 3] {
    let cls = cls.to_uppercase();
    let geom_class = "rs_".to_owned() + cls.as_str();
    
    [geom_class, String::from("vctrs_vctr"), String::from("list")]
}

pub fn determine_geoms_class(x: &Robj) -> [String; 3] {
    let x: List = x.try_into().unwrap();

    let class = x[0].class().unwrap().nth(0).unwrap();

    let all_identical = x.iter().all(|(_, robj)| robj.class().unwrap().nth(0).unwrap() == class);

    let class = if all_identical { 
        x[0].class().unwrap().nth(0).unwrap() 
    } else {
        "geometrycollection"
    };

    geom_class(class)
}

extendr_module! {
    mod utils;
}

