/* ---------------- Search Element in a Rotated Sorted Array ---------------- */
// Problem Statement: 
// Given an integer array arr of size N, sorted in ascending order (with distinct values) and a target value k. 
// Now the array is rotated at some pivot point unknown to you. 
// Find the index at which k is present and if k is not present return -1.

// Input Format: arr = [4,5,6,7,0,1,2,3], k = 0
// Result: 4

// Input Format: arr = [4,5,6,7,0,1,2], k = 3
// Result: -1

/* ------------------------- Binary Search Objective ------------------------ */

// The primary objective of the Binary Search algorithm is 
// ! to efficiently determine the appropriate half to eliminate, thereby reducing the search space by half. 
// ! It does this by determining a specific condition that ensures that the target is not present in that half.

// * Elimination condition determines in which half of searhc space, target doesn't lie and then eliminate it

/* -------------------------------- Optimal Approach -------------------------------- */
// Observation:
// 
/* ----------------------------------- A) ----------------------------------- */
// In sorted array case:
// By having a sorted array, we guarantee that each index divides the array into two sorted halves.
// 
// In the search process, we compare the target value with the middle element, i.e. arr[mid], 
// and then eliminate either the left or right half accordingly. 

// * Elimination condition: Comparison with arr[mid] of a sorted array for deciding in which target is/isn't

// This elimination becomes feasible due to the inherent property of the sorted halves
// (i.e. Both halves always remain sorted).

/* ----------------------------------- B) ----------------------------------- */
// In case of rotation of sorted array: 
// In this case, the array is both rotated and sorted. 
// As a result, the property of having sorted halves no longer holds. 
// 
// This disruption in the sorting order affects the elimination process, 
// making it unreliable to determine the target’s location by solely comparing it with arr[mid].

// ! Key Observation: 
// Though the array is rotated, we can clearly notice that for every index, 
// one of the 2 halves will always be sorted. 

// * Elimination condition: find in which sorted half target doesn't lie and eliminate it

// So, to efficiently search for a target value using this observation, 
// we will follow a simple two-step process. 

// - First, we identify the sorted half of the array. 
// - Once found, we determine if the target is located within this sorted half. 
//    - If not, we eliminate that half from further consideration. 
//    - Conversely, if the target does exist in the sorted half, we eliminate the other half.

// To identify the sorted half, we did:
// If arr[low] <= arr[mid]: In this case, we identified that the left half is sorted.
// If arr[mid] <= arr[high]: In this case, we identified that the right half is sorted.

/* ------------------------------------ x ----------------------------------- */

fn search(arr:&[u32], target:u32)-> Option<usize>{
    let n = arr.len();
    let mut low = 0;
    let mut high = n-1;

    while low <= high {
        let mid = (low + high)/2;
        let curr = arr[mid];
        
        if curr == target{
            return Some(mid);
        }
        
        // identify the sorted half of array (valid due to key observation made earlier)
        // - if left half is sorted else right half is sorted
        if arr[low] <= curr {
            // - if target is present in left half
            if arr[low] <= target && target <= curr {
                high = mid-1;
            }
            else{
                low = mid + 1;
            }
        }
        else {
            if arr[mid] <= target && target <= arr[high]{
                low = mid + 1;
            }
            else{
                high = mid -1;
            }
        }
    }
    None
}



/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod test_searching_in_rotated_sorted_array {
    use super::*;

    #[test]
    fn test_it() {
        let arr = [4,5,6,7,0,1,2];
        assert_eq!(search(&arr,0),Some(4));
        assert_eq!(search(&arr,3),None);
    }
}