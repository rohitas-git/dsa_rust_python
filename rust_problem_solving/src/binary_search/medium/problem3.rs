/* --------------------------- Koko Eating Bananas -------------------------- */
// Problem Statement:
// A monkey is given ‘n’ piles of bananas, whereas the ‘ith’ pile has ‘a[i]’ bananas.
// An integer ‘h’ is also given, which denotes the time (in hours) for all the bananas to be eaten.

// Each hour, the monkey chooses a non-empty pile of bananas and eats ‘k’ bananas.
// If the pile contains less than ‘k’ bananas, then the monkey consumes all the bananas
// and won’t eat any more bananas in that hour.

// Find the minimum number of bananas ‘k’ to eat per hour
// so that the monkey can eat all the bananas within ‘h’ hours.

// Input Format: N = 4, a[] = {7, 15, 6, 3}, h = 8
// Result: 5

/* -------------------------------- Approach -------------------------------- */

// Observation:  We need to minimum eating speed such n-piles are finished in H time
// Koko can at slowest speed eat 1 banana per hour
// Koko 's maximum speed can only be equal to maximum element in piles
// 
// So, our answer space is [1, maxKokoEatingSpeed] 
// and we can use BS to find the right speed in this domain

/* -------------------------------- Analysis -------------------------------- */
// Time Complexity: O(N * log(max(a[]))), 
// where max(a[]) is the maximum element in the array and N = size of the array.

// Ceiling_Div - O(1), O(1)
// Hours Eating - O(N), O(1) where N- no.of piles 
// Find Min eating speed - O(logM * N), where M is maximum no. of bananas of a single pile in piles
/* ------------------------------------ x ----------------------------------- */

use std::cmp::min;

fn minimum_rate_of_eating(piles: &[u32], time: u32) -> u32 {
    let mut lower_bound = 1;
    let mut upper_bound = piles.iter().max().unwrap().clone();
    
    while lower_bound <= upper_bound  {
        let median = lower_bound + (upper_bound - lower_bound) / 2;

        let mut time_spent = hours_eating(piles, median);

        if time_spent <= time {
            upper_bound = median -1;        
        } 
        else {
            lower_bound = median +1;
        }
    }
    lower_bound
}

fn hours_eating(piles: &[u32], speed: u32) -> u32 {
    let mut hours = 0;

    for quantity in piles {
        hours += ceil_div(quantity.clone(), speed);
    }
    hours
}

// ceilVal = (a / b) + ((a % b) != 0)
// ceilVal = (a+b-1) / b
fn ceil_div(quanity: u32, speed: u32) -> u32 {
    (quanity / speed + (quanity % speed != 0) as u32)
}


#[cfg(test)]
mod test_koko_eating_bananas {
    use super::*;

    #[test]
    fn ceiling_div_ok() {
        let r = ceil_div(4, 3);
        assert_eq!(r, 2);

        let r = ceil_div(6, 3);
        assert_eq!(r, 2);

        let r = ceil_div(11, 3);
        assert_eq!(r, 4);
    }

    #[test]
    fn eating_time_given_speed_ok() {
        let piles = &[25, 12, 8, 14, 19];
        let res = hours_eating(piles, 25);
        assert_eq!(res,5);

        let piles = &[7, 15, 6, 3];
        let res = hours_eating(piles, 7);
        assert_eq!(res,6);
    }

    #[test]
    fn get_right_min_eating_speed() {
        let piles = &[25, 12, 8, 14, 19];
        let time = 5;
        let res = minimum_rate_of_eating(piles, time);
        assert_eq!(res,25);


        let piles = &[30,11,23,4,20];
        let time = 6;
        let res = minimum_rate_of_eating(piles, time);
        assert_eq!(res,23);
    }
}