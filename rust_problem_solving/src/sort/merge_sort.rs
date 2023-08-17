/* ------------------------------- Merge Sort ------------------------------- */
// Intuition

// 1.
// MergeSort is a divide and conquers algorithm,
// -Divide the array into halves
// -Sort both halves recursively
// -Merge sorted halves and return

// 2.
// There are 2 main functions :
// merge(): This function is used to merge the 2 halves of the array.
//          It assumes that both parts of the array are sorted and merges both of them.
//
// mergeSort():
// This function divides the array into 2 parts. low to mid and mid+1 to high where,
// low = leftmost index of the array
// high = rightmost index of the array
// mid = Middle index of the array

//3.
// We recursively split the array, and go from top-down until all sub-arrays size becomes 1.

/* -------------------------------- Algorithm ------------------------------- */

// Sort pseudocode:
//  - if low >= high return
//  - mid = (low + high)/2
//  - Sort(arr,low,mid)  (left half)
//  - Sort(arr,mid+1,high) (right half)
//  - merge(arr, low, mid, high)

// Merge pseudocode:
// - 1.Create an temp Array of size of merged array = sizeOf(left_Arr + right_Arr)
// - 2.Two pointers; one to leftArr's low and one to rightArr's mid+1
// - 3.while both ptrs <= lastIndex {compare the selected element from each Arr and insert smallest to temp Array }
// - 4.Copy the left-out elements in both halves into temp Array
// - 5.Copy elements of temp array in range(low,high) to Original Array

/* ----------------------------- Complexity ---------------------------- */
//  T(n) = 2*T(n/2) + Cn
//  T(1) = C
//  >> T(n) = Cn * logN
//
// Time: theta(n*logN)
// Space: theta(n)              (size of temp array is n)
/* ------------------------------------ X ----------------------------------- */
//  V1 slightly better than V2

pub fn do_sort(arr: &mut [u32]) {
    let high = u32::try_from(arr.len()).unwrap();
    sort(arr, 0, high - 1);
}

pub fn sort(arr: &mut [u32], low: u32, high: u32) {
    if low == high {
        return;
    }

    let mid = (low + high) / 2;

    sort(arr, low, mid);

    sort(arr, mid + 1, high);

    merge(arr, low, mid, high);
}

// This function is used to merge the 2 halves of the array.
pub fn merge(arr: &mut [u32], low: u32, mid: u32, high: u32) {
    let len = (high - low + 1) as usize;
    let mid = usize::try_from(mid).unwrap();
    let high = usize::try_from(high).unwrap();
    let low = usize::try_from(low).unwrap();
    let mut temp = vec![0; len];

    let left_start = low;
    let left_end = mid;
    let right_start = (mid + 1);
    let right_end = high;

    let left = &arr[low..mid];
    let right = &arr[mid..high];

    let mut i = left_start;
    let mut j = right_start;
    let mut k = 0;

    while i <= left_end && j <= right_end {
        if arr[i] <= arr[j] {
            temp[k] = arr[i];
            i += 1;
        } else {
            temp[k] = arr[j];
            j += 1;
        }
        k += 1;
    }

    while i <= left_end {
        temp[k] = arr[i];
        i += 1;
        k += 1;
    }

    while j <= right_end {
        temp[k] = arr[j];
        j += 1;
        k += 1;
    }

    for idx in 0..len {
        arr[left_start + idx] = temp[idx];
    }
}

#[cfg(test)]
mod test_merge_sort {
    use super::*;

    #[test]
    fn sort_ok() {
        let mut arr2 = [1, 3, 5, 2, 4, 9, 6, 8, 7];
        // merge_sort(&mut arr2);
        do_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

pub fn merge_sort_v2(arr: &mut [u32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Divide the array into two halves
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    // Recursively sort the left and right halves
    merge_sort_v2(&mut left);
    merge_sort_v2(&mut right);

    // Merge the sorted halves back into the original array
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
