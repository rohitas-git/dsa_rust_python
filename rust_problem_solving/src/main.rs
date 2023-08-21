#![allow(unused)]

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

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = args.get(1).cloned().unwrap_or_default();
    let input: u32 = input.parse().unwrap_or(0);
    let _b = args
        .get(2)
        .unwrap_or(&String::from("0"))
        .parse()
        .unwrap_or(0);
    let n = input;

    let mut arr = [11, 3, 9, 7, 5, 2, 4];
    // qs_v2::quick_sort(&mut arr);

    // let r = palindrome("ABCDCBA");
    // println!("{}", r);

    // print_all_primes::naive(n);
    // print_all_primes::sieve_of_eratosthenes(n);
    // print_all_primes::sieve_of_eratosthenes_better(n);

    // is_prime(input);
    // is_prime::answer_better(input);
    // is_prime::answer_super(input);

    // prime_factorization::normal(input);
    // print_all_divisors::v2(input);
    // print_all_divisor::v1(input);

    // let mut arr = [1, 231, 1241, 41, 15, 12];
    // let l = arr.len();
    // reverse_array::rusty(&arr);
    // reverse_array::by_swapping(&mut arr);
    // reverse_array::recursive(&mut arr, 0, l-1);

    // reverse_array_v2(&arr);

    // caluclate_factorial::iterative(input);
    // caluclate_factorial::recursive(input);

    // sum_n_natural_numbers::parameterized_way(0,input);
    // println!("{}",sum_n_natural_numbers::functional_way(input));

    // print_n_to_1_v2(1,input);
    // print_n_to_1_v1(input,input);

    // print_text_ntimes(1,input);

    // print_1_to_n::backtracking_v2(input,input);
    // print_1_to_n::recursion_v2(1,input);
    // print_1_to_n::backtracking(input);
    // print_1_to_n::recursion(input);

    // trailing_zeroe_in_factorial(input);

    // is_armstrong(input);

    // let res = gcd_brute_force(input, b);
    // println!("{}", res);

    // lcm::naive(input, _b);
    // lcm::efficient(input, _b)

    // is_palindromev1(input);
    // is_palindromev2(input);

    // print_square(input);

    // print_ladderv1(input);
    // print_ladderv2(input);
    // print_ladderv3(input);
    // print_ladderv4(input);

    // print_trianglev1(input);
    // print_trianglev2(input);
    // print_trianglev3(input);
    // print_trianglev4(input);
    // print_trianglev5(input);

    // print_charv1(input);
    // print_charv2(input);
}
