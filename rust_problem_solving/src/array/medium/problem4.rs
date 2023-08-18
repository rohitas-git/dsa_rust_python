/* ---------- Kadane’s Algorithm : Maximum Subarray Sum in an Array --------- */

// Problem Statement:
// Given an integer array arr, find the contiguous subarray (containing at least one number) which
// has the largest sum and returns its sum and prints the subarray.

// Example 1:
// Input: arr = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6 [4,-1,2,1]

/* -------------------------------- Approach -------------------------------- */

/* ------------------------------------ x ----------------------------------- */

use std::collections::BTreeMap;

fn brute(arr: &[i32]) {
    let n = arr.len();
    let map: BTreeMap<i32, usize> = BTreeMap::new();
    let mut prev = 0;
    let mut k = 0;
    let mut id = (0, 0);

    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += arr[j];   
            if sum > prev {
                prev = sum;
                id = (i, j);
            }
        }
    }

    println!("{:?}", &arr[id.0..(id.1)]);
    println!("{}", prev);
    // return
}

fn better2(arr: &[i32]) -> {
    let n = arr.len();
    let mid = n/2;

    let left = &arr[0..mid];
    let right = &arr[mid..n];

    let answ = better2(left) + better2(arr)
}

fn better(arr: &[i32]) {
    let n = arr.len();

    let mut left = 0;
    let mut right = n - 1;
    let mut total = 0;
    
    for i in 0..n{
        total+=arr[i];
    }

    let mut left_sum = 0;
    let mut right_sum = 0;

    while left <=right {
        left_sum+=arr[left];
        right_sum+=arr[right];



    }

    while right < n {}
}

#[cfg(test)]
mod test_max_subarray_sum {
    use super::*;

    #[test]
    fn brute_ok() {
        let arr1 = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        brute(&arr1);

        let arr2 = [1, 2, 7, -4, 3, 2, -10, 9, 1];
        brute(&arr2);
    }
}
