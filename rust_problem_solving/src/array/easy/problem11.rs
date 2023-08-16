/* -------------- Count Maximum Consecutive One’s in the array -------------- */
// Problem Statement:
// Given an array that contains only 1 and 0 return the count of maximum consecutive ones in the array.

// Example 1:
// Input: prices = {1, 1, 0, 1, 1, 1}
// Output: 3

/* -------------------------------- Approach: ------------------------------- */
// 1. Using variable_count and max_variable_count


/* ------------------------------- Complexity ------------------------------- */

/* ------------------------------------ x ----------------------------------- */

fn solution(arr: &[u32]) -> u32 {
    let mut variable_count = 1;
    let mut max_variable = 0;

    for i in 0..(arr.len() - 1) {
        if arr[i] == arr[i + 1] && arr[i] == 1 {
            variable_count += 1
        } else {
            variable_count = 1;
        }
        if max_variable < variable_count {
            max_variable = variable_count;
        }
    }

    max_variable
}

fn solution_v2(arr: &[u32]) -> u32 {
    let mut count = 0;
    let mut max = 0;

    for i in 0..(arr.len()) {
        if arr[i] == 1 {
            count += 1;
        } else {
            count = 0;
        }
        max = std::cmp::max(count, max);
    }

    max
}

#[cfg(test)]
mod test_max_consecutives_in_array {
    use super::*;

    #[test]
    fn solution_ok() {
        let arr = [1, 1, 0, 0, 0, 0, 1, 1, 1];
        assert_eq!(solution(&arr), 3);
        assert_eq!(solution_v2(&arr), 3);
    }
}
