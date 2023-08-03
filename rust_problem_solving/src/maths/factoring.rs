use super::prime::is_prime::super_efficient as is_prime;

pub mod prime_factorization {

    use super::is_prime;

    pub fn normal(n: u32) {

        for i in 2..n{
            let mut count = 0;
            let mut x = i;

            if  is_prime(i) && is_divisor(i,n) {
                while is_divisor(x,n){
                    print!("{} ", i);
                    x = x * i;
                }
                println!("");
            }   
        }
    }

    fn is_divisor(i:u32,n:u32)-> bool {
        if n % i == 0 {
            return true;
        } 
        else {
            return false;
        } 
    }
}

pub mod print_all_divisors{

    // Time O(n); space O(1)
    pub fn worst(n:u32){
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
    pub fn v1(n:u32){
        let mut i =1;
        let r = (n as f32).sqrt() as u32;

        while i*i <= n {
            if n % i == 0{
                println!("{}",i);
                if i != n/i {
                    println!("{}", n/i);
                }
            }
            i+=1;
        }
    }

    // time O(sqrt(N));  space O(1)
    // Prints the divisors in order 
    pub fn v2(n:u32){
        let mut i =1;
        let r = (n as f32).sqrt() as u32;
        
        // while divisor < sqrt(n)
        while i*i < n {
            if n % i == 0{
                println!("{}",i);
            }
            i+=1;
        }

        // print 2nd divisor in the divisor pair corresponding to 1st divisors in descending order 
        while i>= 1 {
            if n % i == 0 && n/i != (i-1){
                println!("{}", n/i);
            }
            i -= 1;
        }
    }
}

