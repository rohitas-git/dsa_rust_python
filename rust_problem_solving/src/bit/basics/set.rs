
/* -------------------------------- Setting a bit ------------------------------- */
// Setting a bit means that if K-th bit is 0, then set it to 1 
// and if it is 1 then leave it unchanged.

// Any bit <bitwise OR> Set bit = Set bit
// which means,
// 0 | 1 = 1
// 1 | 1 = 1

// (x >> n) & 1 gives you the nth bit,
// x & !(1 << n) clears the nth bit,
// and x | (1 << n) sets the nth bit.

/* ------------------------------------ x ----------------------------------- */
use crate::bit::print_binary::*;

fn set(number: u32, kth: u32){
    let mask = 1 << kth;
    let setted = number | mask ;

    print!("Before setting: {} - ", number);
    print_binary_rep(number);

    print!("After setting: {} - ", setted);
    print_binary_rep(setted);
}


#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_setting() {
        set(5, 1);
        set(4, 0);
    }
}

