/* ------------------------------- Quick Sort ------------------------------- */

// Intuition:
// Quick Sort is a divide-and-conquer algorithm like the Merge Sort.
// But this algorithm does not use any extra array for sorting(though it uses an auxiliary stack space).
// So, from that perspective, Quick sort is slightly better than Merge sort.

/* ------------------------------ Algorithm: ----------------------------- */
// Recursive repeatition of:
//      1. Pick a pivot and place it in its correct place in the sorted array.
//      2. Shift smaller elements on the left of the pivot
//         and larger ones to the right.

/* ------------------------ Pivot --------------------------- */
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

/* ------------------------------- Complexity ------------------------------- */
// T(n) = T(k-1) + T(n-k-1) + C1 (pivot) + C2(parition)
// T(1) = theta(1)
// S(n) = C3(parition) + n (stack space) = C3 + n
//
// Worst Case: when the pivot is the greatest or smallest element of the array.
// F(n) = F(0)+F(n-1)  or  F(n) = F(n-1) + F(0)
// Worst case time: O(n^2)
//
// Best case: when the pivot is the middle element or near to middle element of the array.
// Recurrence : F(n) = 2F(n/2)
// Time: O(N*logN)
//
// Time for best and avg case : O(N*logN)
// Space: O(N) (parition) + O(N) (Aux stack space)

/* ------------------------------------ - ----------------------------------- */

pub mod qs_v2 {
    pub fn quick_sort(arr: &mut [u8]) {
        qs(arr, 0, arr.len() - 1);
    }

    // low - first index of arr
    // high - last index of arr

    fn qs(arr: &mut [u8], low: usize, high: usize) {
        if (low < high) {
            let p_index = parition(arr, low, high);
            qs(arr, low, p_index - 1);
            qs(arr, p_index + 1, high);
        }
    }

    // Inside the function, we will first select the pivot(i.e. arr[low] in our case).
    // Now, we will again take two-pointers i and j. The i pointer points to low and the j points to high.
    // Now, the pointer i will move forward and find the first element that is greater than the pivot. Similarly, the pointer j will move backward and find the first element that is smaller than the pivot.
    // Here, we need to add some checks like i <= high-1 and j >= low+1. Because it might happen that i is standing at high and trying to proceed or j is standing at low and trying to exceed.
    // Once we find such elements i.e. arr[i] > pivot and arr[j] < pivot, and i < j, we will swap arr[i] and arr[j].
    // We will continue step 3 and step 4, until j becomes smaller than i.
    // Finally, we will swap the pivot element(i.e. arr[low]) with arr[j] and will return the index j i.e. the partition index.
    fn parition(arr: &mut [u8], low: usize, high: usize) -> usize {
        let mut pivot = low;
        let mut pivot_val = arr[pivot];
        let mut smaller = low;
        let mut larger = high;

        println!("S:{}, L:{}", smaller, larger);
        println!("{:?}", arr);
        while smaller < larger {
            println!("Before:: S:{}, L:{}", smaller, larger);
            
            while arr[smaller] < pivot_val && smaller <= high - 1 {
                smaller += 1;
            }
            while arr[larger] > pivot_val && larger >= low + 1 {
                larger -= 1;
            }
            
            if smaller < larger {
                (arr[larger], arr[smaller]) = (arr[smaller], arr[larger]);
            }
            println!("After:: S:{}, L:{}", smaller, larger);
            println!("After {:?}", arr);
        }
        (arr[pivot], arr[larger]) = (arr[larger], arr[pivot]);

        larger
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
}
pub mod qs_v1 {
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

        // Select Pivot - Middle element of array
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
        println!("{:?}", arr);
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
}
