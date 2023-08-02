// A prime number is a natural number that is only divisible by 1 and by itself.

// Idea: Divisor of n : (x,y)
//  say x <= y --> x*x <= n --> x <= sqrt(n)
// 
// Divisor pair of number n will always have one divisor will be smaller or equal to sqrt(N)
// Thus, if N has no divisor between 1..sqrt(n) => N is prime

// pub use is_prime::super_efficient;
pub mod is_prime{

    pub fn answer_super(n: u32){
        let c = super_efficient(n);
        println!("{}", c);
    }

    pub fn answer_better(n: u32){
        let c = efficient(n);
        println!("{}", c);
    }

    // worst case time theta(N) or in general, time O(N) 
    fn brute(n: u32)-> bool{
        if n == 1{
            return false;
        }
        for i in 2..n{
            if n % i == 0 {
                return false;
            }
        }   
        true
    }

    // Time: theta( sqrt(N) )
    // Space: O(1)
    fn efficient(n:u32)-> bool{
        if n == 1{
            return false;
        }
        let mut i = 2;
        while (i * i <= n){
            if n % i == 0 {
                return false;
            }
            i += 1 
        }   
        true
    }

    // worst case time - theta(sqrt(N))
    pub fn super_efficient(n: u32)->bool{
        if n == 1{
            return false
        }

        if n == 2 || n == 3{
            return true
        }

        if n % 2 == 0 || n % 3 == 0 {
            return false
        }

        let mut i = 5;

        while i * i <= n {
            if n % i == 0 || n % (i+2) == 0 {
                return false
            }
            i+= 6
        }
        return true;
    }
}

pub mod print_all_primes{
    use super::is_prime::super_efficient as is_prime;

    // check all numbers in 2..n+1 and print those that are prime 
    // Time: O(n*sqrt(n))
    pub fn naive(n: u32) {

        for i in 2..(n+1){
            if is_prime(i){
                print!("{} ", i);
            }
        }

    }

    // In numbers in 2..n+1, leave those numbers that multiplied from primes 
    // for 2: 4,6,8,10,12 ... till n+1
    // for 3: 6,9,12,15,18 .. till n+1
    // 
    // Print those numbers that are left over from above
    pub fn sieve_of_eratosthenes(n:u32){
        if n <=1{
            return
        }
        let n = usize::try_from(n).expect("unable to cast N as usize");
        let mut is_prime = vec![true; (n-1) as usize];
        let mut i:usize = 2;

        // O(sqrt(n)*n) ??
        while i*i <= n {
            if is_prime[i]{
                // disable numbers multiplied from prime numbers
                // O(n) ??
                for j in ( (2*i)..(n+1)).step_by(i){
                    is_prime[j] = false;
                }
            }
            i +=1 ;
        }
        // O(n) ??
        for i in 2..(n+1){
            if is_prime[i] {
                print!("{} ", i);
            }
        }
    }

    // time: O(N loglogN)
    // Optmizes the step where we switch off /lay off multiples of prime numbers
    pub fn sieve_of_eratosthenes_better(n:u32){
        if n <=1{
            return
        }
        let n = usize::try_from(n).expect("unable to cast N as usize");
        let mut is_prime = vec![true; (n+1) as usize];
        let mut i:usize = 2;

        while i <= n {

            if is_prime[i]{
                print!("{} ", i);
                for j in ( (i*i)..(n+1)).step_by(i){
                    is_prime[j] = false;
                    
                    if(j+i > n){break;}
                }
            }
            i +=1 ;
        }
    }
}