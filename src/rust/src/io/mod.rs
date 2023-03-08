use extendr_api::prelude::*;

pub mod geojsonimpl;
pub mod wkbimpl;
pub mod wktimpl;

extendr_module! {
    mod io;
    use wktimpl;
    use wkbimpl;
    use geojsonimpl;
}
