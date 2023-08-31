/* -------------------------- Peak element in Array ------------------------- */
// Problem Statement: 
// Given an array of length N. 
// Peak element is defined as the element greater than both of its neighbors. 
// Formally, if ‘arr[i]’ is the peak element, ‘arr[i – 1]’ < ‘arr[i]’ and ‘arr[i + 1]’ < ‘arr[i]’. 
// Find the index(0-based) of a peak element in the array. 
// If there are multiple peak numbers, return the index of any peak number.

/* -------------------------------- Approach -------------------------------- */

// Conditions are enough to handle the array with multiple peaks. 
// Based on the observation, in an array with multiple peaks, 
// an index has four possible positions as follows:
// 
// - The index is a trough where a decreasing sequence ends and an increasing sequence begins.
// - The index might be on the left half.
// - The index might be the peak itself.
// - The index might be on the right half.

// arr[i] is the peak, arr[i] > arr[i-1] and arr[i] > arr[i+1]
//      But there are the following edge cases:
//          -> i=0, i=n-1, n=1
// If arr[i] > arr[i-1]: we are in the left half.
// If arr[i] > arr[i+1]: we are in the right half.
// If we in trough, we can eliminate either left or right half. 
//      Because both halves of such index contains a peak

/* ------------------------------------ x ----------------------------------- */

fn get_peak_element(arr:&[u32]) -> usize{
    let n = arr.len();
    let mut peak = u32::MAX as usize;

    // Edge cases for arr[i] is peak
    if n==1{
        return 0;
    }
    if arr[n-1] > arr[n-2]{
        return n-1;
    }
    if arr[0] > arr[1]{
        return 0;
    }

    let mut low = 1;
    let mut high = n-1;

    while low<=high {
        let mid = low + (high - low) / 2;

        // If arr[mid] is the peak:
        if arr[mid] > arr[mid+1] && arr[mid] > arr[mid-1]{
            peak = mid;
            break;
        }

        // If arr[mid] is left of peak
        if arr[mid] > arr[mid-1]{
            low = mid+1;
        }
         // If arr[mid] is right of peak 
         // Or, arr[mid] is trough 
        else {
            high = mid-1;
            // low = mid+1; // choose either of them in case of trough
        }
    }
    peak
}

#[cfg(test)]
mod test_getting_peak_element {
    use super::*;

    #[test]
    fn test_solution() {
        let arr = [1,2,3,4,5,6,7,8,5,1];
        let res = get_peak_element(&arr);
        assert_eq!(res, 7);
        println!("{}",arr[res]);

        let arr = [1,2,1,3,5,6,4];
        let res = get_peak_element(&arr);
        assert_eq!(res, 5);
        println!("{}",arr[res]);

        let arr = [1,2,3,4];
        let res = get_peak_element(&arr);
        assert_eq!(res, 3);
        println!("{}",arr[res]);

        let arr = [5,1,2,3];
        let res = get_peak_element(&arr);
        // assert_eq!(res, 0);
        assert_eq!(res, 3);
        println!("{}",arr[res]);

        let arr = [1,2,3,4];
        let res = get_peak_element(&arr);
        assert_eq!(res, 3);
        println!("{}",arr[res]);

    }
}