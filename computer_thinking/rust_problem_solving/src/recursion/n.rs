

// Forward Recursion
pub fn print_1_to_n_recursion(n: u32){
    if n == 0 {
        return
    }
    println!("{}",n);
    print_1_to_n_recursion(n-1);
}

// Backward Recursion
pub fn print_1_to_n_backtracking(n: u32){
    if n == 0 {
        return
    }
    print_1_to_n_backtracking(n-1);
    println!("{}",n);
}

pub fn print_ntimes(n:u32){
    if n == 0 {
        return
    }
    println!("Wow !!!");

    print_ntimes(n-1);

}