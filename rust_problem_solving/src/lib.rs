#[cfg(feature = "tree")]
mod tree;

#[cfg(feature = "patterns")]
mod patterns;
#[cfg(feature = "patterns")]
use patterns::{alphabet::*, ladder::*, square::*, triangle::*};

#[cfg(feature = "maths")]
mod maths;
#[cfg(feature = "maths")]
use maths::{
    armstrong::*, factoring::*, gcd_lcd::*, modulo::*, palindrome::*, prime::*, trailing_zero::*,
};

#[cfg(feature = "recursion")]
mod recursion;

#[cfg(feature = "recursion")]
use recursion::{array::*, factorial::*, fibonacci::*, n::*, string::*, sum::*};

#[cfg(feature = "list")]
mod list;

#[cfg(any(feature = "sort", feature = "mergeSort"))]
mod sort;
#[cfg(feature = "sort")]
use sort::{
    bubble::*,
    insertion::*,
    quick_sort::{qs_v1::quick_sort, *},
    selection::*,
    *,
};

#[cfg(any(feature = "sort", feature = "mergeSort"))]
use crate::sort::merge_sort;

#[cfg(feature = "array")]
mod array;

#[cfg(feature = "binarySearch")]
mod binary_search;

#[cfg(feature = "hashing")]
mod hashing;

#[cfg(feature = "linkedList")]
mod linked_list;


#[cfg(feature = "stack")]
mod stack;
#[cfg(feature = "queue")]
mod queue;

#[cfg(feature = "dynamic_programming")]
mod dynamic_programming;

#[cfg(feature = "bit")]
pub mod bit;