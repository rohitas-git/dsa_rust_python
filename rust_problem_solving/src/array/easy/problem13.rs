/* -------------- Longest Subarray with given Sum K (Positives and Negatives) -------------- */
// Problem Statement:
// Given an array and a sum k,
// we need to print the length of the longest subarray that sums to k.

// Example 2:
// Input Format: N = 7, k = 3, array[] = [1,2,3,1,1,1,1];
// Result: 3

/* -------------------------------- Approach -------------------------------- */
// naive: time O(n^2), space O(1)
//
// Optimal for + - Sum : using hashMap
// Using hashMap to store prefix Sums <X,id>,
// find if prefixSum such that currPrefix - K is present in Map

// Optimal for + Sum: using two pointer
// We are using two pointers i.e. left and right.
// The left pointer denotes the starting index of the subarray and the right pointer denotes the ending index.
/* ------------------------------- Complexity ------------------------------- */

// Optimal for + - Sum:
// time - O(N) if using unordered Map / HashMap     [.get - O(1)~]
//      - O(N logN) if using ordered Map / BTreeMap [.get - O(logN)]
// space - O(N)

// Optimal for + Sum:
// time - O(2*N)
// space - O(1)
/* ------------------------------------ x ----------------------------------- */

fn naive(arr: &[u32], sum: u32) -> u32 {
    let n = arr.len();
    let mut length = 0;

    for i in 0..n {
        let mut s = 0;

        for j in i..n {
            s += arr[j];

            if s == sum {
                length = std::cmp::max(length, j - i + 1);
            }
        }
    }

    length as u32
}

use std::collections::{BTreeMap, HashMap};

// 1. Find prefixSum x for each index i as we iterate the array
// 2. Find sub-array whose sum is k and ends at i and thus, find remaining sub-array sum = X-K in Map
// 3. Save <prefixSum, i> if prefixSum already not in Map
// After iterating the array, we get len of longest sub-array which sums to K
// Note: to_sum can be positive or negative
fn optimal_for_sum_positive_negative(arr: &[i32], to_sum: i32) -> usize {
    let n = arr.len();
    let mut prefix_sum_map: HashMap<i32, usize> = HashMap::new();

    let mut prefix_sum = 0;
    let mut longest = 0;

    for i in 0..n {
        prefix_sum += arr[i];

        if prefix_sum == to_sum {
            longest = std::cmp::max(longest, i + 1);
        }

        // search of sub-array ending with index i, whose sum equals to_sum
        let remains = prefix_sum - to_sum;
        if prefix_sum_map.get(&remains) != None {
            let len = i - prefix_sum_map.get(&remains).unwrap();
            longest = std::cmp::max(longest, len);
        }

        // to ensure the minimize the prefSumMap[x-k] in i - prefSumMap[x-k]
        // to avoid edge case where key remains same but value increases
        if prefix_sum_map.get(&prefix_sum) == None {
            prefix_sum_map.insert(prefix_sum, i);
        }
    }
    longest
}

// 1. Sub-array is defined by left and right index pointers, with initially at 0 and sum = arr[0]
// 2. Subtract leftmost element from sum and move left ptr +1 till sum<k or left>right
// 3. If sum = k, decide max btw prev maxLen or right-left+1
// 4. Move right ptr ++ and if right < n, add rightmost element to sum
// 5. Do 2,3,4 steps till right >= n
// Note: k can only be positive
fn optimal_for_sum_positive(arr: &[i32], k: i32) -> usize {
    let n = arr.len();

    let (mut left, mut right) = (0, 0);
    let mut sum = arr[0];
    let mut max_len = 0;

    while right < n {
        while left <= right && sum > k {
            sum -= arr[left];
            left += 1;
        }

        if sum == k {
            max_len = std::cmp::max(max_len, right - left + 1);
        }

        right += 1;
        if right < n {
            sum += arr[right];
        }
    }

    max_len
}

#[cfg(test)]
mod test_longest_subarray_with_sum_k {
    use super::*;

    #[test]
    fn sum_positive_negative_ok() {
        let arr = [1, 2, 3, 1, 1, 1, 1];
        assert_eq!(optimal_for_sum_positive_negative(&arr, 3), 3);

        let arr = [2, 0, 0, 3];
        assert_eq!(optimal_for_sum_positive_negative(&arr, 3), 3);

        let arr = [2, -1, -1, -1, -5, 10];
        assert_eq!(optimal_for_sum_positive_negative(&arr, -6), 5); // [2,-1,-1,-1,5].sum == 6
    }

    #[test]
    fn sum_positive_ok() {
        let arr = [1, 2, 3, 1, 1, 1, 1];
        assert_eq!(optimal_for_sum_positive(&arr, 3), 3);

        let arr = [2, 0, 0, 3];
        assert_eq!(optimal_for_sum_positive(&arr, 3), 3);

        let arr = [2, -1, -1, -1, -5, 10];
        assert_eq!(optimal_for_sum_positive(&arr, -6), 5); // [2,-1,-1,-1,5].sum == 6
    }
}
