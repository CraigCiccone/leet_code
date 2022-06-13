// MEDIUM
//
// Given a triangle array, return the minimum path sum from top to bottom.
// For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

// Example 1:

// Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
// Output: 11
// Explanation: The triangle looks like:
//    2
//   3 4
//  6 5 7
// 4 1 8 3
// The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).

// Example 2:

// Input: triangle = [[-10]]
// Output: -10

// Constraints:

// 1 <= triangle.length <= 200
// triangle[0].length == 1
// triangle[i].length == triangle[i - 1].length + 1
// -104 <= triangle[i][j] <= 104

// Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?

use std::cmp;

fn depth_first_search(triangle: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    if row == triangle.len() {
        return 0;
    }

    let sum_left = triangle[row][col] + depth_first_search(triangle, row + 1, col);
    let sum_right = triangle[row][col] + depth_first_search(triangle, row + 1, col + 1);

    return cmp::min(sum_left, sum_right);
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    depth_first_search(&triangle, 0, 0)
}

fn main() {
    println!(
        "{}",
        minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    );
    println!("{}", minimum_total(vec![vec![-10]]));
}
