/* --------------- Find the Smallest Divisor Given a Threshold -------------- */
// Problem Statement: You are given an array of integers ‘arr’ and an integer i.e. a threshold value ‘limit’. 
// Your task is to find the smallest positive integer divisor, 
// such that upon dividing all the elements of the given array by it, 
// the sum of the division’s result is less than or equal to the given threshold value.

// Input Format: N = 5, arr[] = {1,2,3,4,5}, limit = 8
// Result: 3

// NOTE:
// While dividing the array elements with a chosen number, 
// we will always take the ceiling value. And then we will consider their summation. 
// For example, 3 / 2 = 2.
/* ------------------------------------ x ----------------------------------- */

use std::cmp::min;

fn get_smallest_int_divisor(arr:&[u32],threshold_value: u32) -> u32{
    let mut start = arr.iter().min().unwrap().clone();
    let mut end = arr.iter().max().unwrap().clone();
    let mut res = u32::MAX;

    while start <= end {
        let mid = start + (end-start)/2;
        let sum = quotient_sum(arr, mid);
        
        if sum <= threshold_value {
            res = min(res, mid);
            end = mid-1;
        }else{
            start = mid+1;
        }
    }
    res
}

fn quotient_sum(arr:&[u32], val: u32) -> u32{
    let mut total = 0;

    for num in arr{
        let ceiled_val = (num + val -1)/val;
        total += ceiled_val; 
        println!("{}/{} -> {} -> t {}",num, val, ceiled_val, total);
    }     
    total
}

#[cfg(test)]
mod test_find_smallest_int_divisor {
    use super::*;

    #[test]
    fn check_quotient_sum() {
        let arr = [1,2,3,4,5];
        // let res = quotient_sum(&arr, 2);
        // assert_eq!(res,9);

        let res = quotient_sum(&arr, 3);
        assert_eq!(res,7);
    }

    #[test]
    fn bs_solution_ok() {
        let arr = [1,2,3,4,5];
        let res = get_smallest_int_divisor(&arr, 8);
        assert_eq!(res,3);

        let arr = [8,4,2,3];
        let res = get_smallest_int_divisor(&arr, 10);
        assert_eq!(res,2);
    }
}