// Problem Statement: Given an array, find the second smallest and second largest element in the array.
// Print ‘-1’ in the event that either of them doesn’t exist.

// Approaches:
// 1. Brute: Sort the array. 1 and N-2 item of array is the answer [T:O(NlogN) for sorting, S:O(1)]
// 2. Better: Find the smallest and largest element in the array in a single traversal. Then find the second largest and smallest. [t:O(n), s:O(1)]
// 3. Optimal:

use funty::Integral;

pub fn get_second_largest_smallest<T: Integral>(arr: &[T]) -> Option<(T, T)> {
    let a = get_second_largest(arr).expect("Panic getting second largest");
    let b = get_second_smallest(arr).expect("Panic getting second smallest");

    Some((a,b))
}

fn get_second_largest<T: Integral>(arr: &[T]) -> Option<T> {
    if arr.len() == 1 {
        return None;
    }

    let mut lar = &arr[0];
    let mut slar = Integral::MIN;

    for x in &arr[1..] {
        if x > lar {
            slar = lar.clone();
            lar = x;
        } else if x != lar {
            if x > &slar || slar == Integral::MIN {
                slar = x.clone();
            }
        }
    }
    Some(slar)
}

fn get_second_smallest<T: Integral>(arr: &[T]) -> Option<T> {
    if arr.len() == 1 {
        return None;
    }

    let mut small = &arr[0];
    let mut ssma = Integral::MAX;

    for x in &arr[1..] {
        if x < small {
            ssma = small.clone();
            small = x;
        } else if x != small {
            if x < &ssma || ssma == Integral::MAX {
                ssma = x.clone();
            }
        }
    }
    Some(ssma)
}

#[cfg(test)]
mod test_problem2 {
    use super::*;

    #[test]
    fn second_largest_smallest_ok() {
        let arr = &[-12,0,40,99];
        let a = get_second_largest_smallest(arr).unwrap();

        assert_eq!(a,(40,0));
    }


    #[test]
    fn second_largest_ok() {
        let arr = &[-12,0,40,99];
        let a = get_second_largest(arr).unwrap();

        assert_eq!(a,40);
    }


    #[test]
    fn second_smallest_ok() {
        let arr = &[12u32,0,40,99];
        let a = get_second_smallest(arr).unwrap();

        assert_eq!(a,12);
    }

    #[test]
    fn second_smallest_negatives_ok() {
        let arr = &[-12,0,40,99];
        let a = get_second_smallest(arr).unwrap();

        assert_eq!(a,0);
    }

    

}