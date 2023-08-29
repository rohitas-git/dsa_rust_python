/* ---------------- Search Element in Rotated Sorted Array II --------------- */
// Problem Statement:
// Given an integer array arr of size N, sorted in ascending order
// (may contain duplicate values) and a target value k.
// Now the array is rotated at some pivot point unknown to you.
// Return True if k is present and otherwise, return False.

// Input Format: arr = [7, 8, 1, 2, 3, 3, 3, 4, 5, 6], k = 3
// Result: True

/* -------------------------------- Approach -------------------------------- */

// To identify the sorted half, we did:
// If arr[low] <= arr[mid]: In this case, we identified that the left half is sorted.
// If arr[mid] <= arr[high]: In this case, we identified that the right half is sorted.

// Consequently, the previous approach will not work when arr[low] = arr[mid] = arr[high].

// Handle the edge case arr[low] = arr[mid] = arr[high]:
//
// Check if arr[low] = arr[mid] = arr[high]: If this condition is satisfied,
// we will just increment the low pointer and decrement the high pointer by one step.
// We will not perform the later steps until this condition is no longer satisfied.
// So, we will continue to the next iteration from this step.

/* ------------------------------------ x ----------------------------------- */

fn search(arr: &[u32], target: u32) -> bool{
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let curr = arr[mid];

        if curr == target {
            return true;
        }

        // move low and high till we are out of edge case
        if arr[low] == curr && curr == arr[high] {
            low += 1;
            high -= 1;
            continue;
        }

        // identify the sorted half of array (valid due to key observation made earlier)
        // - if left half is sorted else right half is sorted
        if arr[low] <= curr {
            // - if target is present in left half
            if arr[low] <= target && target <= curr {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        } else {
            if arr[mid] <= target && target <= arr[high] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
    }
    false
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod test_searching_in_rotated_sorted_array {
    use super::*;

    #[test]
    fn test_it() {
        let arr = [7, 8, 1, 2, 3, 3, 3, 4, 5, 6];
        assert_eq!(search(&arr, 3), true);
        assert_eq!(search(&arr, 10), false);
    }
}
