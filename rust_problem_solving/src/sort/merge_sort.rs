/* ------------------------------- Merge Sort ------------------------------- */
// Intuition

// 1.
// MergeSort is a divide and conquers algorithm,
// - it divides the given array into equal parts and then merges the 2 sorted parts.
// - divide the given array into two sorted parts and merge them

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

pub fn call_sort(arr: &mut [u32]) {
    let high = u32::try_from(arr.len()).unwrap();
    let high = high - 1;
    sort(arr, 0, high);
}

pub fn sort(arr: &mut [u32], low: u32, high: u32) {
    if low >= high {
        return;
    }

    let mid = (low + high) / 2;

    // sort the left half
    sort(arr, low, mid);

    // sort the right half
    sort(arr, mid + 1, high);

    // merge the sorted halves
    merge(arr, low, mid, high);
}

pub fn merge(arr: &mut [u32], low: u32, mid: u32, high: u32) {
    // 1.
    let len = usize::try_from(high - low + 1).unwrap();
    let mut temp = vec![];

    // 2.
    let mut left = usize::try_from(low).unwrap();
    let mut right = usize::try_from(mid + 1).unwrap();

    let mid = usize::try_from(mid).unwrap();
    let high = usize::try_from(high).unwrap();
    let low = usize::try_from(low).unwrap();

    // 3.
    while (left <= mid) && (right <= high) {
        if arr[left] <= arr[right] {
            temp.push(arr[left]);
            left += 1;
        } else {
            temp.push(arr[right]);
            right += 1;
        }
    }

    // 4.
    while (left <= mid) {
        temp.push(arr[left]);
        left += 1;
    }
    while (right <= high) {
        temp.push(arr[right]);
        right += 1;
    }

    // 5.
    for i in low..high {
        arr[i] = temp[i - low]
    }
}

#[cfg(test)]
mod test_selection_sort {
    use super::*;

    #[test]
    fn sort_ok() {
        let mut arr2 = [1, 3, 5, 2, 4, 9, 6, 8, 7];
        // call_sort(&mut arr2);
        sort(&mut arr2, 0, 8);
        assert_eq!(arr2, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // println!("{:?}",arr2);
    }
}
