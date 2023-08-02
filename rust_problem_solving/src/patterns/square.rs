// #![allow(unused_imports)]
// #![allow(dead_code)]

pub fn print_square(n: u32){
    for _i in 0..n {
        for _j in 0..n{
            print!("*");
        }
        println!()
    }
}