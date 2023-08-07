pub mod modulo {


    // The modular multiplicative inverse is an integer X such that:
    // A X ≅ 1 (mod M)   
    // 
    // The value of X should be in the range {1, 2, … M-1},
    // The multiplicative inverse of “A modulo M” exists if and only if A and M are relatively prime (i.e. if gcd(A, M) = 1)
    pub fn multiplicative_inverse(a: u32, m: u32) -> Result<u32, String> {
        for x in 0..m{
            if ( ((a % m) * (x % m)) % m == 1){
                return Ok(x as u32);
            }
        }
        return Err("A and M are not relatively prime".into());
    }
    // Time Complexity: O(M)
    // Auxiliary Space: O(1)

}

#[cfg(test)]
mod test_multiplicative_inverse{
    use super::modulo::multiplicative_inverse;

    #[test]
    fn check_correct(){
        assert_eq!( multiplicative_inverse(4, 11), Ok(3u32));
    }
}
