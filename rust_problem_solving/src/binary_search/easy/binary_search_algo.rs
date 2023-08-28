/* ---------------------------------- Note: --------------------------------- */

// - Binary search is only applicable in a sorted search space.
// The sorted search space does not necessarily have to be a sorted array.
// It can be anything but the search space must be sorted.

// - In binary search, we generally divide the search space into two equal halves
// and then try to locate which half contains the target.
// According to that, we shrink the search space size.

/* ------------------------------- Complexity ------------------------------- */

// In the algorithm, in every step, we are basically dividing the search space into 2 equal halves. 
// This is actually equivalent to dividing the size of the array by 2, every time. 
// After a certain number of divisions, the size will reduce to such an extent 
// that we will not be able to divide that anymore and the process will stop. 
// The number of total divisions will be equal to the time complexity.
// If a number n can be divided by 2 for x times:
//      2^x = n
// Therefore, x = logn (base is 2)
// Time - O(log N)

// Iterative - O(logN), O(1)
// Recursive - O(logN), O(N)

/* ------------------------------------ Algorithm ----------------------------------- */

pub fn recursive_binary_search(arr: &[u32], target: u32, low: usize, high: usize) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid = low + (high - low) / 2;
    let mid_val = arr[mid];

    if target == mid_val {
        // println!("==");
        return Some(mid);
    } else if target < mid_val {
        // println!("<--");
        return recursive_binary_search(arr, target, low, mid-1);
    } else {
        // println!("-->");
        return recursive_binary_search(arr, target, mid + 1, high);
    }
}

pub fn iterative_binary_search(arr: &[u32], target: u32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let mid_val = arr[mid];

        if target == mid_val {
            return Some(mid);
        } else if target < mid_val {
            high = mid-1;
        } else {
            low = mid + 1;
        }
    }
    None
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod test_binary_search {
    use super::*;

    #[test]
    fn recursive_ok() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let low = 0;
        let high = arr.len() - 1;

        let res = recursive_binary_search(&arr, 3, low, high);
        assert_eq!(res, Some(2));

        let res = recursive_binary_search(&arr, 8, low, high);
        assert_eq!(res, Some(7));

        let res = recursive_binary_search(&arr, 11, low, high);
        assert_eq!(res, None);
    }

    #[test]
    fn iterative_ok() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    
        let res = iterative_binary_search(&arr, 3);
        assert_eq!(res, Some(2));

        let res = iterative_binary_search(&arr, 8);
        assert_eq!(res, Some(7));

        let res = iterative_binary_search(&arr, 11);
        assert_eq!(res, None);
    }
}
