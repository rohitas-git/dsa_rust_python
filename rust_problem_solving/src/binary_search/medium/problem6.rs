/* ----------------- Capacity to Ship Packages within D Days ---------------- */
// Problem Statement: 
// You are the owner of a Shipment company. 
// You use conveyor belts to ship packages from one port to another. 
// The packages must be shipped within ‘d’ days.
// The weights of the packages are given in an array ‘of weights’. 
// The packages are loaded on the conveyor belts every day in the same order as they appear in the array. 
// The loaded weights must not exceed the maximum weight capacity of the ship.
// Find out the least-weight capacity so that you can ship all the packages within ‘d’ days.

// Input Format: N = 5, weights[] = {5,4,5,2,3,4,5,6}, d = 5
// Result: 9

// Input Format: N = 10, weights[] = {1,2,3,4,5,6,7,8,9,10}, d = 1
// Result: 55

/* ------------------------------ Observations ------------------------------ */
// Minimum ship capacity: In order to ship all the weights, the minimum ship capacity should be 
// equal to the maximum of the weights array i.e. nax(weights[]).

// Maximum ship capcity: If the ship capacity is equal to the sum of all the weights, we can ship all goods within a single day. 
// Any capacity greater than this will yield the same result.

/* ------------------------------------ x ----------------------------------- */

use std::cmp::min;

fn least_weight_capacity(weights: &[u32], days: u32){
    let min_cap = weights.iter().max().unwrap().clone();
    let max_cap: u32 = weights.iter().sum();

    let mut low = min_cap;
    let mut high = max_cap;
    let mut res = u32::MAX;

    while low <= high {
        let this_cap = low + (high-low)/2;
        
        let time = find_days(weights, this_cap);

        if time <= days{
            res = min(time, res)
        }
    }

}

fn find_days(weights: &[u32], capacity:u32) -> u32{
    let mut load = 0;
    let mut days = 1;

    for weight in weights{
        
        if load + weight > capacity{
            days +=1;
            load = weight.clone();
        }else{
            load += weight;
        }
    }
    days
}

#[cfg(test)]
mod test_find_least_weight_capacity {
    use super::*;

    #[test]
    fn find_days_ok() {
        let weights = [5,4,5,2,3,4,5,6];
        let res = find_days(&weights, 9);
        assert_eq!(res,5);

        let weights = [1,2,3,4,5,6,7,8,9,10];
        let res = find_days(&weights, 15);
        assert_eq!(res,5);
    }

    #[test]
    fn test_solution() {
        let weights = [5,4,5,2,3,4,5,6];
        let res = least_weight_capacity(&weights, 5);
    }
}
