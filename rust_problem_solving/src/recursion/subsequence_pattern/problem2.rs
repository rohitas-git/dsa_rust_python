
/* ------------------------ Generate all parenthesis ------------------------ */

// generate combinations of well formed parenthesis having 'N' pairs


/* ------------------------------------ x ----------------------------------- */

fn parenthesis_combinations(n: usize, string: &mut String, extras: i32){
    if n < 2 || n % 2 != 0{
        return 
    }
    if string.len() >= n {
        if extras == 0{
            println!("> {:?}", string);
        }
        return
    }
    
    // Choose 0
    string.push('(');
    parenthesis_combinations(n, string, extras+1);
    
    // Or Choose 1
    string.pop();
    if extras > 0{
        string.push(')');
        parenthesis_combinations(n, string, extras-1);
        // Back to Original string
        string.pop();
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_parenthesis() {
        parenthesis_combinations(10, &mut String::new(), 0);
    }
}