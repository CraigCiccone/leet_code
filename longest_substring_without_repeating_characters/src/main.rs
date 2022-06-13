// MEDIUM
//
// Given a string s, find the length of the longest substring without repeating characters.
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
//
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
//
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
// Constraints:
//
// 0 <= s.length <= 5 * 10^4
// s consists of English letters, digits, symbols and spaces.

use std::collections::HashMap;

fn length_of_longest_substring(s: String) -> i32 {
    let mut found = HashMap::new();
    let mut sub_str = vec![];
    let mut max = 0;

    for char in s.chars() {
        if found.contains_key(&char) {
            let shift = *found.get(&char).unwrap() + 1;

            for _ in 0..shift {
                let val = sub_str.remove(0);
                found.remove(&val);
            }

            for value in found.values_mut() {
                *value = *value - shift;
            }
        }

        found.insert(char, sub_str.len());
        sub_str.push(char);

        if sub_str.len() > max {
            max = sub_str.len();
        }
    }

    max as i32
}

fn main() {
    println!("{}", length_of_longest_substring(String::from("abcabcbb"))); // 3 - abc or bca or cab
    println!("{}", length_of_longest_substring(String::from("bbbbb"))); // 1 - b
    println!("{}", length_of_longest_substring(String::from("pwwkew"))); // 3 - wke
    println!("{}", length_of_longest_substring(String::from("aab"))); // 2 - ab
    println!("{}", length_of_longest_substring(String::from("dvdf"))); // 3 - vdf
    println!("{}", length_of_longest_substring(String::from("ckilbkd"))); // 5 - ckilb
    println!("{}", length_of_longest_substring(String::from("asljlj"))); // 4 - aslj
}
