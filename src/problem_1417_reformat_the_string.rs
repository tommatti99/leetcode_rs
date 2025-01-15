use std::{fmt::format, vec};

/*
You are given an alphanumeric string s. (Alphanumeric string is a string consisting of lowercase English letters and digits).

You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit. That is, no two adjacent characters have the same type.

Return the reformatted string or return an empty string if it is impossible to reformat the string.

 
Example 1:

Input: s = "a0b1c2"
Output: "0a1b2c"
Explanation: No two adjacent characters have the same type in "0a1b2c". "a0b1c2", "0a1b2c", "0c2a1b" are also valid permutations.

Example 2:

Input: s = "leetcode"
Output: ""
Explanation: "leetcode" has only characters so we cannot separate them by digits.

Example 3:

Input: s = "1229857369"
Output: ""
Explanation: "1229857369" has only digits so we cannot separate them by characters.

 
*/
/*
use crate::Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let nums: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
        let mut res: String = String::new();
        let mut vec_char = Vec::new();
        let mut vec_numb = Vec::new();

        if !s.contains(nums) || !s.parse::<i128>().is_err() {
            return "".to_string();
        }

        for c in s.chars() {
            if nums.contains(&c) {
                vec_numb.push(c);
                continue;
            }
            vec_char.push(c);
        }

        if vec_char.len() < vec_numb.len() - 1 || vec_char.len() -1 > vec_numb.len() {
            return "".to_string();
        }
        
        for idx in 0..s.len() {
            res = format!("{}{}{}",res, vec_numb.get(idx).map_or("", |&v| &v.clone().to_string()), vec_char.get(idx).unwrap_or(&'\0'));
        }


        res
    }
}

#[test]
pub fn test_reformat() {
    assert_eq!(Solution::reformat("1229857369".to_string()), "".to_string());
} */