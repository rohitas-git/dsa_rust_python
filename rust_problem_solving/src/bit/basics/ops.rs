
/* --------------------------- Bitwise Operations: -------------------------- */

#[cfg(test)]
mod test_operations {
    use crate::bit::print_binary::*;

    #[test]
    fn see_ops(){
        // 5 = 00000101
        let a = 5u8;
        // 9 = 00001001
        let b = 9u8;

        dbg!(a & b); // 00000001
        dbg!(a | b); // 00001101
        dbg!(a ^ b); // 00001100
        dbg!(!a, !b); // ~a = 11111010
        dbg!(b << 1); // 00010010
        dbg!(b >> 1); // 00000100
        dbg!(-5 & 5);
        dbg!(-5 | 5);
        dbg!(-5 ^ 5);
        dbg!(-5  << 2);
        println!("{:b}", -1i8);
        dbg!(1^(-1i8));
        dbg!(a+b == (a|b) + (a&b));
        dbg!( 6&1, 7&1); // The & operator can be used to quickly check if a number is odd or even.

    }

    #[test]
    fn test_all() {
        let x = 4;

        // println!("x:   {:b}", x);
        print_binary_rep_v2(x);
        println!("x^0s: {:b}",x^0);
        println!("x^1s = ~x: {:b}",x^(u32::MAX));
        println!("x^x: {:b}",x^x);
        
        println!("x & 0s: {:b}",x & 0);
        println!("x & 1s: {:b}",x & (u32::MAX) ); 
        println!("x & x: {:b}",x & x );  

        println!("x | 0s: {:b}",x | 0);
        println!("x | 1s: {:b}",x | (u32::MAX));
        println!("x | x: {:b}",x | x);
    }
}