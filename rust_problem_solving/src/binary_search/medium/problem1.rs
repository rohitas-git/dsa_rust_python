
/* -------------- Finding Sqrt of a number using Binary Search -------------- */
// Problem Statement: 
// You are given a positive integer n. 
// Your task is to find and return its square root. 
// If ‘n’ is not a perfect square, then return the floor value of ‘sqrt(n)’.

/* ------------------------------------ x ----------------------------------- */

fn brute(target:u32) -> u32{
    let mut res = 0;
    
    if target == 0 || target == 1{
        return target;
    }

    let mut i=0;
    while i*i <= target  {
        i+=1;
    }
    res = i-1;
    println!("{}",res);
    res
}

fn optimal(target:u32) -> u32{
    let mut res = 0;

    let mut i = target;

    if target == 0 || target == 1{
        return target;
    }

    while i > 1{
        i=i/2;
        res+=1;
    }
    res
}

// Now, we are not given any sorted array on which we can apply binary search. 
// But, if we observe closely, we can notice that our answer space i.e. [1, n] is sorted. 
// So, we will apply binary search on the answer space.
fn optimal_with_bs(target:u32) -> u32{
    let mut low =1;
    let mut high = target;

    while low <= high {
        let mid = low + (high-low)/2;
        let val = mid*mid;

        if val <= target{
            low = mid+1;
        }
        else {
            high = mid-1;
        }

    }
    high
}

#[cfg(test)]
mod test_sqrt {
    use super::*;

    #[test]
    fn optimal_ok() {
        
        let res = optimal(4);
        assert_eq!(res, 2);

        let res = optimal(16);
        assert_eq!(res, 4);

        let res = optimal(15);
        assert_eq!(res, 3);
    
        let res = optimal(1);
        assert_eq!(res, 1);

        let res = optimal(0);
        assert_eq!(res, 0);
    }

    #[test]
    fn optimal_with_bs_ok() {
        
        let res = optimal_with_bs(4);
        assert_eq!(res, 2);

        let res = optimal_with_bs(16);
        assert_eq!(res, 4);

        let res = optimal_with_bs(15);
        assert_eq!(res, 3);
    
        let res = optimal_with_bs(1);
        assert_eq!(res, 1);

        let res = optimal_with_bs(0);
        assert_eq!(res, 0);
    }

    #[test]
    fn brute_ok() {
        
        let res = brute(4);
        assert_eq!(res, 2);

        let res = brute(16);
        assert_eq!(res, 4);

        let res = brute(15);
        assert_eq!(res, 3);
    
        let res = brute(1);
        assert_eq!(res, 1);

        let res = brute(0);
        assert_eq!(res, 0);
    }
}
