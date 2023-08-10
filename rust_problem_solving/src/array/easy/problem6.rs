/* --------------------------- Problem Statement: --------------------------- */
// Problem Statement:
// Given an array of integers, rotating array of elements by k elements either left or right.

/* -------------------------------- Approach: ------------------------------- */
// 1. Using a temp array to store elements to be shifted
//
// 2. Using ” Reversal Algorithm “
//      For Rotating Elements to right
//      Step 1: Reverse the last k elements of the array
//      Step 2: Reverse the first n-k elements of the array.
//      Step 3: Reverse the whole array.
//
//      For Rotating Elements to left
//      Step 1: Reverse the first k elements of the array
//      Step 2: Reverse the last n-k elements of the array.
//      Step 3: Reverse the whole array.
//

/* ------------------------------- Complexity ------------------------------- */
// 1. time O(n), space O(k), where k is number of elements to be shifted
// 2. time O(n), space O(1)

enum Direction {
    Left,
    Right,
}

fn rotate(dir: Direction, amount: usize, arr: &mut [u32]) {
    let n = arr.len();
    match dir {
        Direction::Left => {
            reverse(&mut arr[..amount]);
            reverse(&mut arr[amount..]);
            reverse(arr);
        }
        Direction::Right => {
            reverse(&mut arr[(n-amount)..]);
            reverse(&mut arr[..(n-amount)]);
            reverse(arr);
        }
    }
}

fn reverse(arr: &mut [u32]) {
    let n = arr.len();
    let half = n / 2 as usize;
    for i in 0..half {
        (arr[i], arr[n - i - 1]) = (arr[n - i - 1], arr[i])
    }
}

#[cfg(test)]
mod test_rotate {
    use super::*;
    use Direction::*;

    #[test]
    fn test_reverse() {
        let mut arr = [1, 2, 3, 4];
        reverse(&mut arr);
        assert_eq!(arr, [4, 3, 2, 1]);
    }

    #[test]
    fn test_rotate_left() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        let k = 2;
        rotate(Left, k, &mut arr);
        assert_eq!(arr, [3, 4, 5, 6, 7, 1, 2]);
    }

    #[test]
    fn test_rotate_right() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        let k = 2;
        rotate(Right, k, &mut arr);
        assert_eq!(arr, [6, 7, 1, 2, 3, 4, 5]);
    }
}
