/* ------------------------- Search Insert Position ------------------------- */
// Problem Statement:
// You are given a sorted array arr of distinct values and a target value x.
// You need to search for the index of the target value in the array.
//
// If the value is present in the array, then return its index.
// Otherwise, determine the index where it would be inserted in the array while maintaining the sorted order.

/* -------------------------------- Solution -------------------------------- */
// On deep introspection of the given problem, we can easily understand that
// we have to find the correct position or the existing position of the target number in the given array.

// Now, if the element is not present,
// we have to find the nearest greater number of the target number.
// So, basically, we are trying to find an element arr[ind] >= x and hence the lower bound of the target number i.e. x.

// The lower bound algorithm returns the first occurrence of the target number if the number is present
// and otherwise, it returns the nearest greater element of the target number.

/* ------------------------------------ x ----------------------------------- */

// Same as finding Lower Bound algorithm

fn search_insert_position(arr: &[u32], target: u32) -> Option<usize> {
    let mut answer = None;
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {        
        let mid = low + (high - low) / 2;
        if target <= arr[mid] {
            answer = Some(mid);
            high = mid.checked_sub(1).unwrap_or(0);
            if high == 0{
                break;
            }
        } else {
            low = mid.checked_add(1).unwrap_or(0);
            if low == n-1{
                answer = Some(low)
            }
        }
    }
    answer
}

#[cfg(test)]
mod test_finding_lower_bound {
    use super::*;

    #[test]
    fn test_solution() {
        let arr = [1, 3, 5, 6, 7, 8];

        let res = search_insert_position(&arr, 2);
        assert_eq!(res, Some(1));

        let res = search_insert_position(&arr, 5);
        assert_eq!(res, Some(2));

        let res = search_insert_position(&arr, 8);
        assert_eq!(res, Some(5));

        let res = search_insert_position(&arr, 0);
        assert_eq!(res, Some(0));

        let res = search_insert_position(&arr, 20);
        assert_eq!(res, Some(5));
    }
}
