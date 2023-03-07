use extendr_api::prelude::*;

pub mod wktimpl;
pub mod geojsonimpl;
pub mod wkbimpl;

extendr_module! {
    mod io;
    use wktimpl;
    use wkbimpl;
    use geojsonimpl;
}
