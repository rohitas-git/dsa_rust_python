#![allow(dead_code)]

#[cfg(feature = "sort")]
pub mod bubble;
#[cfg(feature = "sort")]
pub mod insertion;
#[cfg(feature = "sort")]
pub mod quick;
#[cfg(feature = "sort")]
pub mod selection;

#[cfg(any(feature = "mergeSort", feature = "sort"))]
pub mod merge_sort;
#[cfg(feature = "mergeSort")]
pub use merge_sort::do_sort;
