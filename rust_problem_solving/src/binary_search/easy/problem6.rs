/* -------------------- Count Occurrences in Sorted Array ------------------- */
// Problem Statement:
// You are given a sorted array containing N integers and a number X,
// you have to find the occurrences of X in the given array.

/* ------------------------------------ x ----------------------------------- */

// Approach:
// Find the first occurence of target
// If not present, then return None; else find last occurence
// total number of occcurrences = last - first + 1

/* ------------------------------------ x ----------------------------------- */

fn count_occurrences(arr: &[u32], target: u32) -> Option<usize> {
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;
    let first = get_first_occurence(arr, target);
    if first == None{
        return  None;
    }
    let last = get_last_occurence(arr, target);
    
    Some(last.unwrap() - first.unwrap() + 1)
}

fn get_first_occurence(arr: &[u32], target: u32) -> Option<usize> {
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] < target {
            low = mid + 1;
        } else if arr[mid] > target {
            high = mid - 1;
        } else {
            if mid == 0 || arr[mid - 1] != arr[mid] {
                return Some(mid);
            } else {
                high = mid - 1;
            }
        }
    }
    None
}

fn get_last_occurence(arr: &[u32], target: u32) -> Option<usize> {
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] < target {
            low = mid + 1;
        } else if arr[mid] > target {
            high = mid - 1;
        } else {
            if mid == n - 1 || arr[mid] != arr[mid + 1] {
                return Some(mid);
            } else {
                low = mid + 1;
            }
        }
    }
    None
}

#[cfg(test)]
mod test_count_occurrences {
    use super::*;

    #[test]
    fn test_solution() {
        let arr = [3, 4, 13, 13, 13, 20, 40];

        let res = count_occurrences(&arr, 13);
        assert_eq!(res, Some(3));

        let res = count_occurrences(&arr, 60);
        assert_eq!(res, None);
    }

    #[test]
    fn getting_first() {
        let arr = [3, 4, 13, 13, 13, 20, 40];

        let res = get_first_occurence(&arr, 13);
        assert_eq!(res, Some(2));

        let res = get_first_occurence(&arr, 60);
        assert_eq!(res, None);
    }

    #[test]
    fn getting_last() {
        let arr = [3, 4, 13, 13, 13, 20, 40];

        let res = get_last_occurence(&arr, 13);
        assert_eq!(res, Some(4));

        let res = get_last_occurence(&arr, 60);
        assert_eq!(res, None);
    }
}
