// Print all Divisors of a given Number

// Brute approach
// Time: theta(N)
// Space: theta(1)
pub fn print_all_divisors_v1(n: u32){

    for i in 1..=n{
        if n % i == 0 {
            println!("Divisor: {}", i);
        }
    }

}

// Optimal approach: 
// - the quotient of a divisor is also a divisor
// - root of the number is centre of series of divisor
// Corner case: N is perfect square; divisor == quotient in this case
// Time: O(sqrt(n))
// Space: O(1)
pub fn print_all_divisors_v2(n: u32){
    let root = (n as f32).sqrt() as u32;
    for i in 1..=root{
        if n % i == 0 {
            println!("Divisor: {}", i);
            let quotient = n/i;
            if quotient != i {
                println!("Divisor: {}", n/i);
            }
        }
    }
}