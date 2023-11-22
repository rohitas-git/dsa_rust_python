/* --------------------------- Implement Pow(x,n) --------------------------- */
// Problem Statement: Given a double x and integer n, calculate x raised to power n.
// Basically Implement pow(x, n).

/* ------------------------------------ x ----------------------------------- */

// O(logN), O(1)
fn pow(x: i32, n: i32) -> f64 {
    let mut ans = 1.0;
    let mut nn = n.abs();
    let mut x = x as f64;

    while nn > 0 {
        // 2^5 = 2 * 2^4
        if nn % 2 == 1 {
            ans *= x;
            nn -= 1;
        }
        // 2^10 = (2*2)^5
        else {
            x = x * x;
            nn /= 2;
        }
    }
    if n < 0 {
        1.0 / ans
    } else {
        ans
    }
}

// O(logN), O(logN)
fn pow_rec(x: i32, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    if n < 0 {
        1.0 / pow(x, n.abs())
    } else if n % 2 == 0 {
        pow_rec(x * x, n.abs() / 2)
    } else {
        pow_rec(x, n.abs() - 1) * x as f64
    }
}

// O(n), O(1)
fn brute(x: i32, n: i32) -> f64 {
    let nn = n.abs();
    let mut ans = 1;
    for _i in 0..nn {
        ans *= x;
    }

    if n >= 0 {
        ans as f64
    } else {
        1.0 / ans as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brute_test() {
        dbg!(pow_rec(2, 3));
        dbg!(pow_rec(2, -3));
        dbg!(pow_rec(2, 0));
    }
}
