// Selection Sort Algorithm
//
// Ascending order (Array -- Sorted arr + Unsorted arr)
// 1. Select the range of unsorted array (first index of unsorted_arr) and loop over outer..N
// 2. Select a min based on outer and iterate over unsorted array to find true minimum
// 3. Swap minimum to first element of unsorted arr
// 4. Iterate step 1-3 till sortedArr.len() == total len

// Worst and Avg case time - O(n^2) 
// best case time - O(n^2)
// Space - O(1)
pub fn sort(arr: &mut [u8]) {
    let len = arr.len();

    // 1. 4.
    for outer in 0..len {
        let mut min_id = outer;

        // 2.
        for inner in outer..len {
            if arr[min_id] > arr[inner] {
                min_id = inner;
            }
        }

        // 3.
        let tmp = arr[min_id];
        arr[min_id] = arr[outer];
        arr[outer] = tmp;
    }
}

#[cfg(test)]
mod _test_selection_sort {
    use super::*;

    #[test]
    fn sort_correctness() {
        let mut arr2 = [1, 3, 5, 2, 4];
        sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        // println!("{:?}",arr2);
    }
}
