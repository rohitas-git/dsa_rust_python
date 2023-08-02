
// Optimized Euclidean Algorithm
// Time: 
pub fn gcd_optimal(mut a:u32, mut b:u32)->u32{
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
    
}

// Optimized Euclidean Algorithm (Recursive)
// Time: 
pub fn gcd_optimal_recursive(a:u32, b:u32)->u32{
    if b == 0 {
        return a;
    }
    
    return gcd_optimal_recursive(b, a % b);
    
}

// Euclidean Algorithm
// Time: O(min(a,b))
pub fn gcd_better(mut a: u32, mut b: u32) -> u32{
    while a != b{

        if a > b{
            a = a - b;
        }
        else {
            b = b -a ;
        }
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