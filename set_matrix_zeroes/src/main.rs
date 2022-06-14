// MEDIUM
//
// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
// You must do it in place.
//
// Example 1:
//
// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]
//
// Example 2:
//
// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//
// Constraints:
//
// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -2^31 <= matrix[i][j] <= 2^31 - 1

fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut cols = vec![];
    let mut rows = vec![];

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            if *val == 0 {
                cols.push(col_idx);
                rows.push(row_idx);
            }
        }
    }

    for (row_idx, row) in matrix.iter_mut().enumerate() {
        for (col_idx, val) in row.iter_mut().enumerate() {
            if cols.contains(&col_idx) || rows.contains(&row_idx) {
                *val = 0;
            }
        }
    }

    println!("{:?}", matrix);
}

fn main() {
    set_zeroes(&mut vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
    set_zeroes(&mut vec![
        vec![0, 1, 2, 0],
        vec![3, 4, 5, 2],
        vec![1, 3, 1, 5],
    ]);
}
