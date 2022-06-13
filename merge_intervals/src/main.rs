// MEDIUM
//
// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and
// return an array of the non-overlapping intervals that cover all the intervals in the input.
//
// Example 1:
//
// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
//
// Example 2:
//
// Input: intervals = [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//
// Constraints:
//
// 1 <= intervals.length <= 10^4
// intervals[i].length == 2
// 0 <= starti <= endi <= 10^4

use std::cmp;

fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    if b[0] >= a[0] && b[0] <= a[1] {
        true
    } else {
        false
    }
}

fn combine(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    vec![cmp::min(a[0], b[0]), cmp::max(a[1], b[1])]
}

fn run(intervals: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut results = intervals.clone();

    'outer: for (i, a) in intervals.iter().enumerate() {
        for (j, b) in intervals.iter().enumerate() {
            if i != j && overlap(a, b) {
                results.retain(|interval| interval != a && interval != b);
                let merged = combine(a, b);
                if !results.contains(&merged) {
                    results.push(merged);
                    break 'outer;
                }
            }
        }
    }

    results
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut old = intervals.clone();
    let mut new = run(&old);

    while new != old {
        old = new;
        new = run(&old);
    }

    new
}

fn main() {
    println!(
        "{:?}",
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
    println!("{:?}", merge(vec![vec![1, 4], vec![4, 5]]));
    println!("{:?}", merge(vec![vec![1, 4], vec![4, 5], vec![4, 5]]));
    println!("{:?}", merge(vec![vec![1, 4], vec![2, 3]]));
}
