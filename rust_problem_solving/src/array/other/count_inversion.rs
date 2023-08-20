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
    res
}

fn count_merge(arr:&mut [u32], l:usize, mid:usize, r:usize) -> u32{
    let mut left = arr[l..(mid+1)].to_vec();
    let mut right = arr[(mid+1)..(r+1)].to_vec();
    let n = left.len();
    let m = right.len();
    let (mut res, mut i, mut j, mut k) = (0,0,0,l);
    while i < n && j < m {
        if left[i] <= right[j]{
            arr[k] = left[i];
            i+=1;
        }
        else {
            arr[k] = right[j];
            j+=1;
            res += (n-i);
        
        }
        k+=1;        
    }
    
    while i < n {
        arr[k] = left[i];
        k+=1;
        i+=1;
    }
    while j < m {
        arr[k] = right[j];
        k+=1;
        j+=1;
    }
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
        let n = arr1.len();
        assert_eq!(count_inversion(&mut arr1, 0, n-1), 3 );

        let mut arr2 = [10,20,30,40];
        let n = arr2.len();
        assert_eq!(count_inversion(&mut arr2, 0, n-1), 0 );

        let mut arr3 = [40,30,20,10];
        let n = arr3.len();
        assert_eq!(count_inversion(&mut arr3, 0, n-1), 6 );
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