/* --------------------------- Two Sum --------------------------- */
// Check if a pair with given sum exists in Array
// Problem Statement: Given an array of integers arr[] and an integer target.

// Example 1:
// Input Format: N = 5, arr[] = {2,6,5,8,11}, target = 14
// Result: [1, 3]

// Example 2:
// Input Format: N = 5, arr[] = {2,6,5,8,11}, target = 15
// Result: None

/* -------------------------------- Approach: ------------------------------- */
// 1. Better: Using hashmap
//
// 2. Optimal:
// First sort the array and try to choose numbers in a greedy manner
//      left++ if sum > target, right++ sum <target
//

/* ------------------------------- Complexity ------------------------------- */
// brute: time O(N^2)
// better: time O(N) space O(N)
//
// optimal:  [using mergeSort to sort in my solution]
//  time O(N) + O(N logN)
//  space O(N)
/* ------------------------------------ x ----------------------------------- */

use std::collections::HashMap;

fn brute(arr: &[u32], target: u32) -> Option<(usize, usize)> {
    let n = arr.len();

    for i in 0..n {
        let other = target - (arr[i] as u32);
        for j in 0..n {
            if arr[j] == other && i != j {
                return Some((i, j));
            }
        }
    }
    None
}

fn better(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    let n = arr.len();
    let mut store: HashMap<i32, usize> = HashMap::new();

    let mut sum = 0;

    for i in 0..n {
        let other: i32 = (target - arr[i]);
        if store.get(&other) != None {
            let j = store.get(&other).unwrap().clone();
            return Some((i, j));
        }
        store.insert(arr[i], i);
    }

    None
}
// But if in the interview, we are not allowed to use the map data structure,
// then we should move on to the following approach i.e. two pointer approach.
// This approach will have the same time complexity as the better approach.

use crate::merge_sort::do_sort;

fn optimal(arr: &mut [u32], target: u32) -> Option<(u32, u32)> {
    let n = arr.len();
    do_sort(arr);

    let mut left = 0;
    let mut right = n - 1;

    while left < right {
        let sum = arr[left] + arr[right];

        if sum < target {
            left += 1;
        } else if sum > target {
            right -= 1;
        } else {
            return Some((arr[left], arr[right]));
        }
    }
    None
}

#[cfg(test)]
mod test_find_pair_with_target_sum {
    use super::*;

    #[test]
    fn optimal_ok() {
        let mut arr = [2, 8, 9, 11, 5, 3];
        let target = 14;
        assert_eq!(optimal(&mut arr, target), Some((3, 11)));

        let mut arr = [2, 8, 9, 11, 5, 3];
        let target = 16;
        assert_eq!(optimal(&mut arr, target), Some((5, 11)));

        let mut arr = [2, 8, 9, 11, 5, 3];
        let target = 11;
        assert_eq!(optimal(&mut arr, target), Some((2, 9)));
    }

    #[test]
    fn brute_ok() {
        let arr = [2, 8, 9, 11, 5, 3];
        let target = 14;
        assert_eq!(brute(&arr, target), Some((2, 4)));

        let arr = [2, 8, 9, 11, 5, 3];
        let target = 16;
        assert_eq!(brute(&arr, target), Some((3, 4)));

        let arr = [2, 8, 9, 11, 5, 3];
        let target = 11;
        assert_eq!(brute(&arr, target), Some((0, 2)));
    }

    #[test]
    fn better_ok() {
        let arr = [2, 8, 9, 11, 5, 3];
        let target = 14;
        assert_eq!(better(&arr, target), Some((4, 2)));

        let arr = [2, 8, 9, -11, 5, 3];
        let target = -6;
        assert_eq!(better(&arr, target), Some((4, 3)));

        let arr = [2, 8, 9, 11, 5, 3];
        let target = 25;
        assert_eq!(better(&arr, target), None);

        let arr = [2, 8, 9, 11, 5, 3];
        let target = 1;
        assert_eq!(better(&arr, target), None);
    }
}
