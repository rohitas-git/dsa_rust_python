pub fn print_binary_rep_v2(x: u32) {
    print!("{}: ",x);
    for i in (0..32).rev() {
        print!("{}", (x >> i) & 1);
    }
    println!();
}


pub fn print_binary_rep(x: u32) {
    println!("{}: {:b}",x, x);
}
