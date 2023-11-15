/* --------------- 3 Sum : Find triplets that add up to a zero -------------- */

// Problem Statement:
// Given an array of N integers, your task is to find unique triplets
// that add up to give a sum of zero.
//
// In short, you need to return an array of all the unique triplets
// [arr[a], arr[b], arr[c]] such that i!=j, j!=k, k!=i, and their sum is equal to zero.

// Example 1:
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1], [0,1,-1]]

// Example 2:
// Input: nums=[-1,0,1,0]
// Output: Output: [[-1,0,1],[-1,1,0]]

/* -------------------------------- Approach -------------------------------- */

// Brute:
// check all possible triplets using 3 loops
// and find those whose sum is equal to the given target

// Better:
// Reduce a loop by using hashet to lookup the 3rd element in triplet
// In set, store all the elements between the indices i and j in a HashSet
// and then we will search for the third element in the HashSet.

// Optimal:
// Intend to get rid of two things
// i.e. the HashSet we were using for the look-up operation
// and the set data structure used to store the unique triplets.
// - the set data structure was basically storing the unique triplets in sorted order
// - the HashSet was used to search for the third element.
//
// That is why, we will first sort the entire array,
// and then to get the unique triplets,
// we will simply skip the duplicate numbers while moving the pointers.
//
// So, We will first sort the array.
// Then, we will fix a pointer i, and the rest 2 pointers j and k will be moving.

/* ------------------------------- Complexity ------------------------------- */

// Brute:
// Time: O(N^3 * log(no. of unique triplets))
//      {inserting triplets into the set - O(log(no. of unique triplets))}
// Space: O(no. of the unique triplets)

// Better:
// Time: O(N^2 * log(no. of unique triplets))
//      {inserting triplets into the set - O(log(no. of unique triplets))}
// Space: O(no. of the unique triplets)

// Optimal:
// Time: O(NlogN)+O(N2)
// Space: O(no.of triplets)

/* -------------------------------- Solution -------------------------------- */
use std::{cmp::Ordering, collections::HashSet};

fn optimal_3sum(arr: &[i32]) -> Vec<[i32; 3]> {
    let n = arr.len();
    let mut soln: Vec<[i32; 3]> = Vec::new();
    let target = 0;

    // sorting the array
    let mut arr = arr.to_vec();
    arr.sort();

    for i in 0..n {
        // skip duplicates
        if i != 0 && arr[i] == arr[i - 1] {
            continue;
        }

        // moving 2 pointers
        let mut j = i + 1;
        let mut k = n - 1;
        while j < k {
            let total_sum = arr[i] + arr[j] + arr[k];
            match total_sum.cmp(&target) {
                Ordering::Greater => k -= 1,
                Ordering::Less => j += 1,
                Ordering::Equal => {
                    let tmp = [arr[i], arr[j], arr[k]];
                    soln.push(tmp);
                    j += 1;
                    k -= 1;

                    // skip the duplicates:
                    while j < k && arr[j] == arr[j - 1] {
                        j += 1
                    }
                    while j < k && arr[k] == arr[k + 1] {
                        k -= 1
                    }
                }
            }
        }
    }
    soln
}

/// 3 triplets are elements whose sum is equal to target
fn better_3sum(arr: &[i32]) -> HashSet<[i32; 3]> {
    let n = arr.len();
    let mut set: HashSet<[i32; 3]> = HashSet::new();
    let target = 0;

    for i in 0..n {
        let mut lookup: HashSet<i32> = HashSet::new();
        for j in (i + 1)..n {
            let third = target - arr[i] - arr[j];
            if lookup.contains(&third) {
                let mut tmp = [arr[i], arr[j], third];
                tmp.sort();
                set.insert(tmp);
            }
            lookup.insert(arr[j]);
        }
    }

    set
}

/// 3 triplets are index of elements whose sum is equal to target
fn brute_3sum(arr: &[i32]) -> HashSet<[usize; 3]> {
    let n = arr.len();
    let mut set: HashSet<[usize; 3]> = HashSet::new();
    let target = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if arr[i] + arr[j] + arr[k] == target {
                    let mut tmp = [i, j, k];
                    tmp.sort();
                    set.insert(tmp);
                }
            }
        }
    }

    set
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod test_solutions {
    use super::*;

    #[test]
    fn optimal_soln() {
        let arr = [-1, 0, 1, 2, -1, -4];
        let soln = Vec::from([[-1, -1, 2], [-1, 0, 1]]);
        assert_eq!(soln, optimal_3sum(&arr));

        let arr = [-1, 0, 1, 0];
        let soln = Vec::from([[-1, 0, 1]]);
        assert_eq!(soln, optimal_3sum(&arr));
    }

    #[test]
    fn better_soln() {
        let arr = [-1, 0, 1, 2, -1, -4];
        let soln = HashSet::from([[-1, -1, 2], [-1, 0, 1]]);
        assert_eq!(soln, better_3sum(&arr));

        let arr = [-1, 0, 1, 0];
        let soln = HashSet::from([[-1, 0, 1]]);
        assert_eq!(soln, better_3sum(&arr));
    }

    #[test]
    fn brute_soln() {
        let arr = [-1, 0, 1, 2, -1, -4];
        let soln = HashSet::from([[0, 3, 4], [0, 1, 2], [1, 2, 4]]);
        assert_eq!(soln, brute_3sum(&arr));

        let arr = [-1, 0, 1, 0];
        let soln = HashSet::from([[0, 1, 2], [0, 2, 3]]);
        assert_eq!(soln, brute_3sum(&arr));
    }
}
