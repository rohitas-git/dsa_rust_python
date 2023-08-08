/* ------------------------------- Quick Sort ------------------------------- */

// Intuition:
// Quick Sort is a divide-and-conquer algorithm like the Merge Sort.
// But this algorithm does not use any extra array for sorting(though it uses an auxiliary stack space).
// So, from that perspective, Quick sort is slightly better than Merge sort.

/* ------------------------------ Algorithm: ----------------------------- */
// Recursive repeatition of:
//      1. Pick a pivot and place it in its correct place in the sorted array.
//      2. Shift smaller elements(i.e. Smaller than the pivot) on the left of the pivot
//         and larger ones to the right.

// Pivot: The element or the pivot can be chosen by our choice.
// After choosing the pivot(i.e. the element),
// place it in its correct position(i.e. The place it should be after the array gets sorted) in the array.
//
// Pivot can be:
//
// -The first element of the array
// -The last element of the array
// -Median of array
// -Any Random element of the array


// T(n) = T(k-1) + T(n-k-1) + 2*n 
// T(1) = theta(1)
// S(n) = S(k-1) + S(n-k-1) + n = 3*n 
//
// Time: O(N*logN)
// Space: O(N)
pub fn quick_sort(arr: &mut [u8]) {
    let len = arr.len();

    // base case
    if len == 1 || len == 0 {
        return;
    }

    // Select and position the pivot
    let pivot = pivot(arr);

    // Shift the elements according to pivot
    parition(arr, pivot);

    // sort the left sub-array
    quick_sort(&mut arr[..pivot]);

    // sort the right sub-array
    quick_sort(&mut arr[(pivot + 1)..len]);
}

// time: O(n) ; space: O(1)
fn pivot(arr: &mut [u8]) -> usize {
    let len = arr.len();

    // Select Pivot - First element of array
    let mut pivot = len / 2;

    // Position the pivot
    let mut pos = 0;
    for i in 0..len {
        if arr[i] < arr[pivot] {
            pos += 1;
        }
    }
    if pos != pivot {
        (arr[pos], arr[pivot]) = (arr[pivot], arr[pos]);
        pivot = pos;
    }
    pivot
}

// Time: O(n)       {t = 2*n}
// space: Best - O(1), Avg/Worst - O(n)
fn parition(arr: &mut [u8], pivot: usize) {
    // Shift elements
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    // println!("{}: {}", left, right);

    for i in 0..arr.len() {
        if i < pivot {
            if arr[i] > arr[pivot] {
                left.push(i);
            }
        }
        if i > pivot {
            if arr[i] < arr[pivot] {
                right.push(i);
            }
        }
    }
    // println!("{:?}",pivot);
    
    // println!("{:?}",arr);
    // println!("left {:?}",left);
    // println!("right {:?}",right);
    // std::cmp::min(left.len()

    for i in 0..left.len() {
        let a = left[i];
        let b = right[i];
        (arr[a], arr[b]) = (arr[b], arr[a]);
    }
    println!("{:?}",arr);
}

#[cfg(test)]
mod test_quick_sort {
    use super::*;

    #[test]
    fn ok_sort_worst_case() {
        let mut arr2 = [1, 3, 9, 7, 5, 2, 4];
        quick_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5, 7, 9]);
    }
}
