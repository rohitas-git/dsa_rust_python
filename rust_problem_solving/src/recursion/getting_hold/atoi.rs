// Problem Statement:
// Convert a given string into 32-bit signed integer.

// Steps:
// 1. Discarding any leading whitespaces
// 2. If next character (if not at end of string) is '-' or '+',
//      let it determine sign of integer. Default is positive
// 3. Read and accumulate digits until a non-digit char is encountered or reached end of input
// 4. Convert collected digits into an integer. If no digits read, then integer is 0
// 5. If it falls outside i32 range, constrain it.
// 6. Return the integer

// Example
// Input:   45rohit12
// Output: 45

fn atoi_recursive(s: &str, sign: Option<bool>, collected: Option<u32>) -> i32 {
    let s = s.trim();
    let mut its_char = s.chars();
    let this_char = its_char.next();
    let mut sign = sign;

    // Base condition
    if this_char.is_none() || this_char.unwrap().is_alphabetic() {
        if collected.is_none() {
            return 0;
        }
        if sign.is_none() || sign.unwrap() {
            if collected.unwrap() > i32::MAX as u32 {
                return i32::MAX;
            }
            return collected.unwrap() as i32;
        }
        if !sign.unwrap() {
            if collected.unwrap() > i32::MAX as u32 + 1 {
                return i32::MIN;
            }
            return -(collected.unwrap() as i32);
        }
    }

    let new_s = its_char.as_str();
    let this_char = this_char.unwrap();
    
    // Getting sign 
    let mut new_collected = collected;
    if collected.is_none() {
        match this_char {
            '+' => {sign = Some(true); return atoi_recursive(new_s, sign, collected); },
            '-' => {sign = Some(false); return atoi_recursive(new_s, sign, collected); },
            _ => {new_collected = Some(0);}
        }
    }
    
    let d = this_char as u32 - 48;
    let collected = Some(new_collected.unwrap() * 10 + d);

    // println!("collected: {:?}  sign: {:?}, this_char: {:?}", collected, sign, this_char);

    atoi_recursive(new_s, sign, collected)
}


#[cfg(test)]
mod test_atoi {
    use super::*;

    #[test]
    fn test_recursive() {
        let s = "45rohi12";
        let d = atoi_recursive(s, None, None);
        dbg!(d);

    }
}