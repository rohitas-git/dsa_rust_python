
// Steps
// 1. Pick a pivot - {last, first, mid, arbitary}
//      && Place the pivot in the place it would be in the sorted array
// 2. Smaller on the left and greater on the right of pivot


fn sort(arr: &mut [u32]){
    if arr.len() <= 1{
        return;
    }
    let pivot_position = 0;
    let parition = parition_wrt_pivot(arr, pivot_position);

    sort(&mut arr[..parition]);
    sort(&mut arr[parition+1..]);
}

fn parition_wrt_pivot(arr: &mut [u32], pivot_position: usize) -> usize{
    let n = arr.len();
    let mut i = 0;
    let mut j = n-1;
    let pivot = arr[pivot_position];

    // smaller elements on the left and greater on the right 
    while j >= i {
        // stop at position from front whose element is greater than pivot
        while arr[i] <= pivot {
            i+=1;
        }
        // stop at position whose from end element is smaller than pivot
        while arr[j] >= pivot {
            j-=1;
        }
        // i and j crossed each other -> break loop
        if j < i{
            break;
        }
        // then swap them as smaller item should left of pivot
        dbg!(i,j);
        (arr[i], arr[j]) = (arr[j], arr[i]);
        
        // std::mem::swap(arr.get_mut(i).unwrap(), arr.get_mut(j).unwrap());
    }
    // put pivot to its correct place
    (arr[pivot_position], arr[j]) = (arr[j], arr[pivot_position]);
    
    j
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_quicksort() {
        let arr = &mut [4,1,9,2,6,8,3];
        sort(arr);
        dbg!(arr);
    }
}