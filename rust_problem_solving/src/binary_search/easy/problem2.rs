/* -------------------------- Implement Upper Bound ------------------------- */
// Problem Statement:
// Given a sorted array of N integers and an integer x,
// write a program to find the upper bound of x.

// The upper bound algorithm finds the first or the smallest index in a sorted array
// where the value at that index is greater than the given key i.e. x.

// find i such that arr[i] > x and arr[i] is the largest among the answers

/* ------------------------------------ x ----------------------------------- */

// assuming sorted in ascending order
fn find_upper_bound(arr: &[u32], target: u32) -> Option<usize> {
    let mut answer = None;
    let mut low = 0;
    let mut high = (arr.len() - 1);

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid ] > target {
            answer = Some(mid);
            high = mid.checked_sub(1).unwrap_or(0);
            if high == 0{
                break;
            }
        } else {
            low = mid.checked_add(1).unwrap_or(0);
            if low == arr.len()-1 && arr[low] != target{
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

        let res = find_upper_bound(&arr, 2);
        assert_eq!(res, Some(1));

        let res = find_upper_bound(&arr, 5);
        assert_eq!(res, Some(3));

        let res = find_upper_bound(&arr, 8);
        assert_eq!(res, None);

        let res = find_upper_bound(&arr, 0);
        assert_eq!(res, Some(0));

        let res = find_upper_bound(&arr, 20);
        assert_eq!(res, Some(5));
    }
}
