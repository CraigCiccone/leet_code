// EASY
//
// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
//
// Example 1:
//
// Input: n = 2
// Output: [0,1,1]
// Explanation:
// 0 --> 0
// 1 --> 1
// 2 --> 10
//
// Example 2:
//
// Input: n = 5
// Output: [0,1,1,2,1,2]
// Explanation:
// 0 --> 0
// 1 --> 1
// 2 --> 10
// 3 --> 11
// 4 --> 100
// 5 --> 101
//
// Constraints:
//
// 0 <= n <= 10^5

fn count_bits(n: i32) -> Vec<i32> {
    let mut answer = vec![];

    for num in 0..=n {
        answer.push(format!("{num:b}").chars().filter(|b| *b == '1').count() as i32);
    }

    answer
}

fn main() {
    println!("{:?}", count_bits(2));
    println!("{:?}", count_bits(5));
}
