/* --------------------------- Problem Statement: --------------------------- */
// Problem Statement:
// You are given an array of integers,
// your task is to move all the zeros in the array to the end of the array
// and move non-negative integers to the front by maintaining their order.

/* -------------------------------- Approach: ------------------------------- */
// 1. Optimal
//  Using two pointers i and j. j points to first zero found. i will be iterate over arr
//  If set the condn if that arr[i] != 0 then swap (arr[i],arr[j]). j moves to one place to right.
//  Note: j will always point to a zero in arr

/* ------------------------------- Complexity ------------------------------- */
//  1. T:O(n), S:O(1)

fn move_zeros_to_end(arr: &mut [u32]) -> bool {
    let mut zero_ptr = usize::MAX;
    let n = arr.len();

    for k in 0..n {
        if arr[k] == 0 {
            zero_ptr = k;
            break;
        }
    }
    if zero_ptr == usize::MAX {
        return false;
    }

    for i in (zero_ptr + 1)..n {
        if arr[i] != 0 {
            (arr[zero_ptr], arr[i]) = (arr[i], arr[zero_ptr]);
            zero_ptr += 1;
        }
    }
    true
}

#[cfg(test)]
mod test_moving_zeroes {
    use super::*;

    #[test]
    fn test_ok() {
        let mut arr = [1, 0, 2, 3, 0, 2, 0, 0, 4, 5, 1];
        move_zeros_to_end(&mut arr);
        assert_eq!(arr, [1, 2, 3, 2, 4, 5, 1, 0, 0, 0, 0]);
    }
}
