/* ----------------- Find the repeating and missing numbers ----------------- */
// Problem Statement:
// You are given a read-only array of N integers with values also in the range [1, N] both inclusive.
// Each integer appears exactly once except A which appears twice and B which is missing.
// The task is to find the repeating and missing numbers A and B
// where A repeats twice and B is missing.

// Example 1:
// Input Format:  array[] = {3,1,2,5,3}
// Result: {3,4)

// Example 2:
// Input Format: array[] = {3,1,2,5,4,6,7,5}
// Result: {5,8)

/* --------------------------------- Approch -------------------------------- */

// Brute:
// Count occurence of each arr element using for loop and thus, find repeating and missing element

// Better:
// using the hashing technique, we will store the frequency of each element between 1 to N.
//
// using hashset, try inserting arr elem in it and when it's false, repeating
// and try inserting 1..=n in it and when it's true, missing

// Optimal
// V1: Idea is to convert the given problem into mathematical equations.
//      Since we have two variables i.e. missing and repeating, we will try to form two linear equations.
//      And then we will find the values of two variables using those equations.
//      equation generated from sum of natural numbers and sum of squares of natural numbers
//
// V2: Using XOR
//      step 1. Find the XOR of the repeating number(X) and the missing number(Y)
//      step 2. Find the first different bit from right between the repeating and the missing number
//      step 3. Based on the position of the different bits we will group all the elements into 2 different groups

/* ------------------------------- Complexity ------------------------------- */

// Brute:
// time: O(N^2), space: O(1)

// Better:
// time: O(2*N *(InsertTimeComplexity)), space: O(n)
//
// Better 2:
// time: O(2*N), space: O(n)

// Optimal:
// v1: time: O(N), space: O(1)
// v2: time: O(N), space: O(1)

/* ---------------------------------- Soln ---------------------------------- */

use std::collections::HashSet;

/// using XOR
fn optimal_soln_v2(arr: &[u32]) -> (u32, u32) {
    let n = arr.len();

    let mut xr = 0;

    // Step 1: Find XOR of all elements:
    for i in 0..n {
        xr ^= arr[i];
        xr ^= i as u32 + 1;
    }

    // Step 2: Find the differentiating bit number:
    let number = (xr & !(xr - 1));

    


    let repeating = 0;
    let missing = 0;

    (repeating as u32, missing as u32)
}

/// convert the given problem into mathematical equations.
fn optimal_soln_v1(arr: &[u32]) -> (u32, u32) {
    let n = arr.len();

    // sum of numbers in [1,n]
    let natural_sum = (n * (n + 1) / 2) as u32;
    let arr_sum: u32 = arr.iter().sum();

    // sum of squares of numbers in [1,n]
    let sq_natural_sum = (n * (n + 1) * (2 * n + 1) / 6) as u32;
    let sq_arr_sum: u32 = arr.iter().map(|&elem| elem * elem).sum();

    // [x - repeating number, y - missing number]
    // x - y
    let diff_x_y: i32 = arr_sum as i32 - natural_sum as i32;

    // x*x - y*y
    let diff_sqx_sqy = sq_arr_sum as i32 - sq_natural_sum as i32;

    // x + y
    let sum_x_y = diff_sqx_sqy / diff_x_y;

    let repeating = (sum_x_y + diff_x_y) / 2;
    let missing = (sum_x_y - diff_x_y) / 2;

    (repeating as u32, missing as u32)
}

fn better_soln(arr: &[u32]) -> (u32, u32) {
    let mut missing = 0;
    let mut repeating = 0;
    let n = arr.len();

    let mut set: HashSet<u32> = HashSet::new();
    for elem in arr {
        if !set.insert(*elem) {
            repeating = *elem;
        }
    }

    for elem in 1..=n {
        if set.insert(elem as u32) {
            missing = elem as u32;
        }
    }

    (repeating, missing)
}

fn better_soln_v2(arr: &[u32]) -> (u32, u32) {
    let mut missing = 0;
    let mut repeating = 0;
    let n = arr.len();

    let mut occurences: Vec<u32> = vec![0; n];

    for elem in arr {
        *occurences.get_mut(*elem as usize).unwrap() += 1;
    }

    for i in 1..=n {
        if occurences[i] == 2 {
            repeating = i;
        }
        if occurences[i] == 0 {
            missing = i;
        }

        if repeating != 0 && missing != 0 {
            break;
        }
    }

    (repeating as u32, missing as u32)
}

fn brute_soln(arr: &[u32]) -> (u32, u32) {
    let mut missing = 0;
    let mut repeating = 0;
    let n = arr.len();

    for i in 1..=n {
        let mut count = 0;
        for j in 0..n {
            if arr[j] == i as u32 {
                count += 1;
            }
        }
        if count == 2 {
            repeating = i as u32;
        } else if count == 0 {
            missing = i as u32;
            break;
        }
        if repeating != 0 && missing != 0 {
            break;
        }
    }
    (repeating, missing)
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod test_prob4 {
    use super::*;

    #[test]
    fn optimal_v1() {
        let arr = [3, 1, 2, 5, 3];
        let sol = (3, 4);
        assert_eq!(optimal_soln_v1(&arr), sol);

        let arr = [3, 1, 2, 5, 4, 6, 7, 5];
        let sol = (5, 8);
        assert_eq!(optimal_soln_v1(&arr), sol);
    }

    #[test]
    fn better() {
        let arr = [3, 1, 2, 5, 3];
        let sol = (3, 4);
        assert_eq!(better_soln(&arr), sol);

        let arr = [3, 1, 2, 5, 4, 6, 7, 5];
        let sol = (5, 8);
        assert_eq!(better_soln(&arr), sol);
    }

    #[test]
    fn brute() {
        let arr = [3, 1, 2, 5, 3];
        let sol = (3, 4);
        assert_eq!(brute_soln(&arr), sol);

        let arr = [3, 1, 2, 5, 4, 6, 7, 5];
        let sol = (5, 8);
        assert_eq!(brute_soln(&arr), sol);
    }
}
