// Insertion Sort algorithm
//
// Ascending order (Array -- Sorted arr + Unsorted arr)
// 1. Select an element in each iteration from the unsorted array(using a loop).
// 2. Place it in its corresponding position in the sorted part and shift the remaining elements accordingly (using an inner loop and swapping).
// 3. Iterate step 1-2 (n-1  outer loop iterations )

// Worst and Avg case time - O(n^2) 
// best case time - O(n)
// Space - O(1)

pub fn sort(arr: &mut [u8]) {
    let len = arr.len();

    // 1. 3.
    for outer in (0..len) {
        let selected = outer;

        // 2.
        for inner in (1..=selected).rev() {

            if arr[inner] >= arr[inner-1] {
                break;
            }
            else {
                // 3.
                let tmp = arr[inner];
                arr[inner] = arr[inner - 1];
                arr[inner - 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod test_bubble_sort {
    use super::*;

    #[test]
    fn ok_worst_case() {

        let mut arr2 = [1, 3, 5, 2, 4];
        sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        println!("{:?}", arr2);
    }

    #[test]
    fn ok_best_case() {

        let mut arr2 = [1, 2, 3, 4, 5];
        sort(&mut arr2);
        assert_eq!(arr2, [1, 2, 3, 4, 5]);

        println!("{:?}", arr2);
    }
}