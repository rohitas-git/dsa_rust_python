/* -------------- Longest Subarray with given Sum K(Positives and Negatives) -------------- */
// Problem Statement:
// Given an array and a sum k,
// we need to print the length of the longest subarray that sums to k.

// Example 2:
// Input Format: N = 7, k = 3, array[] = [1,2,3,1,1,1,1];
// Result: 3

/* -------------------------------- Approach -------------------------------- */
// naive time O(n^2), space O(1)
//
// Better :
// Using hashMap to store prefix Sums <X,id>,
// find prefixSum such that currPrefix - K is present in Map

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

use std::collections::BTreeMap;
fn better(arr: &[u32], sum: u32) {
    let n = arr.len();
    let mut prefix_sum_map: BTreeMap<u32, usize> = BTreeMap::new();

    let mut prefix_sum = 0;
    let mut longest = 0;

    for i in 0..n {
        prefix_sum += arr[i];
        prefix_sum_map.insert(prefix_sum, i);

        if prefix_sum_map.get(prefix_sum - sum) != None {}
    }
}

fn solution_1(arr: &[u32], sum: u32) -> u32 {
    let mut len_sub_once = 0;
    let mut max_len = 0;

    for i in 0..arr.len() {
        if arr[i] == sum && max_len == 0 {
            max_len = 1;
            continue;
        } else {
        }
    }

    println!(" Longest Subarray length: {} ", max_len);
    max_len
}

fn solution_2(arr: &[u32], sum: u32) {}

#[cfg(test)]
mod test_longest_subarray_with_sum_k {
    use super::*;

    #[test]
    fn found_it() {
        let arr = [1, 2, 3, 1, 1, 1, 1];
        assert_eq!(solution_1(&arr, 3), 3);
    }
}
