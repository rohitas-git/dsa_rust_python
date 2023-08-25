/* ----------------------- Spiral Traversal of Matrix ----------------------- */
// Problem Statement:
// Given a Matrix, print the given matrix in spiral order.

// Input: Matrix[][] =
//      { { 1, 2, 3, 4 },
//      { 5, 6, 7, 8 },
//      { 9, 10, 11, 12 },
//      { 13, 14, 15, 16 } }
//
// Outhput: 1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10.

fn traverse_spirally(matrix: &Vec<Vec<u32>>) {
    let n = matrix.len();
    let mut top = (0, 0);
    let mut right = (top.0, top.1 + n - 1);
    let mut left = (top.0 + n - 1, top.1);
    let mut bottom = (top.0 + n - 1, top.1 + n - 1);
    while top.0 <= bottom.0 && top.1 <= bottom.1 {
        // 1. top -> right
        print!("|");
        for i in top.1..=right.1 {
            print!(" {} ", matrix[top.0][i]);
        }
        print!("|->");
        top.0 += 1;
        top.1 += 1;

        // 2. right to bottom
        print!("|");
        for i in (right.0 + 1..=bottom.0) {
            print!(" {} ", matrix[i][right.1]);
        }
        print!("|->");
        right.0 += 1;
        right.1 -= 1;

        // 3. bottom -> left
        print!("|");
        for i in (left.1..=(bottom.1 - 1)).rev() {
            print!(" {} ", matrix[bottom.0][i]);
        }        
        print!("|->");

        bottom.0 -= 1;
        bottom.1 -= 1;

        // 4. left -> right
        print!("|");
        for i in (top.0..=(left.0 - 1)).rev() {
            print!(" {} ", matrix[i][left.1]);
        }
        print!("|->");
        left.0 -= 1;
        left.1 += 1;

        println!("-->");
    }
}

#[cfg(test)]
mod test_spiral_traversal {
    use super::*;

    #[test]
    fn test_solution() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        traverse_spirally(&matrix);
    }
}
