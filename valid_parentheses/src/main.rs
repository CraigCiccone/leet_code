// EASY
//
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
//
// Example 1:
//
// Input: s = "()"
// Output: true
//
// Example 2:
//
// Input: s = "()[]{}"
// Output: true
//
// Example 3:
//
// Input: s = "(]"
// Output: false
//
// Constraints:
//
// 1 <= s.length <= 10^4
// s consists of parentheses only '()[]{}'.

fn check_close(expected: char, stack: &mut Vec<char>) -> bool {
    if let Some(open) = stack.pop() {
        open == expected
    } else {
        false
    }
}

fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for char in s.chars() {
        match char {
            '(' => stack.push(char),
            '[' => stack.push(char),
            '{' => stack.push(char),
            ')' => {
                if !check_close('(', &mut stack) {
                    return false;
                }
            }
            ']' => {
                if !check_close('[', &mut stack) {
                    return false;
                }
            }
            '}' => {
                if !check_close('{', &mut stack) {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("([)]".to_string()));
    println!("{}", is_valid("()".to_string()));
    println!("{}", is_valid("()[]{}".to_string()));
    println!("{}", is_valid("(]".to_string()));
    println!("{}", is_valid("(][)".to_string()));
    println!("{}", is_valid("(([]{}()))".to_string()));
    println!("{}", is_valid("(([]{}()}))".to_string()));
    println!("{}", is_valid("(()".to_string()));
    println!("{}", is_valid("(((".to_string()));
    println!("{}", is_valid("]".to_string()));
}
