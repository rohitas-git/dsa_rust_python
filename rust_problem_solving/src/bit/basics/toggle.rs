
/* ------------------------------ Toggle a bit ------------------------------ */

// Since XOR of unset and set bit results in a set bit 
// and XOR of a set and set bit results in an unset bit. 
// 
// Hence performing bitwise XOR of any bit with a set bit results in toggle of that bit, 
// i.e

// Any bit <bitwise XOR> Set bit = Toggle
// which means,
// 0 ^ 1 = 1
// 1 ^ 1 = 0.

/* ------------------------------------ x ----------------------------------- */


use crate::bit::print_binary::*;

fn toggle(number: u32, kth: u32){
    let mask = 1 << kth;
    let toggled = number ^ mask;

    print!("Before toggling: {} - ", number);
    print_binary_rep(number);

    print!("After toggling: {} - ", toggled);
    print_binary_rep(toggled);
}


#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_toggle() {
        toggle(5, 0);
        toggle(7, 1);
    }
}