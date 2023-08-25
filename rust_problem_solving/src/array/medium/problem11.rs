/* ------------------------ Rotate Image by 90 degree ----------------------- */
// Problem Statement:
// Given a matrix, your task is to rotate the matrix 90 degrees clockwise.

// Input: [[1,2,3],[4,5,6],[7,8,9]]
// Output: [[7,4,1],[8,5,2],[9,6,3]]

// Input: [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
// Output:[[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

/* -------------------------------- Approach -------------------------------- */
// Brute:
// Take another dummy matrix of n*n,
// and then take the first row of the matrix and put it in the last column of the dummy matrix,
// take the second row of the matrix, and put it in the second last column of the matrix and so.

// Better:
// Step 1: Transpose the matrix. (transposing means changing columns to rows and rows to columns)
// Step 2: Reverse each row of the matrix.
/* ------------------------------------ x ----------------------------------- */

// time O(N*N)
// space O(N*N)
fn brute(matrix: &mut Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let n = matrix.len();
    let mut rotated = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[j][n - i - 1] = matrix[i][j];
        }
    }
    rotated
}

// Time: O(N*N) + O(N*N);
// One O(N*N) is for transposing the matrix
// and the other is for reversing the matrix.
fn optimal_clockwise(matrix: &mut Vec<Vec<u32>>) {
    let n = matrix.len();

    // Transpose: Matrix[i][j] => Matrix[j][i]
    for i in 0..n {
        for j in 0..i {
            (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
        }
    }
    // Reverse each row of matrix
    for i in 0..n {
        matrix[i].reverse();
    }

    // println!("{:?}", matrix);
}

fn optimal_anticlockwise(matrix: &mut Vec<Vec<u32>>) {
    let n = matrix.len();

    // Transpose: Matrix[i][j] => Matrix[j][i]
    for i in 0..n {
        for j in 0..i {
            (matrix[i][j], matrix[j][i]) = (matrix[j][i], matrix[i][j]);
        }
    }
    // Reverse each column of matrix
    let mut ind = n - 1;
    for i in 0..n {
        for j in 0..(n / 2) {
            let tmp = matrix[j][i];
            matrix[j][i] = matrix[ind][i];
            matrix[ind][i] = tmp;
            ind -= 1;
        }
        ind = n - 1;
    }

    // println!("{:?}", matrix);
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod test_clockwise_90_rotation {
    use super::*;

    #[test]
    fn brute_ok() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        let matrix = brute(&mut matrix);
        assert_eq!(matrix, res);

        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let res = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        let matrix = brute(&mut matrix);
        assert_eq!(matrix, res);
    }

    #[test]
    fn optimal_ok() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        optimal_clockwise(&mut matrix);

        assert_eq!(matrix, res);

        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let res = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        optimal_clockwise(&mut matrix);
        assert_eq!(matrix, res);
    }
}
