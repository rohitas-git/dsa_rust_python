/* ------------------------------ Sudoku Solver ----------------------------- */

// Problem Statement:
// Given a 9×9 incomplete sudoku, solve it such that it becomes valid sudoku.

// Valid sudoku has the following properties.
// 1. All the rows should be filled with numbers(1 – 9) exactly once.
// 2. All the columns should be filled with numbers(1 – 9) exactly once.
// 3. Each 3×3 submatrix should be filled with numbers(1 – 9) exactly once.

// Note: Character ‘.’ indicates empty cell.

/* ------------------------------- Complexity ------------------------------- */

// Time Complexity: O(9(n ^ 2)),
// in the worst case, for each cell in the n2 board, we have 9 possible numbers.

// Space Complexity: O(1), since we are refilling the given board itself,
// there is no extra space required, so constant space complexity.

/* ------------------------------------ x ----------------------------------- */

type Number = u32;
type Cell = Option<Number>;

type Sudoku = [[Cell; 9]; 9];
// type Submatrix = [[Cell; 3]; 3];

fn get_solutions(sudoku: &Sudoku) {
    let mut sudoku = sudoku.clone();
    solve_sudoku(&mut sudoku);
    dbg!(sudoku);
}

// Get a solution for sudoku
fn solve_sudoku(sudoku: &mut Sudoku) -> bool {
    for y in 0..9 {
        for x in 0..9 {
            if sudoku[y][x].is_none() {
                for num in 1..=9 {
                    if is_valid(sudoku, x, y, num) {
                        sudoku[y][x] = Some(num);

                        if solve_sudoku(sudoku) {
                            return true;
                        } else {
                            sudoku[y][x] = None;
                        }
                    }
                }
                return false;
            }
        }
    }
    true
}

fn is_valid(sudoku: &Sudoku, x: usize, y: usize, try_val: u32) -> bool {
    let val = try_val;

    // Number already present in row or column or submatrix
    for i in 0..9 {
        // Rows
        if let Some(cell_val) = sudoku[y][i] {
            if cell_val == val {
                return false;
            }
        }

        // Columns
        if let Some(cell_val) = sudoku[i][x] {
            if cell_val == val {
                return false;
            }
        }

        // Submatrix
        let sub_x = 3 * (x / 3) + i / 3;
        let sub_y = 3 * (y / 3) + i % 3;
        if let Some(cell_val) = sudoku[sub_y][sub_x] {
            if cell_val == val {
                return false;
            }
        }
    }
    true
}

fn print_sudoku(board: &Sudoku){
    for y in 0..9{
        for x in 0..9{
            if let Some(val) = board[y][x]{
                print!("| {}", val);
            }
            else{
                print!("| *", );
            }
        }
        print!("|");
        println!();
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_sudoku_solver() {
        let sudoku = [
            [
                Some(9),
                Some(5),
                Some(7),
                None,
                Some(1),
                Some(3),
                None,
                Some(8),
                Some(4),
            ],
            [
                Some(4),
                Some(8),
                Some(3),
                None,
                Some(5),
                Some(7),
                Some(1),
                None,
                Some(6),
            ],
            [
                None,
                Some(1),
                Some(2),
                None,
                Some(4),
                Some(9),
                Some(5),
                Some(3),
                Some(7),
            ],
            [
                Some(1),
                Some(7),
                None,
                Some(3),
                None,
                Some(4),
                Some(9),
                None,
                Some(2),
            ],
            [
                Some(5),
                None,
                Some(4),
                Some(9),
                Some(7),
                None,
                Some(3),
                Some(6),
                None,
            ],
            [
                Some(3),
                None,
                Some(9),
                Some(5),
                None,
                Some(8),
                Some(7),
                None,
                Some(1),
            ],
            [
                Some(8),
                Some(4),
                Some(5),
                Some(7),
                Some(9),
                None,
                Some(6),
                Some(1),
                Some(3),
            ],
            [
                None,
                Some(9),
                Some(1),
                None,
                Some(3),
                Some(6),
                None,
                Some(7),
                Some(5),
            ],
            [
                Some(7),
                None,
                Some(6),
                Some(1),
                Some(8),
                Some(5),
                Some(4),
                None,
                Some(9),
            ],
        ];

        print_sudoku(&sudoku);
    }

    
}
