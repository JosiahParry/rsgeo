use extendr_api::prelude::*;

pub mod cast;
pub mod combine;
pub mod expand;
pub mod explode;

extendr_module! {
    mod casting;
    use cast;
    use expand;
    use combine;
    use explode;
}
