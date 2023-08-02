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