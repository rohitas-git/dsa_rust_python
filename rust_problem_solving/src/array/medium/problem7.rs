/* --- next_permutation : find next lexicographically greater permutation --- */

// Problem Statement:
// A permutation of an array of integers is an arrangement of its members into a sequence or linear order.
// For example, for arr = [1,2,3],
// the following are all the permutations of arr: [1,2,3], [1,3,2], [2, 1, 3], [2, 3, 1], [3,1,2], [3,2,1].

// Given an array Arr[] of integers, rearrange the numbers of the given array into
// the lexicographically next greater permutation of numbers.
//
// If such an arrangement is not possible,
// it must rearrange to the lowest possible order (i.e., sorted in ascending order).

// Input format: Arr[] = {1,3,2}
// Output: Arr[] = {2,1,3}

// Input format: Arr[] = {3,2,1}
// Output: Arr[] = {1,2,3}

/* ------------------------------------ x ----------------------------------- */

fn brute(arr: &[u32]) -> &[u32] {
    let n = arr.len();

    // let perms = arr.clone(

    arr
}
