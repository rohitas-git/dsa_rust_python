/* -------------------- Problems on Functional Recursion -------------------- */

mod palindrome{
    fn is_palindrome(string: &str) -> bool{
        let chars = get_chars(string);
        is_palindrome_1(&chars)
    }

    fn is_palindrome_1(chars: &[char]) -> bool{
        let n = chars.len();
        if n <= 1{
            return true;
        }
        else if chars[0] != chars[n-1]{
            return false;
        }
        is_palindrome_1(&chars[1..(n-1)])
    }

    fn get_chars(string: &str) -> Vec<char>{
        string.chars().collect()
    } 

    #[cfg(test)]
    mod test_super {
        use super::*;
    
        #[test]
        fn test_palindrome() {
            let string = "madam";
            assert!(is_palindrome(string));

            let string = "11211";
            assert!(is_palindrome(string));

            let string = "aabbcc";
            assert!(!is_palindrome(string));
        }
    }
}

mod reverse_arr {
    fn reverse_array(arr: &mut [u32]) {
        let _n = arr.len();

        reverse_array_1(arr, 0, _n - 1);
        // reverse_array_2(arr, 0);
    }

    fn reverse_array_2(arr: &mut [u32], i: usize) {
        let n = arr.len();
        if i >= n / 2 {
            return;
        }
        swap(arr, i, n - 1 - i);
        reverse_array_2(arr, i + 1);
    }

    fn reverse_array_1(arr: &mut [u32], l: usize, r: usize) {
        if l >= r {
            return;
        }
        swap(arr, l, r);
        reverse_array_1(arr, l + 1, r - 1);
    }

    fn swap(arr: &mut [u32], l: usize, r: usize) {
        (arr[l], arr[r]) = (arr[r], arr[l])
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_reversal() {
            let arr = &mut [1, 2, 3, 4, 5];
            reverse_array(arr);
            dbg!(arr);
        }
    }
}
