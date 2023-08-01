// A prime number is a natural number that is only divisible by 1 and by itself.

// Time: theta( sqrt(N) )
// Space: O(1)
pub fn is_prime(n: u32) {

    if n == 2 {
        println!("{} is a prime number", n);
        return
    }

    let root = (n as f32).sqrt() as u32;
    for i in 2..=root {
        if n % i == 0 {
            println!("{} is not a prime number", n);
            return;
        }
    }
    println!("{} is a prime number", n);
}
