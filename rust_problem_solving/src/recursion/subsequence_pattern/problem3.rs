/* ------ Power Set: Print all the possible subsequences of the String ------ */

// Problem Statement: Given a string, find all the possible subsequences of the string.

/* ----------------------------------- Sol ---------------------------------- */
// 1. Using bit manipulation
// check whether the ith bit is set or not
// First, write down all the numbers from 0 to 2^(n)-1 and their bit representation.
// 0 means I am not picking the character in my subsequence,
// and 1 means I am picking the character.

// Binary Left Shift Operator (<<).
// The left operands value is moved left by the number of bits specified by the right operand.
// Binary Right Shift Operator(>>). The left operands value is moved right by the number of bits specified by the right operand.

// Basically, we traverse from 0 to 2^(n)-1 and check for every number if their bit is set or not. 
// If the bit is set add that character to your subsequence.

/* ------------------------------------ x ----------------------------------- */

// Time: O(2^n * n)
// Space: O(1)
fn subsequences(string: &str) {
    let n = string.len();

    let mut ans: Vec<String> = vec![];
    
    for num in 0..(1 << n) {
        let mut sub = String::new();
        let mut chars = string.chars();

        for j in 0..n {
            
            //check if the jth bit is set or not
            if (num & (1 << j)) == 1 {
                sub.push(chars.next().unwrap());
            }
        }

        if !sub.is_empty() {
            ans.push(sub);
        }
    }
    println!("{:?}", ans);
}

// 2. Using  recursion
// Time - O(2^n)
// Number of subsequences - 2^n  [n = arr.len()]
// Space - O(n)     [because of call stack]
fn print_subsequence_recursion(arr: &[u32], index: usize, subequence: &mut Vec<u32>) {
    if index >= arr.len() {
        println!("{:?}", subequence);
        return;
    }

    subequence.push(arr[index]);
    print_subsequence_recursion(arr, index + 1, subequence);

    subequence.pop();
    print_subsequence_recursion(arr, index + 1, subequence);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let string = "123";
        subsequences(string);
    }
}
