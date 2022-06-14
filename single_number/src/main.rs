// EASY
//
// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
// You must implement a solution with a linear runtime complexity and use only constant extra space.
//
// Example 1:
//
// Input: nums = [2,2,1]
// Output: 1
//
// Example 2:
//
// Input: nums = [4,1,2,1,2]
// Output: 4
//
// Example 3:
//
// Input: nums = [1]
// Output: 1
//
// Constraints:
//
// 1 <= nums.length <= 3 * 10^4
// -3 * 10^4 <= nums[i] <= 3 * 10^4
// Each element in the array appears twice except for one element which appears only once.

use std::collections::HashMap;

fn single_number(nums: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for num in nums {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }

    for (key, val) in counts {
        if val == 1 {
            return key;
        }
    }

    0
}

fn main() {
    println!("{}", single_number(vec![2, 2, 1]));
    println!("{}", single_number(vec![4, 1, 2, 1, 2]));
    println!("{}", single_number(vec![1]));
}
