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
pub fn optimized_iterative_sort(arr: &mut [u8]) {
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
            println!("{}", inner);
            if arr[inner] > arr[inner + 1] {
                // 3.
                let tmp = arr[inner];
                arr[inner] = arr[inner + 1];
                arr[inner + 1] = tmp;
            }
        }
    }
}

// In the recursive approach, just select the range recursively instead of using any loop
// Worst and Avg case time - O(n^2)
// best case time - O(n)
// Space - O(n)
pub fn recursive_sort(arr: &mut [u8]) {
    let l = arr.len();

    if l == 1 {
        return;
    }

    let mut swapped = false;

    for inner in 0..=(l - 2) {
        if arr[inner] > arr[inner + 1] {
            // 3.
            let tmp = arr[inner];
            arr[inner] = arr[inner + 1];
            arr[inner + 1] = tmp;

            swapped = true;
        }
    }

    if !swapped {
        return;
    }

    recursive_sort(&mut arr[..(l-1)])
}

#[cfg(test)]
mod test_bubble_sort {
    use super::*;

    #[test]
    fn iterative_sort_ok() {
        let mut arr2 = [1, 3, 5, 2, 4];
        sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        // println!("{:?}", arr2);
    }

    #[test]
    fn optimized_sort_ok() {
        let mut arr2 = [1, 3, 5, 2, 4];
        optimized_iterative_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        // println!("{:?}",arr2);
    }

    #[test]
    fn recursive_sort_ok() {
        let mut arr2 = [1, 3, 5, 2, 4];
        recursive_sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        // println!("{:?}",arr2);
    }
}
