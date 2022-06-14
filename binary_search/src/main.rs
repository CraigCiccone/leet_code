// EASY
//
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums.
// If target exists, then return its index. Otherwise, return -1.
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
//
// Example 2:
//
// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1
//
// Constraints:
//
// 1 <= nums.length <= 10^4
// -10^4 < nums[i], target < 10^4
// All the integers in nums are unique.
// nums is sorted in ascending order.

use std::cmp::Ordering;

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low_idx = 0;
    let mut high_idx = nums.len() as i32 - 1;

    while low_idx <= high_idx {
        let mid_idx = high_idx + low_idx / 2;

        match target.cmp(&nums[mid_idx as usize]) {
            Ordering::Equal => return mid_idx,
            Ordering::Greater => low_idx = mid_idx + 1,
            Ordering::Less => high_idx = mid_idx - 1,
        }
    }

    -1
}

fn main() {
    println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 9));
    println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 2));
    println!("{}", search(vec![-1, 0, 3, 5, 9, 12, 13], 0));
    println!("{}", search(vec![-5, -1, 0, 3, 5, 9, 12, 13], -5));
    println!("{}", search(vec![5], -5));
}
