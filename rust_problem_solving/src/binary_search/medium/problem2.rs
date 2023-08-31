/* ---------------- Nth Root of a Number using Binary Search ---------------- */
// Problem Statement: Given two numbers N and M, find the Nth root of M. 
// The nth root of a number M is defined as a number X when raised to the power N equals M. 
// If the ‘nth root is not an integer, return -1.

/* ------------------------------------ x ----------------------------------- */
enum Status{
    Larger,
    Equal,
    Smaller
}
use Status::*;

// Time - O(logM * N)
fn optimal_with_bs(target:u32, n: u32) -> Option<u32>{
    let mut low =1;
    let mut high = target;

    if target == 0{
        return None;
    }

    while low <= high {
        let mid = low + (high-low)/2;
        
        match position_of_powered(mid, n, target) {
            Equal =>  {return Some(mid);},
            Smaller => {low = mid+1;},
            Larger => {high = mid-1;}
        }
    }
    None
}

// Time - O(N)
fn position_of_powered(mid:u32, n:u32, target:u32) -> Status{
    let mut val=1;

    for i in 1..(n+1){
        val *= mid;
        if val > target{
            return Larger;
        }
    }
    if val == target{
        return Equal;
    }
    Smaller
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod test_nth_root {
    use super::*;

    #[test]
    fn optimal_with_bs_ok() {
        
        let res = optimal_with_bs(1024, 10);
        assert_eq!(res, Some(2));

        let res = optimal_with_bs(25*25, 4);
        assert_eq!(res, Some(5));

        let res = optimal_with_bs(8, 3);
        assert_eq!(res, Some(2));
    
        let res = optimal_with_bs(125, 4);
        assert_eq!(res, None);

        let res = optimal_with_bs(0, 4);
        assert_eq!(res, None);
    }
}

