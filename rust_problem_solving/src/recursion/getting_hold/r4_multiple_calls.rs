
mod pattern{
    fn f1(n:u32){
        if n > 10 || n < 1{
            return
        }
        f1(n+1);
        f1(n-1);
    }

}

fn get_fibonacci(n: u32) -> u32{
    if n < 1 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    get_fibonacci(n-1) + get_fibonacci(n-2)
}


fn print_fibonacci(n: u32, fib: &mut u32){
    if n<=1{
        *fib +=1;
        return
    }
    print_fibonacci(n-1, fib);
    println!("{}", fib);

}