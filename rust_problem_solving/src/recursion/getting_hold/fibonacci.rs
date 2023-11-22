// F(N) = F(N-1) + F(N-2)
// F(1) = 1, F(O) = 0

fn view_fibonacci_series(n: u32) {
    for i in 0..=n {
        println!("{}: {}", i, fibonacci_iterative(i));
    }
}

// time theta(n), space O(1)
fn fibonacci_iterative(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    } else {
        let mut sec_last = 0;
        let mut last = 1;
        let mut cur = 1;

        for i in 2..=n {
            cur = last + sec_last;

            sec_last = last;
            last = cur;
        }
        return cur;
    }
}

// T(n) = T(n-1) + T(n-2) + O(1)
// T(n) = theta( (1.6180)^n )
// 
// time theta(2^n), space O(n)
pub fn fibonacci_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    else if n == 1 {
        return 1;
    }
    else{
        return fibonacci_recursive(n-1) + fibonacci_recursive(n-2)
    }
}

#[cfg(test)]
mod test_fibonacci {
    use super::*;

    #[test]
    fn test_iterative_ok() {
        let n = 6;
        let res = fibonacci_iterative(n);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_recursive_ok() {
        let n = 6;
        let res = fibonacci_recursive(n);
        assert_eq!(res, 8);
    }
}
