use extendr_api::prelude::*;

pub mod cast;
use cast::*;

pub mod expand;
use expand::*;

pub mod combine;
use combine::*; 

extendr_module! {
    mod casting;
    use cast;
    use expand;
    use combine;
}
