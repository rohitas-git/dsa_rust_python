#![allow(unused)]

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
}
