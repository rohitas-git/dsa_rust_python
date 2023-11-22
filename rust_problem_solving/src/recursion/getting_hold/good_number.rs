// A number is called good number
// if it's every digit (except the rightmost digit) is larger
// than the sum of digits on the right side of that digit

// ex: 843 850

// Problem: Find all good numbers in a..=b such that none of them contains the specified digit

use std::vec;

fn print_all_good_numbers(a: u32, b: u32, d: u32) {
    if b < a {
        return;
    }

    if !has_digit(a, d) && is_good_number(a) {
        println!("> {:?}", a);
    }

    print_all_good_numbers(a + 1, b, d);
}

// O(logN), O(logN)
fn is_good_number(a: u32) -> bool {
    let digits = get_digits_reverse(a);
    let check = true;
    let mut sum = digits[0];

    for digit in digits.iter().skip(1) {
        if *digit <= sum {
            return false;
        }
        sum += digit;
    }
    check
}

fn has_digit(a: u32, target: u32) -> bool {
    get_digits_reverse(a).contains(&target)
}

// O(log N), O(log N)
fn get_digits_reverse(a: u32) -> Vec<u32> {
    let mut digits = vec![];
    let mut num = a;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let a = 1241;
        dbg!(get_digits_reverse(a));
    }

    #[test]
    fn test_goodness() {
        let a = 5;
        dbg!(is_good_number(a));
    }

    #[test]
    fn test_all_goodness() {
        let a = 20;
        let b = 45;
        dbg!(print_all_good_numbers(a,b,1));
    }
}
