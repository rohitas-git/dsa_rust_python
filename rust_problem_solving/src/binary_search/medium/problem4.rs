/* --------------------- Minimum days to make M bouquets -------------------- */
// Problem Statement:
// You are given ‘N’ roses and you are also given an array ‘arr’
// where ‘arr[i]’  denotes that the ‘ith’ rose will bloom on the ‘arr[i]th’ day.
//
// You can only pick already bloomed roses that are adjacent to make a bouquet.
// You are also told that you require exactly ‘k’ adjacent bloomed roses to make a single bouquet.
//
// Find the minimum number of days required to make at least ‘m’ bouquets
// each containing ‘k’ roses. Return -1 if it is not possible.

// Input Format: N = 8, arr[] = {7, 7, 7, 7, 13, 11, 12, 7}, m = 2, k = 3
// Result: 12

// Input Format: N = 5, arr[] = {1, 10, 3, 10, 2}, m = 3, k = 2
// Result: -1

// Input: bloomDay = [1,10,3,10,2], m = 3, k = 1
// Output: 3

/* -------------------------------- Approach -------------------------------- */
// Answer to Minimum days is required. It's answer space - [1 .. =max(bloomDay) ]

// For BS in this space,
//  find day which gives m bouquets with k flowers bloomed in each bouqets

/* -------------------------------- Analysis -------------------------------- */
// Time - O(N*logK), where K = max(array) - min(array) + 1 
// Space - O(N)

/* ------------------------------------ x ----------------------------------- */
#[derive(Debug, PartialEq)]
enum Status {
    Larger,
    Equal,
    Smaller,
}
use std::cmp::min;

use Status::*;

fn least_days_to_get_bouqets(bloom_days: &[u32], size: u32, amount: u32) -> Option<u32> {
    let flowers_req = size * amount;

    if flowers_req > bloom_days.len() as u32 {
        return None;
    }

    let mut start = bloom_days.iter().min().unwrap().clone();
    let mut end = bloom_days.iter().max().unwrap().clone();
    let mut least = u32::MAX;

    while start <= end {
        let this_day = start + (end - start) / 2;
        let bloomed = flowers_bloomed(bloom_days, this_day);
        let bloom_count = bloomed.iter().filter(|x| **x == true).count();

        if bloom_count < flowers_req as usize {
            start = this_day + 1;
        } else {
            match all_bouqets_done(&bloomed, size, amount) {
                Equal =>{ least = min(least, this_day); end= this_day-1;},
                Smaller => start = this_day + 1,
                Larger => end = this_day - 1,
            }
        }
    }
    Some(least)
}

// O(n) O(1)
fn all_bouqets_done(bloomed: &Vec<bool>, size: u32, bouquets_req: u32) -> Status {
    let mut adjacent = false;
    let mut count = 0;
    let mut adj_size = 0;
    for i in 0..bloomed.len() {
        if bloomed[i] == true && !adjacent {
            adjacent = true;
            adj_size = 1;
        } else if bloomed[i] == true && adjacent {
            adj_size += 1;
        } else if bloomed[i] == false && adjacent {
            adjacent = false;
            adj_size = 0;
        }
        if adj_size == size as usize {
            count += 1;
            adj_size =0;
        }
        println!("bloomed:{}  adjacent:{}", bloomed[i], adjacent);
        println!("adj_size {}  count {}", adj_size,count);
    }
    if count == bouquets_req {
        Equal
    } else if count > bouquets_req {
        Larger
    } else {
        Smaller
    }
}

// O(n) O(n)
fn flowers_bloomed(bloom_days: &[u32], day: u32) -> Vec<bool> {
    let mut bloomed = Vec::new();
    for i in 0..(bloom_days.len()) {
        if bloom_days[i] <= day {
            bloomed.push(true);
        } else {
            bloomed.push(false);
        }
    }
    println!("DAY {}", day);
    bloomed
}

fn views_flowers_state(bloom_days: &[u32], day: u32) {
    let bloom_days = &[1, 10, 3, 10, 2];

    let list = flowers_bloomed(bloom_days, day);
    print!("Day:{}  ", day);
    for r in list {
        print!("|{}", if r { "x" } else { "_" });
    }
    print!("|");
    println!();
}

#[cfg(test)]
mod test_min_days_to_get_bouqets {
    use super::*;

    #[test]
    fn flowers_bloomed_ok() {
        let bloom_days = &[1, 10, 3, 10, 2];
        // views_flowers_state(bloom_days, 10);

        let res = flowers_bloomed(bloom_days, 3);
        let bloom_count = res.iter().filter(|x| **x == true).count();
        println!("{}", bloom_count);
        assert_eq!(res, vec![true, false, true, false, true]);

        let res = flowers_bloomed(bloom_days, 9);
        let bloom_count = res.iter().filter(|x| **x == true).count();
        println!("{}", bloom_count);
        assert_eq!(res, vec![true, false, true, false, true]);
    }

    #[test]
    fn bouqets_made_ok() {
        let bloom_days = &[1, 10, 3, 10, 2];
        let bloomed = flowers_bloomed(bloom_days, 3);
        views_flowers_state(bloom_days, 3);

        let res = all_bouqets_done(&bloomed, 1, 3);
        assert_eq!(res, Equal);
        let res = all_bouqets_done(&bloomed, 1, 5);
        assert_eq!(res, Smaller);
        let res = all_bouqets_done(&bloomed, 1, 1);
        assert_eq!(res, Larger);
    }

    #[test]
    fn least_days_ok() {
        let bloom_days = &[1, 10, 3, 10, 2];
        
        let res = least_days_to_get_bouqets(bloom_days, 1, 3);
        assert_eq!(res,Some(3));

        let res = least_days_to_get_bouqets(bloom_days, 5, 3);
        assert_eq!(res,None);

        let bloom_days = &[7, 7, 7, 7, 13, 11, 12, 7];
        let res = least_days_to_get_bouqets(bloom_days, 2, 3);
        assert_eq!(res, Some(12));
    }
}
