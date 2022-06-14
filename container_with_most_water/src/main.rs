// MEDIUM
//
// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
// Find two lines that together with the x-axis form a container, such that the container contains the most water.
// Return the maximum amount of water a container can store.
// Notice that you may not slant the container.
//
// Example 1:
//
// Input: height = [1,8,6,2,5,4,8,3,7]
// Output: 49
// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
//
// Example 2:
//
// Input: height = [1,1]
// Output: 1
//
// Constraints:
//
// n == height.length
// 2 <= n <= 10^5
// 0 <= height[i] <= 10^4

fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        max_area = max_area.max(height[left].min(height[right]) * (right - left) as i32);
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }

    max_area
}

fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", max_area(vec![1, 1]));
}
