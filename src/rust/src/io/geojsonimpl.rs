use extendr_api::prelude::*;
use extendr_api::List;
use extendr_api::serializer::to_robj;




#[extendr]
pub fn read_geojson(file: &str) -> List {
    let file = std::fs::File::open(file)
        .expect("this to work");
    
    
    
    let data = geojson::FeatureReader::from_reader(file);
    let mut obs = data.features();



    let res = obs.into_iter()
        .map(|x| to_robj(&x.unwrap()))
        .collect::<List>();

    res

    // for ft in obs {
    //     let x = ft.unwrap();
    //     let props = x.properties.unwrap();
    //     let robj = to_robj(&props).unwrap();
    //     let ll: List = robj.try_into().unwrap();
    // }
    

    // let init = obs.nth(0).unwrap();
    // let fts = init.unwrap();
    // let mut props = fts.properties.unwrap();

    // rprintln!("{:?}", props.len());

}


extendr_module! {
    mod geojsonimpl;
    fn read_geojson;
}