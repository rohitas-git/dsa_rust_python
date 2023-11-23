// Time: O(NlogN) ??
// Space: O(n)  ??
fn sort(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at_mut(mid);
    // let left = &mut arr[..mid];
    // let right = &mut arr[mid..];
    sort(left);
    sort(right);

    merge(arr);
}

// Time: O(n)
// Space: O(n)      ,where n - len of arr
//
// Note: Tried to use iterators instead of indexes but not successful now
fn merge(arr: &mut [u32]) {
    let mid = arr.len() / 2;

    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

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

// When included in sort, gives error due to multiple mutable borrows  
fn merge_iterator(left: &mut [u32], right: &mut [u32], arr: &mut [u32]) {
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();

    let mut index = 0;

    while let (Some(&left_val), Some(&right_val)) = (left_iter.peek(), right_iter.peek()) {
        if left_val <= right_val {
            arr[index] = *left_iter.next().unwrap();
        } else {
            arr[index] = *right_iter.next().unwrap();
        }
        index += 1;
    }

    while let Some(&left_val) = left_iter.next() {
        arr[index] = left_val;
        index += 1;
    }

    while let Some(&right_val) = right_iter.next() {
        arr[index] = right_val;
        index += 1;
    }
}

#[cfg(test)]
#[test]
fn test_sorting() {
    let mut data = vec![12, 11, 13, 5, 6, 7];
    println!("Original Vector: {:?}", data);

    sort(&mut data);
    println!("Sorted Vector: {:?}", data);
    assert_eq!(data, vec![5, 6, 7, 11, 12, 13]);
}
