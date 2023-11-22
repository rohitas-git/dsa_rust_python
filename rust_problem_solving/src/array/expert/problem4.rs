/* ------------------ Maximum Product Subarray in an Array ------------------ */

// Problem Statement:
// Given an array that contains both negative and positive integers,
// find the maximum product subarray.

// Input: Nums = [1,2,3,4,5,0]
// Output: 120

// Input: Nums = [1,2,-3,0,-4,-5]
// Output:  20

/* -------------------------------- Approach -------------------------------- */

// Brute:
// Find all possible subarrays of the given array.
// Find the product of each subarray. Return the maximum of all them.

// Better:
// Reduce the loops from 3 to 2 by memorization of prev calculation

// Optimal 1: Apply the logic discussed in the 4th observation to any given subarray
//
// Observations:
// 1. If the given array only contains positive numbers:
//      - maximum product subarray will be the entire array itself.
// 2. If the given array contains an even number of negative numbers:
//      - maximum product subarray will be the entire array itself.
// 3. If the given array also contains an odd number of negative numbers:
//      - Removal of 1 negative number out of the odd number of negative numbers
//        will leave us with an even number of negatives.
//      - Now we need to decide which 1 negative number to remove
//        such that the remaining subarray yields the maximum product.
//      - Upon observation, we notice that each chosen negative number divides the array into two parts.
//        The answer will either be the prefix or the suffix of that negative number.
//      - The maximum product obtained from these prefix and suffix subarrays will be our final answer.
// 4. If the array contains 0’s as well:
//      - never consider 0’s in our answer
//      - we will divide the given array based on the location of the 0’s
//      - and apply the logic of case 3 for each subarray.

// Optimal 2: Motivated by Kandane’s algorithm
// This algorithm maintains prod1 and prod2 to keep track 
//  of the maximum and minimum products ending at the current index, respectively.
// By considering both the positive and negative integers in the array, 
//  it effectively handles cases where the product can flip from a smaller negative number 
//  to a larger positive number or vice versa, 
//  ensuring the calculation of the maximum product subarray.
// 
// The intuition of the algorithm is 
// not to consider the subarray as a part of the answer if its goes against our intention.
// this type of subarray cannot be a part of the subarray with maximum sum.


/* ------------------------------- Complexity ------------------------------- */

// Brute: time O(N^3), space O(1)

// Better: time O(N^2), space O(1)

// Optimal 1: time O(N), space O(1)

// Optimal 2: time O(N), space O(1)

/* -------------------------------- Solution -------------------------------- */

fn optimal_soln_2(arr: &[i32]) -> i32 {
    let mut prod1 = arr[0];
    let mut prod2 = arr[0];
    let mut res = arr[0];

    for item in arr.iter().skip(1) {
        let temp = std::cmp::max(*item, (prod1 * item).max(item * prod2));
        prod2 = std::cmp::min(*item, (prod1 * item).min(item * prod2));
        prod1 = temp;

        res = res.max(prod1);
    }
    res
}

fn optimal_soln_1(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut ans = i32::MIN;

    // stores the product of the prefix subarray
    let mut pre_prod = 1;
    // stores the product of the suffix subarray
    let mut suff_prod = 1;

    for i in 0..n {
        // zero is a divider in getting product of subarray
        if pre_prod == 0 {
            // consider the current element as a part of the new prefix subarray
            pre_prod = 1;
        }
        if suff_prod == 0 {
            // consider the current element as a part of the new suffix subarray
            suff_prod = 1;
        }

        // multiply the elements from the front and back
        pre_prod *= arr[i];
        suff_prod *= arr[n - 1 - i];

        // consider the maximum among the previous answer, ‘pre’ and ‘suff’
        ans = ans.max(pre_prod.max(suff_prod));
    }
    ans
}

fn better_soln(arr: &[i32]) -> i32 {
    let mut result = arr[0];
    let n = arr.len();

    for start in 0..n - 1 {
        // product of subarray arr[start..k] where k in start..n
        let mut subarr_prod = arr[start];

        for next_elem in &arr[start + 1..n] {
            // max of result and subarrProd of arr[start..last]
            result = result.max(subarr_prod);

            // subarrProd of arr[start..=last]
            subarr_prod *= next_elem;
        }
        // max of result and subarrProd of arr[start..n]
        result = result.max(subarr_prod);
    }
    result
}

fn brute_soln(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut prod_max = i32::MIN;

    for start in 0..n - 1 {
        for end in start + 1..n {
            let subarr_prod: i32 = arr[start..=end].iter().product();
            prod_max = subarr_prod.max(prod_max);
        }
    }
    prod_max
}

/* ---------------------------------- Tests --------------------------------- */

#[cfg(test)]
mod test_problem4 {
    use super::*;

    #[test]
    fn test_optimal_2() {
        let arr = [1, 2, 3, 4, 5, 0];
        let res = 120;
        assert_eq!(res, optimal_soln_2(&arr));

        let arr = [-2, 3, 4, -1, 0, -2, 3, 1, 4, 0, 4, 6, -1, 4];
        let res = 24;
        assert_eq!(res, optimal_soln_2(&arr));
    }

    #[test]
    fn test_optimal_1() {
        let arr = [1, 2, 3, 4, 5, 0];
        let res = 120;
        assert_eq!(res, optimal_soln_1(&arr));

        let arr = [-2, 3, 4, -1, 0, -2, 3, 1, 4, 0, 4, 6, -1, 4];
        let res = 24;
        assert_eq!(res, optimal_soln_1(&arr));
    }

    #[test]
    fn test_better() {
        let arr = [1, 2, 3, 4, 5, 0];
        let res = 120;
        assert_eq!(res, better_soln(&arr));

        let arr = [1, 2, -3, 0, -4, -5];
        let res = 20;
        assert_eq!(res, better_soln(&arr));
    }

    #[test]
    fn test_brute() {
        let arr = [1, 2, 3, 4, 5, 0];
        let res = 120;
        assert_eq!(res, brute_soln(&arr));

        let arr = [1, 2, -3, 0, -4, -5];
        let res = 20;
        assert_eq!(res, brute_soln(&arr));
    }
}
