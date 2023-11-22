/* ------------------------------ Letter Phone ------------------------------ */

// Letter Combinations of a Phone number

fn possible_letters(arr: &[u32], steps: usize, letters:&mut String) {
    if steps == arr.len(){
        println!("{}", letters);
        return;
    }

    let possibles = get_letters(arr[steps]).unwrap();

    for letter in possibles{
        letters.push(letter);
        possible_letters(arr, steps + 1, letters);
        letters.pop();
    }
}

fn get_letters(num: u32) -> Option<Vec<char>>{
    let mut possible_letters: Vec<char> = Vec::new();

    match num {
        2 => possible_letters = vec!['a', 'b', 'c'],
        3 => possible_letters = vec!['d', 'e', 'f'],
        4 => possible_letters = vec!['g', 'h', 'i'],
        5 => possible_letters = vec!['j', 'k', 'l'],
        6 => possible_letters = vec!['m', 'n', 'o'],
        7 => possible_letters = vec!['p', 'q', 'r'],
        8 => possible_letters = vec!['t', 'u', 'v'],
        9 => possible_letters = vec!['w', 'x', 'y', 'z'],
        _ => return None
    }

    Some(possible_letters)
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_letters() {
        let arr = &[2,3];
        let steps = 0;
        let letters = &mut "".to_string();
        possible_letters(arr, steps, letters);
    }
}