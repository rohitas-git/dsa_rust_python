/* --------------------------- Stock Buy And Sell --------------------------- */

// Problem Statement: 
// You are given an array of prices where prices[i] is the price of a given stock on an ith day.
// You want to maximize your profit by choosing a single day to buy one stock 
// and choosing a different day in the future to sell that stock. 
// 
// Return the maximum profit you can achieve from this transaction. 
// If you cannot achieve any profit, return 0.

// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and 
// sell on day 5 (price = 6), profit = 6-1 = 5.

// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are 
// done and the max profit = 0.

/* -------------------------------- Approach -------------------------------- */
// Aim: To find a pair a[i], a[j] such that for i < j, a[i] < a[j]
//  It's quite similar to finding a inversion in an array
//  Due to problem statement,
//      - For maximum profit, a[j] - a[i] should have a[i] as minimum found till traversing to j
//      - among the many profits while traversing the array, choose the maximum one.
// 
// Brute: 
// We can simply use 2 loops and track every transaction and maintain a variable max_profit to contain the max value among all transactions.
//  
// Better:
//  written under inspiration from application of merge sort in counting inversions
//  apply divide and conquer technqiue here.
// The idea here: 
//  Find profit in left half, find profit in right half, find profit across left & right halves
//  Return maximum among them
// 
// Optimal:
// Traverse the array while maintaining 
// - a var to contain a minimum value/price found till now
// - a var to store max_profit till now
// At each element, calculate Profit at that price - minimumFound
// Store the max of curr_profit and max_profit into max_profit 
// 

/* ------------------------------------ x ----------------------------------- */

fn brute(arr:&[u32])->u32{
    let n = arr.len();
    let mut max_profit = 0;

    for i in 0..(n-1){
        for j in i..n{
            if arr[i] < arr[j]{
                let diff = arr[j]-arr[i];
                if max_profit < diff {
                    max_profit = diff;
                }
            }
        }
    }
    max_profit
}

mod better{
    // time - O(N * logN)
    // space - O(1)
    pub fn find_max_profit(arr:&[u32])-> u32{
        let n = arr.len();
        count_stock_profit(&arr, 0, n-1)
    }

    pub fn count_stock_profit(arr:&[u32],l:usize, r:usize) -> u32 {
        let mut res = 0;
        if l < r {
            // println!("l:{} r:{}",l,r);
            let mid: usize = (l+r)/2;
            let p_left = count_stock_profit(arr, l, mid);
            let p_right = count_stock_profit(arr, mid+1, r);
            let p_merge = count_merge(arr, l, mid, r);
            res = std::cmp::max(p_left, p_right);
            res = std::cmp::max(res, p_merge);
        }
        res
    }

    // Time: O(N)
    // Aux space: O(1)
    pub fn count_merge(arr:&[u32],l:usize,m:usize,r:usize)->u32{
        
        // Aux space - O(1)
        let (left,right) = arr.split_at(m);

        // println!("left {:?}, right {:?}", left, right);

        let mut res = 0;
        let (mut i , mut j) = (0, 0);

        while i < left.len() && j < right.len(){
            if left[i] < right[j]{
                if res < (right[j]-left[i]){
                    res = right[j]-left[i];
                }

                if j != (right.len()-1) {
                    j+=1;
                }
                else {
                    i+=1
                }
            }
            else {
                if i != (left.len()-1){
                    i+=1;
                }
                else {
                    j+=1;
                }
            }
        
        }
        res
    }


}

// Time O(N)
// Space O(1)
fn optimal(arr:&[u32]) -> u32{
    let mut max_profit = 0;
    let mut min_price = arr[0];

    for i in 1..(arr.len()){
        min_price = if arr[i] < min_price {arr[i]} else {min_price};
        max_profit = std::cmp::max(max_profit, (arr[i]-min_price));
    }
    max_profit
}

#[cfg(test)]
mod test_stock_trading {
    use super::*;

    #[test]
    fn test_optimal_works() {
        let arr = [7,1,5,3,6,4];
        assert_eq!(optimal(&arr), 5);

        let arr = [7,6,4,3,1];
        assert_eq!(optimal(&arr), 0);
    }

    #[test]
    fn test_better_works() {
        let arr = [7,1,5,3,6,4];
        assert_eq!(better::find_max_profit(&arr), 5);

        let arr = [7,6,4,3,1];
        assert_eq!(better::find_max_profit(&arr), 0);
    }

    #[test]
    fn test_brute_works() {
        let arr = [7,1,5,3,6,4];
        assert_eq!(brute(&arr), 5);

        let arr = [7,6,4,3,1];
        assert_eq!(brute(&arr), 0);
    }
}