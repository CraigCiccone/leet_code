// EASY
//
// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
//
// Example 1:
//
// Input: s = "anagram", t = "nagaram"
// Output: true
//
// Example 2:
//
// Input: s = "rat", t = "car"
// Output: false
//
// Constraints:
//
// 1 <= s.length, t.length <= 5 * 10^4
// s and t consist of lowercase English letters.

fn is_anagram(s: String, t: String) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort_unstable();
    t_chars.sort_unstable();

    s_chars == t_chars
}

fn main() {
    println!(
        "{}",
        is_anagram("anagram".to_string(), "nagaram".to_string())
    );
    println!("{}", is_anagram("rat".to_string(), "car".to_string()));
}
