// Subsequence
// A contiguous / non-contiguous sequence which follows the order of the array

// Subarray is a subsequence but subsequence may not be subarray

/* ------------------------------ Problem types ----------------------------- */

// Print all subsequence
//  Keep a DS to store/track the all possible possibilities
//  - go toward a possibility
//  - go towards another possibility
//  - similarly cover all possibilities

// Print all subsequence whose sum == target
//  - parametrize 

// Print any subsequence whose sum == target
//  - return T/F and return early if true 

// Count all subsequence whose sum == target
//  - return 1/0 and add all returns to get count

/* ---------------------------------- Solns ---------------------------------- */

// Time - O(2^n)
// Number of subsequences - 2^n  [n = arr.len()]
// Space - O(n)     [because of call stack]
fn print_subsequence(arr: &[u32], index: usize, subequence: &mut Vec<u32>) {
    if index >= arr.len() {
        println!("{:?}", subequence);
        return;
    }

    println!("Left index: {}", index);
    subequence.push(arr[index]);
    print_subsequence(arr, index + 1, subequence);

    subequence.pop();
    // println!("Left index: {}, sub: {:?}", index, subequence);
    println!("Right index: {}", index);
    print_subsequence(arr, index + 1, subequence);
}

// fn action(given, time_id, history){
//   if limit crossed{
//      do_something;
//      quit;
//   }
//
//   curr_history = history
//
//   GO FORWARD
//   future_history = history.create_future(time)
//   action(given, time+1, future_history)
//
//   GO BACKWARD
//   past_history = history.goto_curr(time)
//   action(given, time+1, past_history)
// }

fn print_all_whose_sum(
    given: &[u32],
    steps: usize,
    hist: &mut Vec<u32>,
    condn_var: u32,
    target: u32,
) {
    if steps >= given.len() {
        if condn_var == target {
            println!("{:?}", hist);
        }
        return;
    }

    hist.push(given[steps]);
    let condn_var = condn_var + given[steps];
    print_all_whose_sum(given, steps + 1, hist, condn_var, target);

    hist.pop();
    let condn_var = condn_var - given[steps];
    print_all_whose_sum(given, steps + 1, hist, condn_var, target);
}

fn print_any_whose_sum(
    given: &[u32],
    steps: usize,
    hist: &mut Vec<u32>,
    sum: u32,
    target: u32,
) -> bool {
    if steps >= given.len() {
        if sum == target {
            println!("{:?}", hist);
            return true;
        } else {
            return false;
        }
    }

    hist.push(given[steps]);
    if print_any_whose_sum(given, steps + 1, hist, sum + given[steps], target) {
        return true;
    }

    hist.pop();
    if print_any_whose_sum(given, steps + 1, hist, sum, target) {
        return true;
    }

    false
}

fn count_all_whose_sum(
    given: &[u32],
    steps: usize,
    sum: u32,
    target: u32,
) -> u32 {
    if steps >= given.len() {
        if sum == target {
            return 1;
        } else {
            return 0;
        }
    }

    let left = count_all_whose_sum(given, steps + 1, sum + given[steps], target);

    let right = count_all_whose_sum(given, steps + 1, sum, target);

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_sum() {
        let arr = [1, 1, 2];
        let target = 2;
        let mut hist = vec![];

        print_all_whose_sum(&arr, 0, &mut hist, 0, target);
        dbg!(count_all_whose_sum(&arr, 0, 0, target));
    }

    #[test]
    fn test_print_all() {
        let arr = [3, 1, 2];
        let mut sub = vec![];
        print_subsequence(&arr, 0, &mut sub);
    }
}
