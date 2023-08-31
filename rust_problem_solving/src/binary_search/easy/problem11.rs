/* ----------------- Search Single Element in a sorted array ---------------- */
// Problem Statement:
// Given an array of N integers.
// Every number in the array except one appears twice.
// Find the single number in the array.

// Input Format: arr[] = {1,1,2,2,3,3,4,5,5,6,6}
// Result: 4

/* -------------------------------- Approach -------------------------------- */

/* ------------------------------------ x ----------------------------------- */

fn search_single_element(arr: &[u32]) -> Option<u32> {
    let n = arr.len();

    // Handle edge cases due to checking if arr[mid] is single element
    if n == 1 {
        return Some(arr[n - 1]);
    }
    if arr[0] != arr[1] {
        return Some(arr[0]);
    }
    if arr[n - 1] != arr[n - 2] {
        return Some(arr[n - 1]);
    }

    let mut low = 1;
    let mut high = n - 2;
    while low <= high {
        let mid = low + (high - low) / 2;
        let curr = arr[mid];
        let prev = arr[mid - 1];
        let next = arr[mid + 1];

        // If arr[mid] is the single element:
        if curr != next && curr != prev {
            return Some(curr);
        }
        // if mid in left of single element else in right of single element
        if (mid % 2 == 0 && curr == next) || (mid % 2 == 1 && curr == prev) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod test_finding_single_elements {
    use super::*;

    #[test]
    fn optimal_solution_works() {
        let arr = [1, 1, 2, 2, 3, 3, 4, 5, 5, 6, 6];
        let res = search_single_element(&arr).unwrap();
        assert_eq!(res, 4);

        let arr = [1, 4, 4, 5, 5, 6, 6];
        let res = search_single_element(&arr).unwrap();
        assert_eq!(res, 1);

        let arr = [1, 1, 4, 4, 5, 5, 6];
        let res = search_single_element(&arr).unwrap();
        assert_eq!(res, 6);

        let arr = [12];
        let res = search_single_element(&arr).unwrap();
        assert_eq!(res, 12);
    }
}
