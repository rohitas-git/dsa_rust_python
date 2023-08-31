/* ----------- Find out how many times the array has been rotated ----------- */
// Problem Statement:
// Given an integer array arr of size N, sorted in ascending order (with distinct values).
// Now the array is rotated between 1 to N times which is unknown.
// Find how many times the array has been rotated.

/* -------------------------------- Approach -------------------------------- */

// Observation:
// the number of rotations in an array is equal to the index(0-based index) of its minimum element.

// Result: Find the index of minimum element to count the number of rotation of array

// Solution 1: Algorithm used in problem 9 with minor changes
// Solution 2: A variant of above algorithm

/* ------------------------------------ x ----------------------------------- */
// ! Only solution 1 works perfectly

fn solution_1(arr: &[u32]) -> usize {
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;
    let mut min = arr[0];
    let mut index = 0;
    let mut rotations = 0;

    if arr[low] <= arr[high] {
        min = arr[low];
        index = low;
    }

    while low <= high {
        let mid = low + (high - low) / 2;
        let current = arr[mid];

        // comparing min and arr[mid]
        if min > current {
            min = current;
            index = mid;
        }
        // If half left to mid is sorted
        if current >= arr[high] {
            if mid != (n - 1) {
                low = mid + 1;
            } else {
                break;
            }
        } else {
            if mid != 0 {
                high = mid - 1;
            } else {
                break;
            }
        }
    }
    rotations = index;
    rotations
}

fn solution_2(arr: &[u32]) -> usize {
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;
    let mut index = 0;
    let mut min = u32::MAX;

    while low <= high {
        let mid = low + (high - low) / 2;
        let current = arr[mid];

        // If search space is already sorted,
        if arr[low] <= arr[high] {
            if arr[low] < min {
                index = low;
                min = arr[low];
            }
            break;
        }
        // If left part is sorted
        if arr[high] > current {
            // Keep the minimum
            if arr[low] < min {
                index = low;
                min = arr[low];
            }
            if mid != (n - 1) {
                // Eliminate right half
                low = mid + 1;
            } else {
                break;
            }
        } else {
            // Keep the minimum
            if arr[mid] < min {
                index = mid;
                min = arr[mid];
            }
            if mid != 0 {
                // Eliminate right half
                high = mid - 1;
            } else {
                break;
            }
        }
    }
    index
}

fn solution_3(arr: &[u32], low: usize, high: usize) -> usize {
    let n = arr.len();
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut index = 0;

    if high < low {
        return 0;
    }
    if high == low {
        return low;
    }

    let mid = (high + low) / 2;
    if mid < high && arr[mid] < arr[mid + 1] {
        return (mid + 1);
    }
    if mid > low && arr[mid] < arr[mid - 1] {
        return mid - 1;
    }

    // Decide whether we need to go to left half or
    // right half
    if arr[high] > arr[mid] {
        return solution_3(arr, low, mid - 1);
    }

    return solution_3(arr, mid + 1, high);
}

#[cfg(test)]
mod test_couting_rotations_of_array {
    use super::*;

    #[test]
    fn solution_1_works() {
        let arr = [1, 2, 3, 4, 5];
        let res = solution_1(&arr);
        assert_eq!(res, 0);

        let arr = [5, 4, 1, 2, 3];
        let res = solution_1(&arr);
        assert_eq!(res, 2);

        let arr = [5, 4, 3, 2, 1];
        let res = solution_1(&arr);
        assert_eq!(res, 4);
    }

    #[test]
    fn solution_2_works() {
        let arr = [1, 2, 3, 4, 5];
        let res = solution_2(&arr);
        assert_eq!(res, 0);

        let arr = [5, 4, 1, 2, 3];
        let res = solution_2(&arr);
        assert_eq!(res, 2);

        let arr = [5, 4, 3, 2, 1];
        let res = solution_2(&arr);
        assert_eq!(res, 4);
    }

    #[test]
    fn solution_3_works() {
        let arr = [1, 2, 3, 4, 5];
        let low = 0;
        let high = arr.len() - 1;

        let res = solution_3(&arr, low, high);
        assert_eq!(res, 0);

        let arr = [5, 4, 1, 2, 3];
        let res = solution_3(&arr, low, high);
        assert_eq!(res, 2);

        let arr = [5, 4, 3, 2, 1];
        let res = solution_3(&arr, low, high);
        assert_eq!(res, 4);
    }
}
