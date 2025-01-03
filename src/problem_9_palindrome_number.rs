/*
9. Palindrome Number

Given an integer x, return true if x is a palindrome, and false otherwise. 

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

*/
use crate::Solution;

impl Solution {

    pub fn is_palindrome(x: i32) -> bool {
        let xs: String = x.to_string();
        let xs_len: usize = xs.len();
        let xs_chars = xs.chars();
        let xs_chars_back = xs.chars().rev();

        if xs_len < 2 {
            return true;
        }

        for i in 0..=(xs_len / 2) - 1 {
            if xs_chars.clone().nth(i).unwrap() != xs_chars_back.clone().nth(i).unwrap() {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_is_pair() {
    assert_eq!(Solution::is_palindrome(3), true);
    assert_eq!(Solution::is_palindrome(321), false);
    assert_eq!(Solution::is_palindrome(2442), true);
    assert_eq!(Solution::is_palindrome(24142), true);
}