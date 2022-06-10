// EASY
//
// Given the array nums consisting of 2n elements in the form [x1,x2,...,xn,y1,y2,...,yn].
// Return the array in the form [x1,y1,x2,y2,...,xn,yn].
//
// Example 1:
//
// Input: nums = [2,5,1,3,4,7], n = 3
// Output: [2,3,5,4,1,7]
// Explanation: Since x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 then the answer is [2,3,5,4,1,7].
//
// Example 2:
//
// Input: nums = [1,2,3,4,4,3,2,1], n = 4
// Output: [1,4,2,3,3,2,4,1]
//
// Example 3:
//
// Input: nums = [1,1,2,2], n = 2
// Output: [1,2,1,2]
//
// Constraints:
//
// 1 <= n <= 500
// nums.length == 2n
// 1 <= nums[i] <= 10^3

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut nums1 = vec![];
    for idx in 0..n {
        nums1.push(nums[idx as usize]);
    }

    let mut nums2 = vec![];
    for idx in n..nums.len() as i32 {
        nums2.push(nums[idx as usize]);
    }

    let mut new_nums = vec![];
    for idx in 0..nums1.len() {
        new_nums.push(nums1[idx]);
        new_nums.push(nums2[idx]);
    }

    new_nums
}

fn main() {
    println!("{:?}", shuffle(vec![2, 5, 1, 3, 4, 7], 3));
    println!("{:?}", shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4));
    println!("{:?}", shuffle(vec![1, 1, 2, 2], 2));
}
