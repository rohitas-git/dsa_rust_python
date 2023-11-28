/* ------------------------- Palindrome Partitioning ------------------------ */

// Problem Statement:
// You are given a string s, partition it in such a way that every substring is a palindrome.
// Return all such palindromic partitions of s.

/* -------------------------------- Approach -------------------------------- */

// 1. Select a substring and check if its palindrome

/* ------------------------------------ x ----------------------------------- */
fn palindrome_paritions(string: &str, ans: &mut Vec<Vec<String>>, curr: &mut Vec<String>) {
    if string.len() == 0 {
        ans.push(curr.clone());
        return;
    }

    for i in 1..=string.len() {
        let substring = &string[..i];
        let check = is_palindrome(substring);
        if check {
            curr.push(substring.to_string());
            palindrome_paritions(&string[i..], ans, curr);
            curr.pop();
        }
    }
}

fn is_palindrome(string: &str) -> bool {
    if string.len() <= 1 {
        return true;
    }
    for (front_char, back_char) in string.chars().zip(string.chars().rev()) {
        if front_char != back_char {
            return false;
        }
    }
    true
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_check() {
        let s = "bb";
        dbg!(is_palindrome(s));
        dbg!(&s[2..s.len()]);
    }

    #[test]
    fn test_soln() {
        let curr = &mut vec![];
        let res = &mut vec![vec![]];
        palindrome_paritions("aab", res, curr);
        dbg!(res);

        let curr = &mut vec![];
        let res = &mut vec![vec![]];
        palindrome_paritions("aabb", res, curr);
        dbg!(res);
    }
}
