use extendr_api::prelude::*;

pub mod cast;
pub mod combine;
pub mod expand;

extendr_module! {
    mod casting;
    use cast;
    use expand;
    use combine;
}
