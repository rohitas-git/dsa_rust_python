/* --------------------------- Problem Statement: --------------------------- */
// Problem Statement: 
// Given an array, and an element num the task is to find if num is present
// in the given array or not. If present print the index of the element or print -1.


/* -------------------------------- Approach: ------------------------------- */
// 1. LINEAR SEARCH
// 2.
//

/* ------------------------------- Complexity ------------------------------- */
// 1. tO(n), sO(1)

/* ------------------------------------ x ----------------------------------- */

fn linear_search_for(num:u32, arr:&[u32])-> Option<u32>{
    let n = arr.len();
    if n == 0{
        return  None;
    }
    
    for i in 0..n{
        if arr[i] == num{
            return Some(i as u32);
        }
    }
    None 
}

#[cfg(test)]
mod test_linear_search {
    use super::*;

    #[test]
    fn test_some() {
        let arr = [1,2,3,4,5]; 
        let r = linear_search_for(2, &arr);
        assert_eq!(r, Some(1));
    }

    #[test]
    fn test_none() {
        let arr = [1,2,3,4,5]; 
        let num = 11;
        let r = linear_search_for(num, &arr);
        assert_eq!(r, None);
    }
}
