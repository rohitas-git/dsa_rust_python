/* ---------- Kadane’s Algorithm : Maximum Subarray Sum in an Array --------- */

// Problem Statement:
// Given an integer array arr, find the contiguous subarray (containing at least one number) which
// has the largest sum and returns its sum and prints the subarray.

// Example 1:
// Input: arr = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6 [4,-1,2,1]

/* -------------------------------- Approach -------------------------------- */
// Brute: 
// Have 3 nested loop to calculate sum of sum of every possible subarray and consider the maximum among them.
// 
// Better: 
// here also check the sum of every possible subarray and consider the maximum among them.
// Just put in 1st inner loop and in 2nd loop, just sum += arrItem to get sum for subarray[i..=j]
// 
// Optimal: (Kadane's Algorithm)
// The intuition of the algorithm is not to consider the subarray as a part of the answer if its sum is less than 0. 
// A subarray with a sum less than 0 will always reduce our answer and so 
// this type of subarray cannot be a part of the subarray with maximum sum.
 
// 

/* ------------------------------- Complexity ------------------------------- */
// Brute: time O(N^3), space O(1)
// Better: time theta(N^2), space theta(1)
// Optimal: time O(N), space theta(1)

/* ------------------------------------ x ----------------------------------- */

fn better(arr: &[i32]) {
    let n = arr.len();
    let mut prev = 0;
    let mut result = (0, 0);

    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += arr[j];

            if sum > prev {
                prev = sum;
                result = (i, j);
            }
        }
    }

    println!("{:?}", &arr[(result.0)..=(result.1)]);
    println!("{}", prev);
    // return
}

fn optimal(arr: &[i32]) -> i32{
    let n = arr.len();

    let mut max_sum = i32::MIN;
    let mut sum = 0;
    let mut subarray_start = 0;
    let mut subarray_end = 0;

    for i in 0..n{
        if sum == 0 {
            subarray_start=i;
        }

        sum += arr[i];

        if sum > max_sum{
            max_sum = sum;
            subarray_end = i;
        }
        // not carrying burden of negative sum forward
        if sum < 0 {
            sum = 0;
        }
    } 
    println!("{:?}", &arr[subarray_start..=(subarray_end)]);
    max_sum

}

#[cfg(test)]
mod test_max_subarray_sum {
    use super::*;

    #[test]
    fn optimal_ok() {
        let arr1 = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        println!("{}",optimal(&arr1));

        let arr2 = [1, 2, 7, -4, 3, 2, -10, 9, 1];
        println!("{}",optimal(&arr2));
    }

    #[test]
    fn better_ok() {
        let arr1 = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
        better(&arr1);

        let arr2 = [1, 2, 7, -4, 3, 2, -10, 9, 1];
        better(&arr2);
    }
}
