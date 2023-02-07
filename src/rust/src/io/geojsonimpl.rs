use extendr_api::prelude::*;
use extendr_api::{List};
use extendr_api::serializer::to_robj;
use geo_types::Geometry;
use crate::types::Geom;

use crate::to_pntr;
use rayon::prelude::*;
use serde_json::Map;
use serde_json::Value;



use geojson::GeoJson;


//#[extendr]
// pub fn read_geojson(file: &str) -> List {
//     let file = std::fs::File::open(file)
//         .expect("this to work");
    
    
    
//     let data = geojson::FeatureReader::from_reader(file);
//     let mut obs = data.features();

//     let x = data.deserialize();
//     let x = x.unwrap();
//     x.into_iter()
//         .map(|w | w.unwrap())

// }


#[extendr]
pub fn read_geojson(file: &str) -> List {

    //let file = std::fs::File::open(file).expect("geojson file to be found");
    let geojson_str = std::fs::read_to_string(file).unwrap();
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    // read only the geometry
    // it doesn't assign the proper class. i suspect this can be fixed on the 
    // r side quickly
    // adapted / stolen directly from https://github.com/urschrei/geojson_example/blob/master/src/owned.rs
    let res = match geojson {
        GeoJson::FeatureCollection(collection) => collection
            .features.into_par_iter()
            .map(|geo| 
                Geometry::try_from(geo.geometry.unwrap()).unwrap()
        ).collect::<Vec<Geometry>>(),
        GeoJson::Feature(feature) => {
            vec![Geometry::try_from(feature.geometry.unwrap()).unwrap()]
        },

        GeoJson::Geometry(geom) => {
            vec![Geometry::try_from(geom).unwrap()]
        }
    };

    res.into_iter()
        .map(|geom| to_pntr(Geom::from(geom)))
        .collect::<List>()

}



#[extendr]
pub fn read_geojson_string(geojson_str: String) -> List {

    //let file = std::fs::File::open(file).expect("geojson file to be found");
    //let geojson_str = std::fs::read_to_string(file).unwrap();
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    // read only the geometry
    // it doesn't assign the proper class. i suspect this can be fixed on the 
    // r side quickly
    // adapted / stolen directly from https://github.com/urschrei/geojson_example/blob/master/src/owned.rs
    let res = match geojson {
        GeoJson::FeatureCollection(collection) => collection
            .features.into_par_iter()
            .map(|geo| 
                Geometry::try_from(geo.geometry.unwrap()).unwrap()
        ).collect::<Vec<Geometry>>(),
        GeoJson::Feature(feature) => {
            vec![Geometry::try_from(feature.geometry.unwrap()).unwrap()]
        },

        GeoJson::Geometry(geom) => {
            vec![Geometry::try_from(geom).unwrap()]
        }
    };

    res.into_iter()
        .map(|geom| to_pntr(Geom::from(geom)))
        .collect::<List>()

}


#[extendr]
pub fn read_geojson_props(file: &str) -> Robj {

    let geojson_str = std::fs::read_to_string(file).unwrap();
    let geojson = geojson_str.parse::<GeoJson>().unwrap();

    let res = match geojson {
        GeoJson::FeatureCollection(collection) => collection
            .features.into_par_iter()
            .map(|feat| feat.properties.unwrap())
            .collect::<Vec<Map<String, Value>>>(),
        GeoJson::Feature(feature) => {
            vec![feature.properties.unwrap()]
        },

        GeoJson::Geometry(_geom) => {
            vec![]
        }
    };

    // pull first element to find the keys 
    let x1 = res[0].clone();
    // collect keys into vec of string
    let keys = x1.keys().collect::<Vec<&String>>();

    let n = res.len(); // number row 
    let nn = keys.len(); // number cols 


    //let mut col_types: Vec<&str> = Vec::with_capacity(nn);
    let mut col_types = match_type(&res[0]);

    for obs in res.iter().skip(1) {
        let keys_i =  match_type(obs);

        let which_unknown = col_types.iter() 
            .enumerate() //now you have indexes :)
            .filter(|(_index, col)| **col == "idfk")
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        // if all aren't true break
        if which_unknown.len() == 0 { 
    
            break 
        }

        // if not fill the types 
        for i in which_unknown.into_iter() {
            col_types[i] = keys_i[i];
        }

    }

    // cast rest to Robj
    let res = to_robj(&res).unwrap();
    let res = List::try_from(res).unwrap();

    let mut res_vec: Vec<Vec<Robj>> = Vec::with_capacity(n);

    // instantiate vectors 
    for _ in 0..(nn) {
        res_vec.push(Vec::with_capacity(nn))
    }

    // fill vectors
    for i in 0..(n) {
        let xi = List::try_from(res[i].to_owned()).unwrap();
        for j in 0..(nn) {
            let val = xi[j].to_owned();
            res_vec[j].push(val)
        }
    }

    let res_vec = res_vec.into_iter().enumerate()
        .map(|(i, col)| {
            let ctype = col_types[i];
                match ctype {
                    "double" => Doubles::from_iter(col.into_iter().map(|xi| Rfloat::try_from(xi).unwrap())).into_robj(),
                    "character" => Strings::from_iter(
                        col.into_iter().map(|xi| Rstr::try_from(xi.as_str().unwrap_or(&Rstr::na())).unwrap())
                    )
                        .into_robj(),
                    "logical" => Logicals::from_iter(col.into_iter().map(|xi| Rbool::try_from(xi).unwrap())).into_robj(),
                    &_ =>  Logicals::from_iter(
                        col.into_iter().map(|_| Rbool::na_value())
                    )
                        .into_robj(),
                    
                }
        }).collect::<Vec<Robj>>();

    let res = List::from_names_and_values(keys, res_vec).unwrap();


    let index = (1..n+1).map(|i| i as i32).collect::<Vec<i32>>();
    res.set_attrib("class", "data.frame").unwrap()

        .set_attrib("row.names", index).unwrap()


}

extendr_module! {
    mod geojsonimpl;
    fn read_geojson;
    fn read_geojson_string;
    fn read_geojson_props;
}

fn match_type(x: &Map<String, Value>) -> Vec<&str> {
    x.values().into_iter()
        .map(
        |val| match val {
            Value::Number(_) => "double",
            Value::String(_) => "character",
            Value::Bool(_) => "logical",
            _ => "idfk"
        }
    ).collect::<Vec<&str>>()
}