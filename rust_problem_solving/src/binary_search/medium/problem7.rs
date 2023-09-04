/* ----------------------- Kth Missing Positive Number ---------------------- */
// Problem Statement: 
// You are given a strictly increasing array ‘vec’ and a positive integer ‘k’. 
// Find the ‘kth’ positive integer missing from ‘vec’.

// Input Format: vec[]={4,7,9,10}, k = 4
// Result: 5
// Explanation: The missing numbers are 1, 2, 3, 5, 6, 8, 11, 12, ……, and so on. Since 'k' is 4, the fourth missing element is 5.

/* -------------------------------- Approach -------------------------------- */

// Brute: 
// The main idea is to shift k by 1 step if the current element is smaller or equal to k. 
// And whenever we get a number > k, we can conclude that k is the missing number.
// 
// Use Loop to traverse the array and inside it:
//      If vec[i] <= k: we will simply increase the value of k by 1.
//      Otherwise, we will break out of the loop.
// Finally, we will return the value of k.

// Optimal:
// We cannot apply binary search on the answer space here as we cannot assure which missing number has the possibility of being the kth missing number. 
// That is why, we will do something different here. 
// We will try to find the closest neighbors (i.e. Present in the array) for the kth missing number by counting the number of missing numbers for each element in the given array.
// 
// For a given value of k as 5, we can determine that the answer falls within the range of 7 to 11. 
// Since there are only 3 missing numbers up to index 3, the 5th missing number cannot be before vec[3], which is 7. 
// Therefore, it must be located somewhere to the right of 7. 
// Our actual answer i.e. 9 also supports this theory. 
// So, by following this process we can find the closest neighbors (i.e. Present in the array) for the kth missing number. 
// In our example, the closest neighbors of the 5th missing number are 7 and 11.
// 
// Number of missing numbers up to index i = vec[i] - (i+1).
// The given array, vec, is currently containing the number vec[i] whereas it should contain (i+1) if no numbers were missing. 
// The difference between the current and the ideal element will give the result.
// 
// After completing the binary search on the indices, the pointer high will point to the closest neighbor(present in the array) that is smaller than the kth missing number. 

/* ------------------------------------ x ----------------------------------- */

fn brute(arr:&[u32], k: u32) -> u32{
    let mut k = k;
    for i in 0..(arr.len()){
        if arr[i] <= k{
            k+=1;
        }else {
            break;
        }
    }
    k
}

fn optimal(arr:&[u32],k: u32)-> u32{
    let mut low = 0;
    let mut high = arr.len()-1;

    while low <= high {
        let mid = low + (high-low)/2;
        let missing = arr[mid] - (mid+1) as u32;

        if missing < k {
            low = mid +1;
        }
        else {
            high = mid-1;
        }
    }
    k + high as u32 + 1
}

/* ---------------------------------- Test ---------------------------------- */
#[cfg(test)]
mod test_finding_missing_kth_number {
    use super::*;

    #[test]
    fn test_brute() {
        let arr = [4,7,9,10];
        let res = brute(&arr, 4);
        assert_eq!(res,5);
    }

    #[test]
    fn test_optimal() {
        let arr = [4,7,9,10];
        let res = optimal(&arr, 4);
        assert_eq!(res,5);
    }
}