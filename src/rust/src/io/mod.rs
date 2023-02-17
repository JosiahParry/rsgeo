use extendr_api::prelude::*;

pub mod wktimpl;
use wktimpl::*;

pub mod geojsonimpl;
use geojsonimpl::*;


pub mod wkbimpl;
use wkbimpl::*;

extendr_module! {
    mod io;
    use wktimpl;
    use wkbimpl;
    use geojsonimpl;
}
