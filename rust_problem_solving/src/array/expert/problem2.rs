/* --------------- 4 Sum | Find Quads that add up to a target value -------------- */

// Problem Statement:
// Problem Statement:
// Given an array of N integers,
// your task is to find unique quads that add up to give a target value.
//
// In short, you need to return an array of all the unique quadruplets
// [arr[a], arr[b], arr[c], arr[d]] such that their sum is equal to a given target.

// 0 <= a, b, c, d < n
// a, b, c, and d are distinct.
// arr[a] + arr[b] + arr[c] + arr[d] == target

// Example 1:
// Input Format: arr[] = [1,0,-1,0,-2,2], target = 0
// Result: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]

// Example 2:
// Input Format: arr[] = [4,3,3,4,4,2,1,2,1,1], target = 9
// Result: [[1,1,3,4],[1,2,2,4],[1,2,3,3]]

/* -------------------------------- Approach -------------------------------- */

// Brute:
// check all possible triplets using 4 loops
// and find those whose sum is equal to the given target

// Better:
// Reduce a loop by using hashet to lookup the 4th element such that sum of quadruplet = target
// In set, store all the elements between the indices j and k in a HashSet
// and then we will search for the third element in the HashSet.

// Optimal:
// Intend to get rid of set and lookup data structure
// Here, as we are dealing with quads instead of triplets,
// we will fix 2 pointers and the rest of the 2 pointers will be moving.

/* ------------------------------- Complexity ------------------------------- */

// Brute:
// Time: O(N^4 * log(no. of unique quadruplets))
//      {inserting quadruplets into the set - O(log(no. of unique quadruplets))}
// Space: O(no. of the unique quadruplets)

// Better:
// Time: O(N^3 * log(no. of unique quadruplets))
//      {inserting quadruplets into the set - O(log(no. of unique quadruplets))}
// Space: O(no. of the unique quadruplets)

// Optimal:
// Time: O(NlogN)+O(N^3)
// Space: O(no.of quadruplets)

/* -------------------------------- Solution -------------------------------- */
use std::{cmp::Ordering, collections::HashSet};

fn optimal_4sum(arr: &[i32], target: i32) -> Vec<[i32; 4]> {
    let n = arr.len();
    let mut soln: Vec<[i32; 4]> = Vec::new();

    // sorting the array
    let mut arr = arr.to_vec();
    arr.sort();

    for i in 0..n {
        // skip duplicates
        if i != 0 && arr[i] == arr[i - 1] {
            continue;
        }

        for j in (i + 1)..n {
            // skip duplicates
            if j != 0 && arr[j] == arr[j - 1] {
                continue;
            }
            // moving 2 pointers
            let mut k = j + 1;
            let mut l = n - 1;
            while k < l {
                let total_sum = arr[i] + arr[j] + arr[l] + arr[l];
                match total_sum.cmp(&target) {
                    Ordering::Greater => l -= 1,
                    Ordering::Less => k += 1,
                    Ordering::Equal => {
                        let tmp = [arr[i], arr[j], arr[k], arr[l]];
                        soln.push(tmp);
                        k += 1;
                        l -= 1;

                        // skip the duplicates:
                        while k < l && arr[k] == arr[k - 1] {
                            k += 1
                        }
                        while k < l && arr[l] == arr[l + 1] {
                            l -= 1
                        }
                    }
                }
            }
        }
    }
    soln
}

/// 4  are elements whose sum is equal to target
fn better_4sum(arr: &[i32], target: i32) -> HashSet<[i32; 4]> {
    let n = arr.len();
    let mut set: HashSet<[i32; 4]> = HashSet::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let mut lookup: HashSet<i32> = HashSet::new();
            for k in (j+1)..n{
                let fourth = target - arr[i] - arr[j] - arr[k];
                if lookup.contains(&fourth) {
                    let mut tmp = [arr[i], arr[j], arr[k], fourth];
                    tmp.sort();
                    set.insert(tmp);
                }
                lookup.insert(arr[k]);
            }
        }
    }

    set
}

/// quadruplets are index of elements whose sum is equal to target
fn brute_4sum(arr: &[i32], target: i32) -> HashSet<[i32; 4]> {
    let n = arr.len();
    let mut set: HashSet<[i32; 4]> = HashSet::new();

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    if arr[i] + arr[j] + arr[k] + arr[l] == target {
                        let mut tmp = [arr[i], arr[j], arr[k], arr[l]];
                        tmp.sort();
                        set.insert(tmp);
                    }
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

    // let ARR1: [i32; 6] = [1, 0, -1, 0, -2, 2];
    // let ARR2: [i32; 10] = [4, 3, 3, 4, 4, 2, 1, 2, 1, 1];
    // let SOL1: Vec<[i32; 4]> = vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]];
    // let SOL2: Vec<[i32; 4]> = vec![[1, 1, 3, 4], [1, 2, 2, 4], [1, 2, 3, 3]];

    #[test]
    fn optimal_soln() {
        let arr1: [i32; 6] = [1, 0, -1, 0, -2, 2];
        let arr2: [i32; 10] = [4, 3, 3, 4, 4, 2, 1, 2, 1, 1];
        let sol1: Vec<[i32; 4]> = Vec::from([[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]]);
        let sol2: Vec<[i32; 4]> = Vec::from([[1, 1, 3, 4], [1, 2, 2, 4], [1, 2, 3, 3]]);

        assert_eq!(sol1, optimal_4sum(&arr1, 0));
        assert_eq!(sol2, optimal_4sum(&arr2, 9));
    }

    #[test]
    fn better_soln() {
        let arr1: [i32; 6] = [1, 0, -1, 0, -2, 2];
        let arr2: [i32; 10] = [4, 3, 3, 4, 4, 2, 1, 2, 1, 1];
        let sol1: HashSet<[i32; 4]> = HashSet::from([[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]]);
        let sol2: HashSet<[i32; 4]> = HashSet::from([[1, 1, 3, 4], [1, 2, 2, 4], [1, 2, 3, 3]]);
        assert_eq!(sol1, better_4sum(&arr1, 0));
        assert_eq!(sol2, better_4sum(&arr2, 9));
    }

    #[test]
    fn brute_soln() {
        let arr1: [i32; 6] = [1, 0, -1, 0, -2, 2];
        let arr2: [i32; 10] = [4, 3, 3, 4, 4, 2, 1, 2, 1, 1];
        let sol1: HashSet<[i32; 4]> = HashSet::from([[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]]);
        let sol2: HashSet<[i32; 4]> = HashSet::from([[1, 1, 3, 4], [1, 2, 2, 4], [1, 2, 3, 3]]);

        assert_eq!(sol1, brute_4sum(&arr1, 0));
        assert_eq!(sol2, brute_4sum(&arr2, 9));
    }
}
