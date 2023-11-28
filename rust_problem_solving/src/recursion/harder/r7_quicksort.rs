
/* ---------------------------------- Quick Sort --------------------------------- */
// 1. Pick a pivot - {last, first, mid, arbitary}
//      && Place the pivot in the place it would be in the sorted array
// 2. Smaller on the left and greater on the right of pivot

/* ------------------------------- Complexity ------------------------------- */

// parition_wrt_pivot:
// time: O(n), space: O(1)

// sort:
// time: O(NlogN), space O(logN)

/* ------------------------------------ x ----------------------------------- */
fn sort(arr: &mut [u32]){
    if arr.len() <= 1{
        return;
    }

    // Note: will need to rewrite parition function for a different pivot
    let pivot_position = 0;

    // parition arr wrt pivot and return position of pivot/parition point
    // i.e smaller on left of pivot and greater on right of pivot
    let parition = parition_wrt_pivot(arr, pivot_position);
    
    // sort the part left of pivot
    sort(&mut arr[..parition]);

    // sort the part right of pivot
    sort(&mut arr[(parition+1)..]);
}

// when chosen pivot is first element, arr[0]
fn parition_wrt_pivot(arr: &mut [u32], pivot_position: usize) -> usize{
    let n = arr.len();
    let mut i = 0;
    let mut j = n-1;
    let pivot = arr[pivot_position];

    // smaller elements on the left and greater on the right 
    while j >= i {
        
        // smaller,greater : EITHER <=, > OR <, >= in while condn comparison

        // stop at position from front whose element is greater than pivot
        while arr[i] <= pivot && i < n{
            i+=1;
        }
        
        // stop at position whose from end element is smaller than pivot
        while arr[j] > pivot && j > 0{
            j-=1;
        }
        
        // i and j crossed each other -> break loop
        if j < i{
            break;
        }
        
        // else  swap them as smaller item should left of pivot
        (arr[i], arr[j]) = (arr[j], arr[i]);
        
    }
    
    // put pivot to its correct place
    (arr[pivot_position], arr[j]) = (arr[j], arr[pivot_position]);
    
    j   // position of parition point
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_quicksort() {
        let arr = &mut [4,1,9,2,6,8,3];
        sort(arr);
        assert_eq!([1,2,3,4,6,8,9], *arr);
    }
}