/* --------------------- Minimum in Rotated Sorted Array -------------------- */
// Problem Statement:
// Given an integer array arr of size N, sorted in ascending order (with distinct values).
// Now the array is rotated between 1 to N times which is unknown.
// Find the minimum element in the array.

// Input Format: arr = [4,5,6,7,0,1,2,3]
// Result: 0

// Note: Assuming clockwise rotation of sorted array

/* -------------------------------- Approach -------------------------------- */
// Brute: Linear Search

// Better: Binary Search [O(logN), O(1)]
// Observe that: 
// [Left Sorted Half] [Right Sorted Half]
// ex: [5,6,7,1,2,3] = [5,6,7] [1,2,3]
// 
// Mid lies in either of halves. The minimum lies in right sorted half since the array is rotated clockwise
// 0. min = minimum(min, current)
// While loop over 1&2: 
// 1. if current >= arr[high] : mid in left sorted half -> low = mid+1 if mid != (n-1) else break
// 2. if current < arr[high] : mid in right sorted half -> high = mid-1 if mid != 0 else break


// Optimal: Optimized binary search
// Addition of If arr[low] <= arr[high]: In this case, the array from index low to high is completely sorted. 
// Therefore, we can simply select the minimum element, arr[low],

/* ------------------------------------ x ----------------------------------- */
use std::cmp::min as minimum;

fn optimal_solution(arr: &[u32]) -> u32 {
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;
    let mut min = arr[0];

    if arr[low] <= arr[high]{
        min = arr[low];
        return min;
    }

    while low <= high {
        let mid = low + (high - low) / 2;
        let current = arr[mid];
        min = minimum(min, current);
        if current >= arr[high] {
            if mid != (n - 1){
                low = mid + 1;
            } else {
                break;
            }
        } 
        else {
            if mid != 0{
                high = mid - 1;
            }else{
                break;
            }
        } 
    }
    min
}

#[cfg(test)]
mod test_getting_minimum_in_rotated_sorted_arr {
    use super::*;

    #[test]
    fn optimal_solution_works() {
        let arr = [4,5,6,7,1,2,3];
        let res = optimal_solution(&arr);
        assert_eq!(res, 1);

        let arr = [4, 5, 6, 7, 1];
        let res = optimal_solution(&arr);
        assert_eq!(res, 1);

        let arr = [1, 2, 4, 5, 6, 7];
        let res = optimal_solution(&arr);
        assert_eq!(res, 1);
    }
}
