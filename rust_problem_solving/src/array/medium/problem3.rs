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
//  if array contains a majority element, its occurrence must be > floor(N/2)
//  idea: the number of majority element - number of minority elements > 0
//  
// - keep track of count of elements and element itself for which are counting
// - After traversing the whole array, we will check the element stored in the variable.
//      If the question states that the array must contain a majority element, 
//      the stored element will be that one but if the question does not state so, 
//      then we need to check if the stored element is the majority element or not. 
//      If not, then the array does not contain any majority element.

// Approach: 
// 1. Initialize 2 variables:
//      Count –  for tracking the count of element
//      Element – for which element we are counting
// 2. Traverse through the given array.
//      If Count is 0 then store the current element of the array as Element.
//      If the current element and Element are the same increase the Count by 1.
//      If they are different decrease the Count by 1.
// 3. The integer present in Element should be the result we are expecting 
/* ------------------------------------ x ----------------------------------- */



fn optimal(arr:&[u32]) -> Option<u32>{
    let n = arr.len();

    let mut element = arr[0];
    let mut count = 0;

    // Applying the algorithm
    for i in 0..n{
        if count == 0{
            count+=1;
            element = arr[i]
        }
        else if element == arr[i] {
            count+=1;
        }
        else{
            count-=1;
        }
    }

    // Checking if the stored element is the majority element
    let mut count_2=0;
    for i in 0..n{
        if arr[i] == element {
            count_2+=1;
        }
    }
    if count_2 > n/2{
        return Some(element);
    }

    None

}



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
    fn optimal_works_ok() {
        let arr = [4, 4, 2, 4, 3, 4, 4, 3, 2, 4];
        assert_eq!(optimal(&arr), Some(4));

        let arr = [2, 2, 1, 1, 1, 2, 2];
        assert_eq!(optimal(&arr), Some(2));
    }

    #[test]
    fn better_works_ok() {
        let arr = [4, 4, 2, 4, 3, 4, 4, 3, 2, 4];
        assert_eq!(better(&arr), Some(4));

        let arr = [2, 2, 1, 1, 1, 2, 2];
        assert_eq!(better(&arr), Some(2));
    }
}
