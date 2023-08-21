// Analysis of Quick Sort after test cases

fn quick_sort_hoare(arr: &mut [u32], l: usize, h: usize) {
    if l < h {
        let p = parition::hoare_parition(arr, l, h);
        // let p =0;
        // println!("l:{} h:{} p:{}", l, h, p);
        quick_sort_hoare(arr, l, p);
        quick_sort_hoare(arr, p + 1, h);
    }
}

mod parition {

    // >> copied from Array/other/
    pub fn hoare_parition(arr: &mut [u32], l: usize, h: usize) -> usize {
        // pivot - first element of array
        let pivot = arr[l];

        let mut i = l;
        let mut j = h;

        loop {
            while arr[i] < pivot {
                i += 1;
            }
            while arr[j] > pivot {
                j -= 1;
            }
            // when i, j violate the above rules, then run below code

            // if i and j cross each other, return j i.e last item index of [<=] Parition
            if i >= j {
                return j;
            }

            // swap the violating i and j
            (arr[i], arr[j]) = (arr[j], arr[i]);
            j -= 1;
            i += 1;
        }
    }
}

#[cfg(test)]
mod test_hoare_quick_sort {
    use super::*;

    #[test]
    fn quick_sort_hoare_accurate() {
        let mut arr = [10, 110, 30, 90, 20, 50];
        quick_sort_hoare(&mut arr, 0, 5);
        assert_eq!(arr, [10, 20, 30, 50, 90, 110]);
    }
}

/* -------------------------------- Best Case ------------------------------- */
//
//                  [0..........7]
//        [0...3]                      [4......7]
//    [0..1]    [2..3]             [4...5]      [6..7]
//   [0]   [1] [2]   [3]         [4]   [5]   [6]     [7]
//

// Work at every level = theta(N)
// total levels = theta(logN)
// Total work = theta(N logN)
//
// Time complexity for best case - theta(N*logN)

/* ------------------------------- Worst Case ------------------------------- */
//
//                  [0..........n-1]
//             [0]                  [1......n-1]
//                                [1]            [2......n-1]
//                                              [2]         [3.......n-1]
// ....
//  (can also be zig-zag)

// Worst case happens when:
//  when parition is dividing the array in such a way that
//      - you one element on one side
//      - remaining elements on another side
//
// Ex:
// If Array is sorted or reverse sorted,
// and you're picking the first element or last element
// Then you are going to enter worst case

// Total work at each level (k) -> theta(1) + theta(n - k)
// Number of levels - N times
//
// Total work for worst case:
//      theta(1) * theta(N) + theta( N + N-1 + N-2 ....1)
//
// =>   theta(N^2) time complexity for worst case

/* ------------------------------ Average Case ------------------------------ */

// Not doing it here because  we'll have to
// -consider every possible input
// -compute time for every possible input
// -divide the sum of times by total no. of inputs

// recursive trees in avg case will not be prefect tree
// we will only be able to calculate upper bounds
//
// finally, we will O(N*logN) time complexity
