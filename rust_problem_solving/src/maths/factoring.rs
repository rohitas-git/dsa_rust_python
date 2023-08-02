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
