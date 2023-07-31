#![allow(unused_imports)]

mod patterns;
use patterns::{ladder::*, square::*, triangle::*, alphabet::*};

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = args.get(1).unwrap();
    let input: u32 = input.parse().unwrap();

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
    print_charv2(input);

}
