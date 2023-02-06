use extendr_api::prelude::*;

pub mod wktimpl;
use wktimpl::*;


pub mod geojsonimpl;
use geojsonimpl::*;

extendr_module! {
    mod io;
    use wktimpl;
    use geojsonimpl;
}