use extendr_api::prelude::*;

pub mod cast;
pub mod expand;
pub mod combine;

extendr_module! {
    mod casting;
    use cast;
    use expand;
    use combine;
}
