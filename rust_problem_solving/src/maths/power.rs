pub mod power {

    // Approach: Binary Exponentiation
    // 1. Every number can be written as sum of power of 2, for ex: 19 = 16 + 2 + 1
    // 2. We can traverse through all bits of a number (LSB to MSB) in Log(N)
    //
    // 3^(19) = 3^(16+2+1) = 3^(bit_Rep(19)) where bit(19) = 10011 (16 _ _ 2 1 )
    // From 3^(1) 3^(2) 3^(4) 3^(8) ..
    // we choose those which have 1 bit at the corresponding position in bits representation of n
    // i.e we result = 3^(16) * 3^(2) * 3^(1)
    //
    // time theta(logN); space O(1)
    pub fn best_iterative(x: u32, n: u32) -> u32 {
        let mut res = 1;
        let mut n = n;
        let mut x = x;

        while n > 0 {
            if (n & 1) == 1 {
                res *= x;
            }

            x = x * x; // prepare for next loop

            n = n >> 1; // = n/2;
        }
        res
    }

    // Time, T(n): O(logN), Space: O(logN)
    //  from T(n) = T(n/2) + theta(1)
    pub fn efficient_recursive(x: u32, n: u32) -> u32 {
        if n == 0 {
            return 1;
        }
        let temp = efficient_recursive(x, n / 2);
        let temp = temp * temp;

        if n % 2 == 0 {
            return temp;
        } else {
            return temp * x;
        }
    }

    // Time: O(n)
    pub fn v2(x: u32, n: i32) -> f64 {
        if x == 1 {
            return f64::from(1);
        }
        if n == 0 {
            return f64::from(1);
        } else if n > 0 {
            let mut res = 1;
            for i in (0)..n {
                res *= x;
            }
            return f64::from(res);
        } else {
            let mut res = f64::from(1);
            for i in (0)..(n.abs()) {
                res = res / f64::from(x);
            }
            return f64::from(res);
        }
    }

    pub fn naive(x: u32, n: u32) {
        let mut res = 1;
        for i in 0..n {
            res *= x;
        }
        println!("{}", res);
    }
}

// t: O(logN)
fn traverse_lsb_to_msb(n: u32) {
    let mut res = 1;
    let mut n = n;

    while n > 0 {
        if n % 2 == 0 {
            print!("1");
        } else {
            print!("0");
        }
        n = n / 2;
    }
}

#[cfg(test)]
mod test_powers_iterative {
    use super::power::best_iterative as power;

    #[test]
    fn check_correctness() {
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(3, 4), 81);
        assert_eq!(power(5, 4), 625);
    }
}

#[cfg(test)]
mod test_powers_v2 {
    use super::power::v2;

    #[test]
    fn n_zero_one() {
        assert_eq!(v2(10, 0), 1.0f64);
    }

    #[test]
    fn n_positive_correct() {
        assert_eq!(v2(2, 2), 4.0f64);
    }

    #[test]
    fn n_negative_one() {
        assert_eq!(v2(2, -1), 0.5f64);
    }
}
