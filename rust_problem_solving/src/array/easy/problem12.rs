/* ----- Find the number that appears once, and the other numbers twice ----- */
// Problem Statement: 
// Given a non-empty array of integers arr, 
// every element appears twice except for one. Find that single one.


// Note that we require that there be pairs of each number in array
// we know, a^a = 0 and 0^k = k
// Thus, we can apply solution approachs of problem10 (finding missing numbers in array)



fn xor_solution(arr:&[u32])->u32{
    let mut only_once = 0u32;

    for i in 0..arr.len(){
        only_once = only_once^arr[i];
    }

    only_once
}

#[cfg(test)]
mod test_find_lonely_number{
    use super::*;

    #[test]
    fn found_it() {
        let arr = [3,1,4,2,4,1,2];
        assert_eq!(xor_solution(&arr), 3);
    }
}