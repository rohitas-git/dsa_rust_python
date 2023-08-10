/* --------------------------- Problem Statement: --------------------------- */
// Problem Statement: Union of Two Sorted Arrays
// Given two sorted arrays, arr1, and arr2 of size n and m. Find the union of two sorted arrays.
// The union of two arrays can be defined as the common and distinct elements in the two arrays.
// NOTE: Elements in the union should be in ascending order.

/* -------------------------------- Approach: ------------------------------- */
// 1. Using Map
// 2. Using Set
// 3. Using two pointers

/* ------------------------------- Complexity ------------------------------- */

/* ------------------------------------ x ----------------------------------- */

mod union_of_two_sorted_arr {
    use std::collections::HashMap;

    pub fn find_union_v0(arr1: &[u32], arr2: &[u32]) -> Vec<u32>{
        let mut union = Vec::new();

        let mut i = 0;
        let mut j = 0;

        while i < arr1.len() || j < arr2.len() {            
            if arr1[i] == arr2[j] && union.last().unwrap() != &arr1[i]{
                union.push(arr1[i]);
                j+=1;
                i+=1;
            }
            if i < arr1.len() && (j >= arr2.len() || arr1[i] <= arr2[j]) {
                
                i += 1;
            } else if j < arr2.len() && (i >= arr1.len() || arr2[j] <= arr1[i]) {
                
                j += 1;
            }
        }

        union        
        
    }

    // In a random order when printing
    pub fn find_union_v1(arr1: &[u32], arr2: &[u32]) -> Vec<u32> {
        let n = arr1.len();
        let m = arr2.len();

        let mut seen = HashMap::new();
        let mut union = Vec::new();

        let mut i = 0;
        let mut j = 0;

        while i < arr1.len() || j < arr2.len() {
            if i < arr1.len() && (j >= arr2.len() || arr1[i] <= arr2[j]) {
                seen.entry(arr1[i]).or_insert(false);
                i += 1;
            } else if j < arr2.len() && (i >= arr1.len() || arr2[j] <= arr1[i]) {
                seen.entry(arr2[j]).or_insert(false);
                j += 1;
            }
        }

        for (&num, &seen_before) in seen.iter() {
            if !seen_before {
                union.push(num);
            }
        }
        union
    }

    pub fn find_union_v2(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
        let mut union = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] < arr2[j] {
                union.push(arr1[i]);
                i += 1;
            } else if arr1[i] > arr2[j] {
                union.push(arr2[j]);
                j += 1;
            } else {
                union.push(arr1[i]);
                i += 1;
                j += 1;
            }
        }

        while i < arr1.len() {
            union.push(arr1[i]);
            i += 1;
        }

        while j < arr2.len() {
            union.push(arr2[j]);
            j += 1;
        }

        union
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_s1() {
        let arr1 = [1, 2, 3, 4, 5];
        let arr2 = [1, 3, 4, 4, 5, 8, 9];

        let r = union_of_two_sorted_arr::find_union_v1(&arr1, &arr2);
        println!("{:?}", r);
    }
}
