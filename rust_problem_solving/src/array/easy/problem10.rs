/* ------------------- Find the missing number in an array ------------------ */
// Problem Statement:
// Given an integer N and an array of size N-1 containing N-1 numbers between 1 to N.
// Find the number(between 1 to N), that is not present in the given array.

// Example 1:
// Input Format: N = 5, array[] = {1,2,4,5}
// Result: 3




/* -------------------------------- Approach: ------------------------------- */
// 1. Brute Intuition:
//  For each number between 1 to N, we will try to find it in the given array using linear search.
//  And if we don’t find any of them, we will return the number.
//
// 2. Better:
//  Using the hashing technique, we will store the frequency of each element of the given array.
//  Now, the number for which the frequency will be 0, will be returned.
//
// 3. Optimal v1
//  Summation Approach: S2 - S1 = missing number
//
// 4. Optimal v2
// XOR Approach: xor1^xor2 = missing number
//  Note: a^a = 0 && 0^a = a

/* ------------------------------- Complexity ------------------------------- */
// brute : time worst: O(N^2), N = size of array + 1; space O(1)
// better: time O(2*N), space: O(N), N = size of array + 1;
// optimal_v1: time O(N), space O(1) 
// optimal v2: time O(N), space O(1) 
// 
// for_sorted_arrays: time worst/avg: O(n), best: O(1); space O(1)

/* ------------------------------- Note ------------------------------- */
// Among the optimal approaches, the XOR approach is slightly better than the summation one 
// because the term (N * (N+1))/2 used in the summation method cannot be stored in an integer if the value of N is big (like 105). 
// In that case, we have to use some bigger data types. But we will face no issues like this while using the XOR approach.

/* ------------------------------------ x ----------------------------------- */

// works for both sorted and unsorted arrays
fn brute(arr: &[u32]) -> Option<u32> {
    let n = arr.len() + 1;
    let arr_len = n - 1;

    for i in 1..=n {
        // flag variable to check if an element exists
        let mut flag = false;

        // Search the element using linear search:
        for j in 0..arr_len {
            if arr[j] == (i) as u32 {
                // i is present in the array:
                flag = true;
                break;
            }
        }
        // check if the element is missing
        if !flag {
            return Some(i as u32);
        }
    }
    return None;
}

fn better(arr: &[u32]) -> Option<u32> {
    let n = arr.len() + 1;
    let arr_len = n - 1;

    let mut hash_map = vec![0u32; n + 1];

    for i in 0..arr_len {
        hash_map[arr[i] as usize] += 1;
    }

    for i in 1..=n {
        if hash_map[i] == 0 {
            return Some(i as u32);
        }
    }
    return None;
}

fn optimal_v1(arr: &[u32]) -> Option<u32> {
    let n = arr.len() + 1;
    let arr_len = n - 1;

    let natural_sum = n * (n + 1) / 2;
    let mut array_sum = 0;
    for i in 0..arr_len{
        array_sum += arr[i];
    }

    return Some(natural_sum as u32-array_sum);

    None
}

fn optimal_v2(arr: &[u32]) -> Option<u32> {
    let n = arr.len() + 1;
    let arr_len = n - 1;

    let mut natural_xor = 0u32;
    let mut arr_xor = 0u32;

    for i in 0..arr_len{
        natural_xor = natural_xor^( (i+1) as u32) ; 
        arr_xor = arr_xor^(arr[i]);
    }
    natural_xor = natural_xor^(n as u32);

    return Some(natural_xor^arr_xor);
}


fn sorted_array_approach(arr: &[u32]) -> u32 {
    let n = arr.len() + 1;
    let arr_len = n - 1;

    for i in 0..arr_len - 1 {
        if (arr[i + 1] - arr[i]) != 1 {
            return (i + 2) as u32;
        }
    }
    return 0;
}

#[cfg(test)]
mod test_missing_element_in_array {
    use super::*;

    #[test]
    fn brute_ok() {
        let arr = [1, 2, 6, 4, 3, 9, 8, 7];
        assert_eq!(brute(&arr).unwrap(), 5);
    }

    #[test]
    fn better_ok() {
        let arr = [9, 4, 3, 2, 6, 7, 8, 1];
        assert_eq!(better(&arr).unwrap(), 5);
    }

    #[test]
    fn optimal_v1_ok() {
        let arr = [1, 2, 3, 4, 6, 7, 8, 9];
        assert_eq!(optimal_v1(&arr).unwrap(), 5);
    }

    #[test]
    fn optimal_v2_ok() {
        let arr = [1, 2, 5, 4, 6, 7, 8, 9];
        assert_eq!(optimal_v2(&arr).unwrap(), 3);
    }

    #[test]
    fn sorted_array_approach_ok() {
        let arr = [1, 2, 3, 4, 6, 7, 8, 9];
        assert_eq!(sorted_array_approach(&arr), 5);
    }
}
