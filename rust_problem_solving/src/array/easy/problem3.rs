// Problem Statement: Given an array of size n, write a program to check if the given array is sorted in (ascending / Increasing / Non-decreasing) order or not. If the array is sorted then return True, Else return False.
// Note: Two consecutive equal values are considered to be sorted.

// Approach:
// 1. Brute: time O(n*n)
// 2. Optimal: time O(n)

fn is_sorted(arr: &[u32]) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_sorted_ok() {
        let arr = &[12, 0, 40, 99];
        let arr2 = &[1, 2, 40, 99];

        assert_eq!(is_sorted(arr), false);
        assert_eq!(is_sorted(arr2), true);
    }
}
