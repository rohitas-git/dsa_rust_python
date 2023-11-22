
/* ----------------------------- Clearing a bit ----------------------------- */

// bitwise AND of any bit with a reset bit results in a reset bit, i.e.

// Any bit <bitwise AND> Reset bit = Reset bit
// which means,
// 0 & 0 = 0
// 1 & 0 = 0

/* ------------------------------------ x ----------------------------------- */

use crate::bit::print_binary::*;

fn clear(number: u32, kth: u32){
    let mask = !(1 << kth);
    let cleared = number & mask;

    print!("Before clearing: {} - ", number);
    print_binary_rep(number);

    print!("After clearing: {} - ", cleared);
    print_binary_rep(cleared);
}


#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_setting() {
        clear(5, 0);
        clear(7, 1);
    }
}