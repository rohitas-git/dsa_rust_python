pub mod palindrome{

    pub fn iterative(s: &str) -> bool {
        let total = s.chars().count();
        let mid: usize = total / 2;
        let mut ch = s.chars();
    
        for i in 0..mid {
            if ch.next().unwrap() != ch.next_back().unwrap() {
                return false;
            }
        }
        true
    }

    pub fn recursive(i:usize, s: &str)->bool{
        let total = s.chars().count();
        let mid: usize = total / 2;
        let mut ch = s.chars();

        if i  >= mid{
            return true
        }

        if ch.nth(i) != ch.nth_back(i){
            return false
        }

        return recursive(i+1, s);
    }

}


#[cfg(test)]
mod test_palindrome_iterative {
    use super::palindrome::iterative as palindrome;

    #[test]
    fn true_for_correct_input() {
        let string_1 = "ABCDCBA";
        assert_eq!(palindrome(string_1), true);
    }

    #[test]
    fn false_for_incorrect_input() {
        let string_2 = "MORNING";
        assert_eq!(palindrome(string_2), false);
    }
}


#[cfg(test)]
mod test_palindrome_recursive {
    use super::palindrome::recursive as palindrome;

    #[test]
    fn true_for_correct_input() {
        let string_1 = "ABCDCBA";
        assert_eq!(palindrome(0,string_1), true);
    }

    #[test]
    fn false_for_incorrect_input() {
        let string_2 = "MORNING";
        assert_eq!(palindrome(0,string_2), false);
    }
}





