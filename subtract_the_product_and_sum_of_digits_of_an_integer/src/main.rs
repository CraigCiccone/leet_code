// EASY
//
// Given an integer number n, return the difference between the product of its digits and the sum of its digits.
//
// Example 1:
//
// Input: n = 234
// Output: 15
// Explanation:
// Product of digits = 2 * 3 * 4 = 24
// Sum of digits = 2 + 3 + 4 = 9
// Result = 24 - 9 = 15
//
// Example 2:
//
// Input: n = 4421
// Output: 21
// Explanation:
// Product of digits = 4 * 4 * 2 * 1 = 32
// Sum of digits = 4 + 4 + 2 + 1 = 11
// Result = 32 - 11 = 21
//
// Constraints:
//
// 1 <= n <= 10^5

fn subtract_product_and_sum(n: i32) -> i32 {
    let n_str = n.to_string();
    let nums = n_str.chars();
    let mut prod = 1;
    let mut sum = 0;

    for char in nums {
        let num: i32 = char.to_string().parse().unwrap();
        prod *= num;
        sum += num;
    }

    prod - sum
}

fn main() {
    println!("{}", subtract_product_and_sum(234));
    println!("{}", subtract_product_and_sum(4421));
}
