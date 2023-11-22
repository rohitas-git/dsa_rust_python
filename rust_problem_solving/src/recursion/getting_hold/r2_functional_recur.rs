/* ----------------- Parameterized and Functional Recursion ----------------- */
// Sum of first N numbers
// Factorial of N

/* ------------------------------------ x ----------------------------------- */
// Parameteric way
// factorial(n, res) -> factorial(n-1, res*n) and return in base condition

// Functional way
// factorial(n) -> return n * factorial(n-1)

/* ------------------------------------ x ----------------------------------- */
// all fns complexity -  O(n), O(n)

mod parametrize_way {
    // store wanted result with parameters
    // and do calculation in the parameters itself

    fn sum_till(n: u32, sum: u32) {
        if n < 1 {
            println!("{}", sum);
            return;
        }
        sum_till(n - 1, sum + n);
    }

    fn factorial(n: u32, res: u32) {
        if n == 1 {
            println!("{}", res);
            return;
        }
        factorial(n - 1, res * n)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sum() {
            sum_till(100, 0);
        }

        #[test]
        fn test_factorial() {
            factorial(5, 1);
        }
    }
}

mod functional_way {
    // don't want parameter to do the work
    // want function to return the answer, i.e return is not ()

    // do calculations using fn's return and not in parameters

    fn sum(n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        n + sum(n - 1)
    }

    fn factorial(n: u32) -> u32 {
        if n == 1 {
            return 1;
        }
        n * factorial(n - 1)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_sum() {
            dbg!(sum(100));
        }

        #[test]
        fn test_factorial() {
            dbg!(factorial(4));
        }
    }
}
