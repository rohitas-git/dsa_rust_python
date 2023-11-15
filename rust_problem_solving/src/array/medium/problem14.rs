/* ---------------------------- Grid Unique Paths --------------------------- */
// Count paths from left-top to the right bottom of a matrix

// Problem Statement:
// Given a matrix m X n, count paths from left-top to the right bottom of a matrix
// with the constraints that from each cell
// you can either only move to the rightward direction or the downward direction.

// Input Format: m = 2, n= 3
// Ouput: 3

// Input Format: m = 2, n= 2
// Output: 2

/* ------------------------------------ Approach ----------------------------------- */

// Recusive approach:
// Recursive choose either downward or rightward till reach bottom-right
// Base case: if reached => 1; if out of bounds => 0


// Dynamic appraoch:
// The dynamic programming solution of this problem is a bit extension of the recursive solution.
// In recursive solutions, there are many overlapping subproblems.
// We memorize the subproblems and avoid the recomputations. 
// Thus, drastically reduce time complexity. 

// Combinatorics Solution
// Each time we are taking an exactly m+n-2 number of steps to reach the end.
// We need n-1 no. of rightward direction 
// and m-1 no. of downward direction to reach the endpoint.
// 
// Calculate the combinations ( ie: (m+n-2)C(n-1) or (m+n-2)C(m-1) ),
// we’ll get the total number of paths.

/* ------------------------------------ x ----------------------------------- */

pub struct Input {
    row: usize,
    column: usize,
}

#[derive(Debug,Clone)]
enum Computed{
    Yes,
    No
}

pub fn unique_paths(input: &Input) -> u32 {
    let r1 = recursive_count(0, 0, input.row, input.column);
    dbg!(r1)
}

fn recursive_count(i: usize, j: usize, row: usize, column: usize) -> u32 {
    
    if i == (row - 1) && j == (column - 1) {
        1
    }
    else if i >= row || j >= column {
        0
    } else {
        recursive_count(i + 1, j, row, column) + recursive_count(i, j + 1, row, column)
    }
}

pub fn unique_paths_v2(input: &Input) -> u32 {
    let mut memo = vec![vec![Option::<u32>::None;input.column];input.row];
    let r2 = dynamic_count(0, 0, input.row, input.column, &mut memo);
    dbg!(r2);
    dbg!(memo[0][0].unwrap())
}

fn dynamic_count(i: usize, j: usize, row: usize, column: usize, memo: &mut [Vec<Option<u32>>] ) -> u32{
    if i == (row - 1) && j == (column - 1) {
        1
    }
    else if i >= row || j >= column {
        0
    }
    else if memo[i][j].is_some(){
        memo[i][j].unwrap()
    } 
    else {
        memo[i][j] = Some(recursive_count(i + 1, j, row, column) + recursive_count(i, j + 1, row, column));
        memo[i][j].unwrap()
    }
}

fn unique_paths_v3(input: &Input) -> u32 {
    let total_steps = input.row + input.column - 2;
    let r_steps = input.row - 1;
    let _d_steps = input.column - 1;
    
    let mut count = 1;
    for i in 1..=r_steps{
        count = count * (total_steps - r_steps + i) / i;
    }

    count as u32
}

#[cfg(test)]
mod test_solution {
    use super::*;

    #[test]
    fn dynamic_soln() {
        let input: Input = Input{row:3, column:3};
        let r1: u32 = unique_paths_v2(&input);
        assert_eq!(r1, 6);

        let input: Input = Input{row:4, column:4};
        let r1: u32 = unique_paths_v2(&input);
        assert_eq!(r1, 20);

        let input: Input = Input{row:5, column:5};
        let r1: u32 = unique_paths_v2(&input);
        assert_eq!(r1, 70);
    }

    #[test]
    fn recursive_soln() {
        let input: Input = Input{row:4, column:4};
        let r1: u32 = unique_paths(&input);
        assert_eq!(r1, 20);

        let input: Input = Input{row:3, column:3};
        let r1: u32 = unique_paths(&input);
        assert_eq!(r1, 6);

        let input: Input = Input{row:5, column:5};
        let r1: u32 = unique_paths(&input);
        assert_eq!(r1, 70);
    }


}