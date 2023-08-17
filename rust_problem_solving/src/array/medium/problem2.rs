/* --------------------- Sort an array of 0s, 1s and 2s --------------------- */
// Problem Statement: Given an array consisting of only 0s, 1s, and 2s.
// Write a program to in-place sort the array without using inbuilt sort functions.

// Input: nums = [2,0,2,1,1,0]
// Output: [0,0,1,1,2,2]

/* -------------------------------- Approach -------------------------------- */
// Brute: Sorting the array

// Better:   [ O(N) + O(N), O(1) ]
// Keeping count of values  
// Overriding 0,1,2s according to their counts

// Optimal:     Time O(N), Space(1)
// This problem is a variation of the popular Dutch National flag algorithm.
// This algorithm contains 3 pointers i.e. low, mid, and high, and 3 main rules.
// The rules are the following:
//
// arr[0….low-1] contains 0. [Extreme left part]
// arr[low….mid-1] contains 1.
// arr[high+1….n-1] contains 2. [Extreme right part], n = size of the array
//
// The middle part i.e. arr[mid….high] is the unsorted segment.

/* ------------------------------------ x ----------------------------------- */
// Dutch National flag algorithm: 
// 3 ptrs divide the array into 4 parts
// part 1 : arr[0..=low-1] contains 0. 
// part 2 : arr[low..=mid-1] contains 1.
// part 3 : arr[mid..=high] is unsorted
// part 4 : arr[high+1..=n-1] contains 2
// Note: Here in this solution, I will work based on the value of the mid pointer.


// Idea is simple: 
// In one Big box, we have balls of three colors R,G,B
// we have 3 flexible paritions for each color: box R, G, B
// Repeat:
//  Pick a ball from Big box 
//  Put them in their respective parition
// till there are 0 balls in Big balls

// The key idea is that by swapping elements based on their values, 
// you effectively partition the array into three segments without the need for extra space. 
// This algorithm maintains the sorting order and achieves the desired time and space complexity.
fn optimal(arr: &mut [u32]) {
    let n = arr.len();

    let mut low = 0;
    let mut mid = 0;
    let mut high = n - 1;
    let mut c = 0;
    while mid <= high {
        if arr[mid] == 0 {
            (arr[mid], arr[low]) = (arr[low], arr[mid]);
            println!("0::{}: {}",c, mid);
            low += 1;
            mid += 1;
            c+=1;
        }

        if arr[mid] == 1 {
            println!("1::{}: {}",c, mid);
            mid += 1;
            c+=1;

        }

        if arr[mid] == 2 {
            println!("2::{}: {}",c, mid);
            (arr[mid], arr[high]) = (arr[high], arr[mid]);
            high -= 1;
            c+=1;

        }
    }
}

#[cfg(test)]
mod test_sorting_array_of_012s {
    use super::*;

    #[test]
    fn optimal_ok() {
        let mut arr = [2, 0, 2, 1, 1, 0];
        optimal(&mut arr);
        assert_eq!(arr, [0, 0, 1, 1, 2, 2]);
    }
}
