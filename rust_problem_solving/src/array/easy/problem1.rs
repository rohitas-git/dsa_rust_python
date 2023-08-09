// Problem Statement: Given an array, we have to find the largest element in the array.

use funty::Integral;

// Approaches:
// 1. We can maintain a max variable that will update whenever the current value is greater than the value in the max variable. 
// 2. We can sort the array in ascending order, hence the largest element will be at the last index of the array. 

pub fn optimal<T: Integral>(arr: &[T]) -> Option<T> {
    let mut lar = arr[0];
    let len = arr.len();
    
    if len == 0 {
        return None
    }
    
    if len==1 {
        return Some(arr[0]);
    }
    
    for i in 0..len {
        if arr[i] > lar {
            lar = arr[i]
        }
    }
    Some(lar)
}

#[cfg(test)]
mod test_solution_problem1 {
    use super::*;

    #[test]
    fn brute_u32() {
        let arr = [1, 2, 3, 4, 5];

        let lar = optimal(&arr).unwrap();
        
        assert_eq!(lar, 5);
        // println!("{}", lar);
    }
}

//  cargo test --package problem_solving --bin problem_solving -- array::easy::problem1::test_solution_problem1::brute_u32 --exact --nocapture
