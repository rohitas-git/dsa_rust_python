/* ---------------- Longest Consecutive Sequence in an Array ---------------- */
// Problem Statement: You are given an array of ‘N’ integers.
// You need to find the length of the longest sequence which contains the consecutive elements.

/* -------------------------------- Approach -------------------------------- */
// Brute:
// A straightforward but basic solution is to examine consecutive sequences for each element in the given array.

// Better:
// We can simply sort the array and run a for loop to find the longest consecutive sequence.

/* ------------------------------------ x ----------------------------------- */

mod brute {

    fn linear_search(arr: &[u32], key: u32) -> bool {
        let n = arr.len();
        for i in 0..n {
            if arr[i] == key {
                return true;
            }
        }
        false
    }

    fn longest_successive_elements(arr: &[u32]) -> u32 {
        let n = arr.len();
        let mut longest = 1;

        for i in 0..n {
            let mut x = arr[i];
            let mut count = 1;

            while linear_search(&arr, x + 1) {
                x += 1;
                count += 1;
            }
            longest = std::cmp::max(longest, count);
        }
        longest
    }

    #[cfg(test)]
    mod find_longest_successive_elements {
        use super::*;
        #[test]
        fn test_brute_solution() {
            let a = [100, 200, 1, 2, 3, 4];
            assert_eq!(longest_successive_elements(&a), 4);
        }
    }
}

// time - O(NlogN) + O(N)
// space - O(1)
fn better(arr: &mut [u32]) -> u32 {
    arr.sort();

    let mut longest = 1;
    let mut count = 0;
    let mut last_smaller = 0;

    for i in 0..arr.len() {
        if arr[i] - 1 == last_smaller {
            last_smaller = arr[i];
            count += 1;
        } else if arr[i] != last_smaller {
            count = 1;
            last_smaller = arr[i];
        }
        longest = std::cmp::max(longest, count);
    }
    longest
}

#[cfg(test)]
mod find_longest_successive_elements {
    use super::*;

    #[test]
    fn test_better() {
        let mut a = [100, 200, 1, 2, 3, 4];
        assert_eq!(better(&mut a), 4);
    }
}
