// Bubble Sort algorithm
//
// Ascending order (Array -- Unsorted arr + Sorted arr )
// 1. Select the range of unsorted array (last index of unsorted_arr) and loop over 0..outer
// 2. Find maximum by comparing adjacent items by iterate over unsorted array
// 3. Keep swapping maximum till end of unsorted arr
// 4. Iterate step 1-3 (n-1  outer loop iterations )


// Worst and Avg case time - O(n^2) 
// best case time - O(n)
// Space - O(1)
pub fn optimized(arr: &mut [u8]) {
    let len = arr.len();

    // 1. 4.
    for outer in (1..len).rev() {
        let mut swapped = false;
        // 2.
        for inner in 0..=(outer - 1) {
            if arr[inner] > arr[inner + 1] {
                // 3.
                let tmp = arr[inner];
                arr[inner] = arr[inner + 1];
                arr[inner + 1] = tmp;

                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

// Worst and Avg case time - O(n^2) 
// best case time - O(n^2) when unoptimized
// Space - O(1)
pub fn sort(arr: &mut [u8]) {
    let len = arr.len();

    // 1. 4.
    for outer in (1..len).rev() {
        // 2.
        for inner in 0..=(outer - 1) {
            if arr[inner] > arr[inner + 1] {
                // 3.
                let tmp = arr[inner];
                arr[inner] = arr[inner + 1];
                arr[inner + 1] = tmp;
            }
        }
    }
}


#[cfg(test)]
mod test_bubble_sort {
    use super::*;

    #[test]
    fn correctness() {
        // let mut arr = [12, 1, 6, 31, 5];
        // sort(&mut arr);
        // assert_eq!(arr, [1, 5, 6, 12, 31]);

        let mut arr2 = [1, 3, 5, 2, 4];
        sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        println!("{:?}", arr2);
    }

    #[test]
    fn optimized_sort_ok() {
        // let mut arr = [12, 1, 6, 31, 5];
        // optimized(&mut arr);
        // assert_eq!(arr, [1, 5, 6, 12, 31]);

        let mut arr2 = [1, 3, 5, 2, 4];
        optimized(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        // println!("{:?}",arr2);
    }
}
