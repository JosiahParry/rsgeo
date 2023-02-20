use extendr_api::prelude::*;

pub mod cast;
use cast::*;

pub mod expand;
use expand::*;

extendr_module! {
    mod casting;
    use cast;
    use expand;
}
