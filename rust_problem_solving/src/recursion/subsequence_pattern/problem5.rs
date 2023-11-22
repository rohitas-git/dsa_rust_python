/* ----------------------------- Combination Sum ---------------------------- */

use std::collections::HashSet;

#[derive(Debug)]
struct Input<'a> {
    arr: &'a [u32],
    target: u32,
    combination: Vec<u32>,
    set: HashSet<Vec<u32>>,
}

impl<'a> Input<'a> {
    fn new(arr: &'a [u32], target: u32) -> Self {
        Input {
            arr,
            target,
            combination: Vec::new(),
            set: HashSet::new(),
        }
    }

    /// print distinct subsequences whose sum = target
    fn combination_sum(&mut self, steps: usize) {
        if steps == self.arr.len() {
            if self.combination.iter().sum::<u32>() == self.target {
                self.combination.sort();
                if self.set.insert(self.combination.clone()) {
                    {
                        println!("{:?}", self.combination);
                    }
                }
            }
            return;
        }

        self.combination.push(self.arr[steps]);
        self.combination_sum(steps + 1);

        self.combination.pop();
        self.combination_sum(steps + 1);
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_sum() {
        let mut input = Input::new(&[1, 2, 3, 1], 5);
        // input.combination_sum(0);

        let mut input = Input::new(&[1,2,3,4,5,6,7,8,9], 5);
        input.combination_sum(0);
    }
}
