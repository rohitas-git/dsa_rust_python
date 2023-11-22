
/* ----------------------------- Finding a bit: ----------------------------- */
// Find the Kth bit in binary representation of N.

fn get_kth_bit(num: u32, k: u32) -> u32{
    dbg!((num >> k) & 1)
}

fn get_kth_bit_v2(num: u32, k: u32) -> u32{
    dbg!(((num & (1 << k)) != 0) as u32 )
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_getting() {
        get_kth_bit_v2(5, 2);
        get_kth_bit_v2(5, 1);
        get_kth_bit_v2(5, 0);
    }
}