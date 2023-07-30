struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        Solution::reverse_no_overflow(x).unwrap_or(0)
    }

    fn reverse_no_overflow(x: i32) -> Option<i32> {
        let mut sum: i32 = 0;
        let mut rem: i32 = x;
        while rem != 0 {
            const DEC_BASE: i32 = 10;
            let digit = rem % DEC_BASE;
            rem = rem / DEC_BASE;
            
            sum = sum.checked_mul(DEC_BASE)?;
            sum = sum.checked_add(digit)?;
        }
        
        Some(sum)
    }
}


