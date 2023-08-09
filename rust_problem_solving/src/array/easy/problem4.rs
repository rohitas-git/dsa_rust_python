// Problem Statement: Given an integer array sorted in non-decreasing order, remove the duplicates in place
// such that each unique element appears only once. The relative order of the elements should be kept the same.

// Approach:
// 1. Brute: first transfer to HashMap then transfer to array  [O(n*log(n))+O(n); O(n)]
// 2. Optimal:
//      We can think of using two pointers ‘i’ and ‘j’, we move ‘j’ till we don’t get a number arr[j] which is different from arr[i].
//      As we got a unique number we will increase the i pointer and update its value by arr[j].

use funty::Integral;

fn optimal_remove_duplicates<T: Integral>(arr: &mut [T]) {
    let mut i = 0;
    let len = arr.len();

    for j in 0..len {
        if arr[j] != arr[i] {
            i += 1;
            arr[i] = arr[j];
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_removing_duplicates_ok() {
        let mut arr = [-12, 20, 99,10, 10, 12, 10, 10, 99, 21, -12];
        optimal_remove_duplicates(&mut arr);
        println!("{:?}",arr);
        // assert_eq!(arr, &mut [-12,20,99,12,10]);
    }
}
