/* ----------------------------- Set Matrix Zero ---------------------------- */
// Problem Statement:
// Given a matrix if an element in the matrix is 0 then you will have to
// set its entire column and row to 0 and then return the matrix.
// Note: Matrix is a square matrix

// Input: matrix=[[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]

// Input: matrix=[[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output:[[0,0,0,0],[0,4,5,0],[0,3,1,0]]

/* ------------------------------------ x ----------------------------------- */

trait Index {
    type Output;

    fn index(&self, index: (usize, usize)) -> Self::Output;
}

mod compile_time_matrix {
    // Compile-Time matrices:
    // Using const generics it's now very easy to define the matrix object with an array of arrays:
    // Here, the amount of rows and columns are hard-coded into the data-type.
    // So to resize you need to a create a new object and all types must be known (defined) at compile time.

    pub struct Matrix<T: Copy, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        pub fn new(data: [[T; COLS]; ROWS]) -> Self {
            Self { data }
        }

        fn index(&self, index: (usize, usize)) -> T {
            self.data[index.0][index.1]
        }
    }

    // fn brute(arr: &mut Matrix<u32>) {
    //     let n = arr.len();
    //     let m = arr[0].len();
    //     if n != m {
    //         println!("Matrix should be a square");
    //     }
    //     let mut zeros: Vec<usize> = vec![];
    //     let mut zero_count = 0;

    //     for row in 0..n {
    //         let mut zero_in_row = false;
    //         for column in 0..m {
    //             if arr[row][column] == 0 {
    //                 zeros.push(column);
    //                 zero_count +=1;
    //                 zero_in_row = true;
    //             }
    //         }
    //         if zero_in_row{
    //             for column in 0..m{
    //                 arr[row][column] = 0;
    //             }
    //         }
    //     }
    //     if zero_count > 0 {
    //         for column in zeros{
    //             for row in 0..n{
    //                 arr[row][column] = 0;
    //             }
    //         }
    //     }
    // }
}

mod dynamic_matrix {
    use super::Index;

    pub struct Matrix<T:Copy> {
        data: Vec<Vec<T>>,
    }

    impl<T> Matrix<T> where T:Copy{
        pub fn new(data: Vec<Vec<T>>) -> Self {
            Self { data }
        }
    }

    impl<T> Index for Matrix<T> where T:Copy {
        type Output = T;

        fn index(&self, index: (usize, usize)) -> Self::Output {
            self.data[index.0][index.1].clone()
        }
    }
}


fn brute(arr: &mut Vec<Vec<u32>>) {
    let n = arr.len();
    let m = arr[0].len();
    if n != m {
        println!("Matrix should be a square");
    }
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
    fn brute_ok() {
        let mut arr = vec![vec![1u32, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        brute(&mut arr);
        println!("{:?}", arr);
        assert_eq!(arr, vec![vec![1u32, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }
}
