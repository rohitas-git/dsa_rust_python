/* ---------------------------- More subsequence ---------------------------- */

use std::collections::HashSet;

fn more_subsequence(a: &str, b: &str) {
    let count_a = count_distinct_subequences(a);
    let count_b = count_distinct_subequences(b);

    if count_a >= count_b{
        println!("{}", a);
    }
    else{
        println!("{}", b);
    }
}

fn count_distinct_subequences(string: &str) -> usize {
    let mut set: HashSet< Vec<char>> = HashSet::new();
    let chars: Vec<char> = string.chars().collect();
    get_subsequence_recursion(&chars, 0, &mut Vec::new(), &mut set);
    set.len()
}

fn get_subsequence_recursion(arr: &[char], index: usize, subequence: &mut Vec<char>, store: &mut HashSet<Vec<char>>) {
    if index >= arr.len() {
        store.insert(subequence.clone());
        return;
    }

    subequence.push(arr[index]);
    get_subsequence_recursion(arr, index + 1, subequence, store);

    subequence.pop();
    get_subsequence_recursion(arr, index + 1, subequence, store);
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_more() {
        let a = "abc";
        let b = "defg";
        more_subsequence(a, b);

        let a = "aaa";
        let b = "da";
        more_subsequence(a, b);

        let a = "aaa";
        let b = "ddddd";
        more_subsequence(a, b);
    }
}