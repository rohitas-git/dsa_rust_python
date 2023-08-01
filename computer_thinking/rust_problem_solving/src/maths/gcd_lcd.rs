
// Time( log_min(a,b) + ? )
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