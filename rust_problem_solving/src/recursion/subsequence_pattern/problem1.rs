/* ----------------- Binary strings with no consecutive 1s. ----------------- */

// Generate all binary strings of length 'N' such that there is no consecutive 1s in them

/* ------------------------------------ x ----------------------------------- */

fn binary_strings(n: usize, string: &mut String){
    if string.len() >= n  {
        println!("> {:?}", string);
        return
    }

    // Choose 0
    string.push('0');
    binary_strings(n, string);

    // Or Choose 1
    string.pop();
    string.push('1');
    binary_strings(n, string);

    // Back to Original string
    string.pop();
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_binary() {
        binary_strings(2, &mut String::new());
    }
}