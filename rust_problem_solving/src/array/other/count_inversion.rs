// Inversioṇ

//  Inversion Count for an array indicates 
//  – how far (or close) the array is from being sorted. 
//  If the array is already sorted, then the inversion count is 0, 
//  but if the array is sorted in reverse order, the inversion count is the maximum.

//  A pair a[i], a[j] form an inversion 
//  when i < j and a[i] > a[j]

use std::vec;


fn count_inversion(arr:&mut [u32],l:usize, r:usize) -> u32 {
    let mut res = 0;

    if l < r {
        let mid: usize = (l+r)/2;
        res += count_inversion(arr, l, mid);
        res += count_inversion(arr, mid+1, r);
        res += count_merge(arr, l, mid, r);
    }
    println!("{:?}",&arr);
    res
}

fn count_merge(arr:&mut [u32], low:usize, mid:usize, high:usize) -> u32{
    let len = (high - low + 1) as usize;
    let mut temp = vec![0; len];
    let mut res = 0;

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
            res += (mid-i+1);
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
    // println!("$$ arr {:?}", arr);
    res as u32
}

fn naive(arr:&[u32]) -> Option<u32>{
    let n = arr.len();
    let mut res = 0;

    for i in 0..(n-1){
        for j in i..n{
            if arr[i] > arr[j]{
                res+=1;
            }
        }
    }
    return Some(res);
}

#[cfg(test)]
mod test_inversion_count {
    use super::*;

    #[test]
    fn count_inversion_work_ok() {
        let mut arr1 = [2,4,1,3,5];
        assert_eq!(count_inversion(&mut arr1, 0, 4), 3 );

        // let mut arr2 = [10,20,30,40];
        // assert_eq!(count_inversion(&mut arr2, 0, 4), 0 );

        // let mut arr3 = [40,30,20,10];
        // assert_eq!(count_inversion(&mut arr3, 0, 4), 6 );
    }

    #[test]
    fn naive_work_ok() {
        let arr1 = [2,4,1,3,5];
        assert_eq!(naive(&arr1), Some(3) );

        let arr2 = [10,20,30,40];
        assert_eq!(naive(&arr2), Some(0) );

        let arr3 = [40,30,20,10];
        assert_eq!(naive(&arr3), Some(6) );
    }
}