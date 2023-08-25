/* ----------------------------- Set Matrix Zero ---------------------------- */
// Problem Statement:
// Given a matrix if an element in the matrix is 0 then you will have to
// set its entire column and row to 0 and then return the matrix.
// Note: Matrix is a square matrix

// Input: matrix=[[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]

// Input: matrix=[[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output:[[0,0,0,0],[0,4,5,0],[0,3,1,0]]

/* -------------------------------- Approach -------------------------------- */

// Brute:
// Traverse the matrix, find zero, mark the cells that will flipped to 0 with a marker
// Again traverse the matrix, find the marked cells and assign them to 0

// Better:
// Store info about zero cell's column in a vec
// If in inner loop, found a zero cell, then push column to mentioned vec
// Paint all cells in that row to 0
// Paint all cells in columns where zero was found to zero

// Optimal:
// The time complexity is minimal as the traversal of a matrix takes at least O(N*M)(where N = row and M = column).
// So only space complexity can be improved.
// So, instead of using two extra matrices row and col,
// we will use the 1st row and 1st column of the given matrix to keep a track of the zero cells
// To consider matrix[0][0] occuring twice, we will use another variable for col[0]

/* ------------------------------------ x ----------------------------------- */

// use std::ops::Index;
// use num::{Integer, NumCast};

// use crate::array::medium::problem10::compile_time_matrix::Matrix;

// trait Indexed: Index<(usize, usize)> {
//     fn len(&self) -> usize;
// }

// mod compile_time_matrix {
//     // Compile-Time matrices:
//     // Using const generics it's now very easy to define the matrix object with an array of arrays:
//     // Here, the amount of rows and columns are hard-coded into the data-type.
//     // So to resize you need to a create a new object and all types must be known (defined) at compile time.

//     use super::*;

//     pub struct Matrix<T, const ROWS: usize, const COLS: usize> where T : Integer+ NumCast {
//         data: [[T; COLS]; ROWS],
//     }

//     impl<T:  Integer+ NumCast, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
//         pub fn new(data: [[T; COLS]; ROWS]) -> Self {
//             Self { data }
//         }
//     }

//     impl<T: Integer+ NumCast, const ROWS: usize, const COLS: usize> Index<(usize, usize)>
//         for Matrix<T, ROWS, COLS>
//     {
//         type Output = T;

//         fn index(&self, index: (usize, usize)) -> &Self::Output {
//             &self.data[index.0][index.1]
//         }
//     }

//     impl<T:  Integer+ NumCast, const ROWS: usize, const COLS: usize> Indexed for Matrix<T, ROWS, COLS> {
//         fn len(&self) -> usize {
//             self.data.len()
//         }
//     }

//     // impl<T:Integer+ NumCast + Zero>
// }

// mod dynamic_matrix {
//     use super::*;

//     pub struct Matrix<T> where T :  Integer+ NumCast {
//         data: Vec<Vec<T>>,
//     }

//     impl<T> Matrix<T>
//     where
//         T:  Integer+ NumCast,
//     {
//         pub fn new(data: Vec<Vec<T>>) -> Self {
//             Self { data }
//         }
//     }

//     impl<T> Index<(usize, usize)> for Matrix<T>
//     where
//         T:  Integer+ NumCast,
//     {
//         type Output = T;

//         fn index(&self, index: (usize, usize)) -> &Self::Output {
//             &self.data[index.0][index.1]
//         }
//     }

//     impl<T: Integer+ NumCast> Indexed for Matrix<T> {
//         fn len(&self) -> usize {
//             self.data.len()
//         }
//     }
// }

// fn brute_generic<MatrixType: Indexed>(arr: &mut MatrixType) where <MatrixType as std::ops::Index<(usize, usize)>>::Output: PartialEq<i32> + Sized + From<u32>  {
//     let n = arr.len();
//     let m = n;
//     let mut zeros: Vec<usize> = vec![];
//     let mut zero_count = 0;

