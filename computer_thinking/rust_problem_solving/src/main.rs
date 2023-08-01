#![allow(unused_imports)]
#![allow(unused)]

#[cfg(feature = "patterns")]
mod patterns;

#[cfg(feature = "patterns")]
use patterns::{alphabet::*, ladder::*, square::*, triangle::*};

#[cfg(feature = "maths")]
mod maths;

#[cfg(feature = "maths")]
use maths::{armstrong::*, divisor::*, gcd_lcd::*, palindrome::*, prime::*, trailing_zero::*};

#[cfg(feature = "recursion")]
mod recursion;

#[cfg(feature = "recursion")]
use recursion::n::*;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = args.get(1).unwrap();
    let input: u32 = input.parse().unwrap();
    let _b = args.get(1).unwrap().parse().unwrap_or(0);

    // print_1_to_n_backtracking(input);
    // print_1_to_n_recursion(input);
    // print_ntimes(input);

    // is_prime(input);

    // print_all_divisors_v2(input);
    // print_all_divisors_v1(input);

    // trailing_zeroe_in_factorial(input);

    // is_armstrong(input);

    // let res = gcd_brute_force(input, b);
    // println!("{}", res);

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
