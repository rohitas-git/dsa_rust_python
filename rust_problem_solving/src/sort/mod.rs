#![allow(dead_code)]

#[cfg(feature = "sort")]
pub mod bubble;

#[cfg(feature = "sort")]
pub mod insertion;

#[cfg(feature = "sort")]
pub mod quick_sort;

#[cfg(feature = "sort")]
pub mod quick_sort_hoare;

#[cfg(feature = "sort")]
pub mod quick_sort_lomuto;

#[cfg(feature = "sort")]
pub mod selection;

#[cfg(any(feature = "mergeSort", feature = "sort"))]
pub mod merge_sort;

#[cfg(feature = "mergeSort")]
pub use merge_sort::do_sort;

#[cfg(feature = "sort")]
pub mod heap_sort;

