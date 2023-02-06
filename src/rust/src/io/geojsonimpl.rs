use extendr_api::prelude::*;
use extendr_api::List;
use extendr_api::serializer::to_robj;
use geo_types::Geometry;
use crate::types::Geom;

use crate::to_pntr;
use rayon::prelude::*;




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



extendr_module! {
    mod geojsonimpl;
    fn read_geojson;
}