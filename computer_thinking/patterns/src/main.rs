#![allow(unused_imports)]

mod patterns;
use patterns::{alphabet::*, ladder::*, square::*, triangle::*};

mod maths;
use maths::{gcd_lcd::*, palindrome::*};

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = args.get(1).unwrap();
    let input: u32 = input.parse().unwrap();
    let b = args.get(1).unwrap().parse().unwrap();

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

    // is_palindromev1(input);
    // is_palindromev2(input);

    let res = gcd_brute_force(input, b);

    println!("{}", res);
}
