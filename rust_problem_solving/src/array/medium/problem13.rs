/* ----------------------- Count Subarray sum Equals K ---------------------- */
// Problem Statement:
// Given an array of integers and an integer k,
// return the total number of subarrays whose sum equals k.

// Input Format: N = 4, array[] = {3, 1, 2, 4}, k = 6
// Result: 2
// Explanation: The subarrays that sum up to 6 are [3, 1, 2] and [2, 4].

/* -------------------------------- Approach -------------------------------- */

// Brute:
// Intuition: We will check the sum of every possible subarray and count how many of them are equal to k.
// To get every possible subarray sum, we will be using three nested loops.

// Better:
// Intuition: If we carefully observe, we can notice that to get the sum of the current subarray
// we just need to add the current element(i.e. arr[j]) to the sum of the previous subarray i.e. arr[i….j-1].

// Optimal:
// In this approach, we are going to use the concept of the prefix sum to solve this problem.
// Here, the prefix sum of a subarray ending at index i simply means the sum of all the elements of that subarray.
// Now, for a subarray ending at index i with the prefix sum x, if we remove the part with the prefix sum x-k,
// we will be left with the part whose sum is equal to k. And that is what we want.

/* ------------------------------------ x ----------------------------------- */

use std::collections::HashMap;
// 1. Find prefixSum x for each index i as we iterate the array
// 2. Count the times that sub-array whose sum is k and ends at i occurs and thus, find count of times that remaining sub-array sum = X-K in Map
// 3. Increase count of prefixSum x in HashMap +1
// After iterating the array, we get count of subarray that have sum = k
// Note: to_sum can be positive or negative
fn count_subarrays_for_sum(arr: &[i32], to_sum: i32) -> usize {
    let n = arr.len();
    let mut prefix_sum = 0;
    let mut count = 0;

    // Key - Value of prefixSum, Value - Number of times it occurs
    let mut prefix_sum_map: HashMap<i32, usize> = HashMap::new();
    // Setting of 0 in the map for case x-k = 0
    prefix_sum_map.insert(0, 1);

    for i in 0..n {
        prefix_sum += arr[i];

        let remains = prefix_sum - to_sum;
        count += prefix_sum_map.get(&remains).unwrap_or(&0usize);

        let mut prev = prefix_sum_map.get(&prefix_sum).unwrap_or(&0usize);
        prefix_sum_map.insert(prefix_sum, (prev + 1));
    }
    count
}

#[cfg(test)]
mod test_couting_subarrays_with_given_sum {
    use super::*;

    #[test]
    fn test_solution() {
        let arr = [3, 1, 2, 4];
        assert_eq!(count_subarrays_for_sum(&arr, 6), 2);

        let arr = [1, 2, 3, 1, 1, 1, 1];
        assert_eq!(count_subarrays_for_sum(&arr, 3), 4);

        let arr = [2, 0, 0, 3];
        assert_eq!(count_subarrays_for_sum(&arr, 3), 3);

        let arr = [2, -1, -1, -1, -5, 10];
        assert_eq!(count_subarrays_for_sum(&arr, 2), 2);
    }
}