//     for row in 0..n {
//         let mut zero_in_row = false;
//         for column in 0..m {
//             if arr[(row, column)] == 0 {
//                 zeros.push(column);
//                 zero_count += 1;
//                 zero_in_row = true;
//             }
//         }
//         if zero_in_row {
//             for column in 0..m {
//                 arr[(row, column)] = NumCast::from(0).unwrap();
//             }
//         }
//     }
//     if zero_count > 0 {
//         for column in zeros {
//             for row in 0..n {
//                 arr[(row, column)] = 0;
//             }
//         }
//     }
// }

// Time:  O(2*(N*M))
// Space: O(1)
fn optimal(matrix: &mut Vec<Vec<u32>>, n: usize, m: usize) {
    let mut col_zero = 1;

    // 1.  Traverse the matrix and mark 1st row & col accordingly:
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 0 {
                // mark i-th row:
                matrix[i][0] = 0;

                // mark j-th column:
                if j != 0 {
                    matrix[0][j] = 0;
                } else {
                    col_zero = 0;
                }
            }
        }
    }

    // 2. Mark with 0 from (1,1) to (n-1, m-1)
    for i in (1..n) {
        for j in (1..m) {
            if matrix[i][j] != 0 && matrix[i][0] == 0 || matrix[0][j] == 0 {
                matrix[i][j] = 0;
            }
        }
    }

    // 3. Finally mark the 1st col & then 1st row:
    if matrix[0][0] == 0 {
        for j in 0..m {
            matrix[0][j] = 0;
        }
    }
    if col_zero == 0 {
        for i in 0..n {
            matrix[i][0] = 0;
        }
    }
}

mod brute {

    //Time: O((N*M)*(N + M)) + O(N*M)
    // Space: O(1)
    fn set_matrix_zeroes(matrix: &mut Vec<Vec<u32>>, n: usize, m: usize) {
        let mark = u32::MAX;

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    mark_row(matrix, n, m, i);
                    mark_col(matrix, n, m, j);
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == mark {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    fn mark_row(matrix: &mut Vec<Vec<u32>>, n: usize, m: usize, i: usize) {
        let mark = u32::MAX;
        for j in 0..m {
            if matrix[i][j] != 0 {
                matrix[i][j] = mark;
            }
        }
    }

    fn mark_col(matrix: &mut Vec<Vec<u32>>, n: usize, m: usize, j: usize) {
        let mark = u32::MAX;
        for i in 0..n {
            if matrix[i][j] != 0 {
                matrix[i][j] = mark;
            }
        }
    }
}

// Time - O(N*M)
// Space - O(zeroes(N,M) < N*M)
fn better(arr: &mut Vec<Vec<u32>>) {
    let n = arr.len();
    let m = arr[0].len();

    let mut zeros: Vec<usize> = vec![];
    let mut zero_count = 0;

    for row in 0..n {
        let mut zero_in_row = false;
        for column in 0..m {
            if arr[row][column] == 0 {
                zeros.push(column);
                zero_count += 1;
                zero_in_row = true;
            }
        }
        if zero_in_row {
            for column in 0..m {
                arr[row][column] = 0;
            }
        }
    }
    if zero_count > 0 {
        for column in zeros {
            for row in 0..n {
                arr[row][column] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test_setting_matrix_zero {
    use super::*;

    #[test]
    fn optimal_ok() {
        let mut arr = vec![
            vec![1, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![0, 1, 1, 1],
        ];
        better(&mut arr);
        assert_eq!(
            arr,
            vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0]
            ]
        );

        let mut arr = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        better(&mut arr);
        assert_eq!(
            arr,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }

    #[test]
    fn better_ok() {
        let mut arr = vec![vec![1u32, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        better(&mut arr);
        assert_eq!(arr, vec![vec![1u32, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut arr = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        better(&mut arr);
        assert_eq!(
            arr,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
