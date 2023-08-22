/* -------------------- Rearrange Array Elements by Sign -------------------- */

// There’s an array ‘A’ of size ‘N’ with an equal number of positive and negative elements.
// Without altering the relative order of positive and negative elements,
// you must return an array of alternately positive and negative values.
// Note: Start the array with positive elements.

// Input:
// arr[] = {1,2,-3,-1,-2,-3}, N = 6
// Output:
// 1 -3 2 -1 3 -2
/* -------------------------------- Approach -------------------------------- */

/* ------------------------------------ x ----------------------------------- */

use std::vec;

// time - O(N), space - O(N)
fn better(arr: &[i32]) -> Result<Vec<i32>, String> {
    let mut pos_index = 0;
    let mut neg_index = 1;

    // println!("l:{} r:{}",arr[arr.len()/2], arr[arr.len()/2 + 1]);
    if arr[arr.len() / 2 - 1] < 0 || arr[arr.len() / 2] > 0 {
        return Err("Positive and Negatives should have same quantity".to_string());
    }

    let mut tmp: Vec<i32> = vec![0i32; arr.len()];
    for i in 0..arr.len() {
        if arr[i] < 0 {
            tmp[neg_index] = arr[i];
            neg_index += 2;
        } else {
            tmp[pos_index] = arr[i];
            pos_index += 2;
        }
    }
    Ok(tmp)
}

// time - O(N) + O(N), space - O(N/2) + O(N/2)
fn brute(arr: &mut [i32]) -> Result<(), String> {
    let mut positives = Vec::new();
    let mut negatives = Vec::new();
    positives.extend_from_slice(&arr);
    negatives.extend_from_slice(&arr);

    let mut positives = positives
        .into_iter()
        .filter(|x| *x > 0)
        .collect::<Vec<i32>>();

    let mut negatives = negatives
        .into_iter()
        .filter(|x| *x <= 0)
        .collect::<Vec<i32>>();

    if positives.len() != negatives.len() {
        return Err("Positives and Negatives should have same quantity".to_string());
    }

    for i in 0..(arr.len()) {
        if i % 2 == 0 {
            arr[i] = positives[i / 2];
        } else {
            arr[i] = negatives[i / 2];
        }
    }
    Ok(())
}

#[cfg(test)]
mod test_rearranging_array_by_sign {
    use super::*;

    #[test]
    fn better_ok() {
        let arr = [1, 2, -3, -1, -2, -3];
        assert_eq!(
            better(&arr),
            Err("Positives and Negatives should have same quantity".to_string())
        );

        let mut arr = [1, 2, 3, -1, -2, -3];
        let res = better(&arr);
        assert_eq!(res.unwrap(), [1, -1, 2, -2, 3, -3]);
    }

    #[test]
    fn brute_ok() {
        let mut arr = [1, 2, -3, -1, -2, -3];
        assert_eq!(
            brute(&mut arr),
            Err("Positive and Negatives should have same quantity".to_string())
        );

        let mut arr = [1, 2, 3, -1, -2, -3];
        brute(&mut arr);
        assert_eq!(arr, [1, -1, 2, -2, 3, -3]);
    }
}
