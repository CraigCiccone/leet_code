// EASY
//
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
// Example 3:
//
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
// Constraints:
//
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, lhs) in nums.iter().enumerate() {
        for (j, rhs) in nums.iter().enumerate() {
            if lhs + rhs == target && i != j {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
}
