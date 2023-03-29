#![feature(no_coverage)]

////////////////////////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! assume_full_coverage {
    ($expression:expr) => {
        (#[no_coverage]
        || $expression)()
    };
}
