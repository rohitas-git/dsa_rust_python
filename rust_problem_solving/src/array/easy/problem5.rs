/* --------------------------- Problem Statement: --------------------------- */
// Problem Statement: Given an array of N integers, left rotate the array by one place.

/* -------------------------------- Approach: ------------------------------- */
// 1. Brute (O(n), O(n))
//      take another dummy array of the same length and then shift all elements in the array toward the left
//      and then at the last element store the index of 0th index of the array
//
// 2. Optimal (O(n), O(1))
//      Store the first element in tmp variable, shift other elements left by looping till n-1th index
//      arr[i] = arr[i-1] and lastly store arr[n-1] = tmp
//

/* ------------------------------------Analysis ----------------------------------- */

fn rotate_left_by_one(arr: &mut [u32]) {
    let tmp = arr[0];
    let n = arr.len();

    for i in 0..(n-1){
        arr[i] = arr[i+1];
    }
    arr[n-1] = tmp;
}

#[cfg(test)]
mod test_left_rotation {
    use super::*;

    #[test]
    fn test_ok() {
        let mut arr = [1, 2, 3, 4, 5];
        rotate_left_by_one(&mut arr);
        assert_eq!(arr, [2, 3, 4, 5, 1]);
    }
}
