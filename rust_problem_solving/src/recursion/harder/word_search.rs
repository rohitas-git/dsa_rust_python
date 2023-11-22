/* ------------------------------- Word Search ------------------------------ */

// Given an m x n grid of characters board and a string word,
// return true if the word exists in the grid.
//
// The word can be constructed from letters of sequentially adjacent cells,
// where adjacent cells are horizontally or vertically neighboring.
// The same letter cell may not be used more than once.

/* -------------------------------- Approach -------------------------------- */
// Approach:  We are going to solve this by using backtracking,
// in this approach first we will linearly search the entire matrix to find
// the first letters matching our given string.
// If we found those letters
// then we can start backtracking in all four directions to find the rest of the letters of the given string.

// Steps:
// Step 1: Find the first character of the given string.
// Step 2: Start Backtracking in all four directions until we find all the letters of sequentially adjacent cells.
// Step 3: At the end, If we found our result then return true else return false.

// Edge cases: Now think about what will be our stopping condition,
// we can stop or return false if we reach the end of the boundaries of the matrix
// or the letter at which we are making recursive calls is not the required letter.
//
// We will also return if we found all the letters of the given word
// i.e. we found the number of letters equal to the length of the given word.

/* ------------------------------------ x ----------------------------------- */
type Board = Vec<Vec<char>>;

fn exist(board: Board, word: &str) -> bool {
    let m = board.len();
    let n = board[0].len();
    let mut board: Vec<Vec<char>> = board
        .iter()
        .map(|line| {
            line.iter()
                .map(|letter| letter.to_ascii_uppercase())
                .collect()
        })
        .collect();

    let index = 0_usize;
    let word: Vec<char> = word.to_uppercase().chars().collect();

    // search for first character of word in board
    for i in 0..m {
        for j in 0..n {
            if board[i][j] == word[index]
                && search_next(&mut board, &word, Some(i), Some(j), index, m, n)
            {
                return true;
            }
        }
    }
    false
}

fn search_next(
    board: &mut Board,
    word: &Vec<char>,
    row: Option<usize>,
    col: Option<usize>,
    index: usize,
    m: usize,
    n: usize,
) -> bool {
    // searched all characters in word
    if index == word.len() {
        return true;
    }

    // boundary check
    if row.is_none() || col.is_none() {
        return false;
    }
    let row = row.unwrap();
    let col = col.unwrap();

    // check if within boundaries and curr position char is already visited
    if row >= m || col >= n || board[row][col] != word[index] || board[row][col] == '!' {
        return false;
    }

    println!("Here: ({}, {}) = {}", row, col, board[row][col]);

    // to prevent reuse of the same char
    let letter = board[row][col];
    board[row][col] = '!';

    // backtrack in 4 directions
    let top = search_next(board, word, row.checked_sub(1), Some(col), index + 1, m, n);
    let left = search_next(board, word, Some(row), col.checked_sub(1), index + 1, m, n);
    let right = search_next(board, word, Some(row), col.checked_add(1), index + 1, m, n);
    let bottom = search_next(board, word, row.checked_add(1), Some(col), index + 1, m, n);

    board[row][col] = letter; // undo change

    top || bottom || left || right
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_finding() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "abcced";
        exist(board, word);
    }
}
