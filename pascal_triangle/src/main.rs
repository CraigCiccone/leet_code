// EASY
//
// Given an integer numRows, return the first numRows of Pascal's triangle.
// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
//
// Example 1:
//
// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
//
// Example 2:
//
// Input: numRows = 1
// Output: [[1]]
//
// Constraints:
//
// 1 <= numRows <= 30

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut rows = vec![vec![1]];

    for row_num in 1..num_rows {
        let prev_row = &rows[(row_num - 1) as usize];
        let mut new_row = vec![];

        for col in 0..=row_num {
            if col == 0 || col == row_num {
                new_row.push(1);
            } else {
                new_row.push(prev_row[col as usize] + prev_row[col as usize - 1]);
            }
        }

        rows.push(new_row);
    }

    rows
}

fn main() {
    println!("{:?}", generate(5));
    println!("{:?}", generate(1));
}
