/* -------- Find the Majority Element that occurs more than N/2 times ------- */
// Problem Statement:
// Given an array of N integers,
// write a program to return an element that occurs more than N/2 times in the given array.
// You may consider that such an element always exists in the array.

// Example:
//
// Input Format: nums[] = {2,2,1,1,1,2,2}
// Result: 2
//
// Input Format: nums[] = {4,4,2,4,3,4,4,3,2,4}
// Result: 4

/* -------------------------------- Approach -------------------------------- */

// 1. Better:
//
// Intuition:
// Use a better data structure to reduce the number of look-up operations and hence the time complexity.
// Moreover, we have been calculating the count of the same element again and again
// – so we have to reduce that also.
//
// O(n) + O(k), k = n - duplicates
// While fill HashMap < num, freq >, check if freq >
//
//

// 2. Optimal:
//  
// Use Moore’s Voting Algorithm:
// 
// 

/* ------------------------------------ x ----------------------------------- */

use std::{collections::HashMap, ops::Add};

fn better(arr: &[u32]) -> Option<u32> {
    let n = arr.len();
    let half = (n / 2) as u32;
    let mut storage: HashMap<u32, u32> = HashMap::new();

    for i in 0..n {
        let mut store = storage.get(&arr[i]);

        if store == None {
            storage.insert(arr[i], 1);
        } else {
            let freq = storage.get_mut(&arr[i]).unwrap().clone();
            storage.insert(arr[i], freq + 1);

            let count = storage.get(&arr[i]).unwrap().to_owned();
            if storage.get(&arr[i]).unwrap().to_owned() > half {
                println!("Returning");
                return Some(arr[i]);
            }
        }
    }
    None
}

#[cfg(test)]
mod test_find_majority_in_arr {
    use super::*;

    #[test]
    fn better_works_ok() {
        let arr = [4, 4, 2, 4, 3, 4, 4, 3, 2, 4];
        assert_eq!(better(&arr), Some(4));

        let arr = [2, 2, 1, 1, 1, 2, 2];
        assert_eq!(better(&arr), Some(2));
    }
}
