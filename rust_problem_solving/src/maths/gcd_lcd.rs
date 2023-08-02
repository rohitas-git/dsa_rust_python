
// Time( log(min(a,b)) ) 
// link (https://www.geeksforgeeks.org/time-complexity-of-euclidean-algorithm/)
// Space : O(1)
pub fn gcd_optimal(mut a:u32, mut b:u32)->u32{
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
    
}

pub fn gcd_brute_force(a:u32, b:u32)->u32{
    let l = std::cmp::min(a,b);
    
    for i in 0..l{
        if a%i == 0 && b%i == 0{
            return i;
        }
    }
    0
}

pub fn lcm_brute_force(a:u32, b:u32)->u32{
    let result = a * b / (gcd_brute_force(a,b));
    result 
}

pub fn lcm_optimal(a:u32, b:u32)->u32{
    let result = a * b / (gcd_optimal(a,b));
    result 
}

pub mod lcm {

    // Worst case time : theta(a*b - max(a,b))
    // Best case time: theta(1)
    pub fn naive(a:u32, b:u32){
        // let mut r = std::cmp::max(a,b);
        let mut r = a.max(b);
        loop {
            if (r % a == 0) && (r % b == 0) {
                println!("{}",r);
                return
            }
            r = r + 1;        
        }
    }

    // Use: a*b = gcd(a,b) * lcm (a,b)
    //
    pub fn efficient(a:u32, b:u32) {
        let result = a * b / (super::gcd_optimal(a,b));
        println!("{}",result);
    
    }



}