/*
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

 

Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

*/
/*
use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {

        if strs.len() == 1 {
            return strs[0].clone();
        }
        
        let res_vec: Vec<char> = strs[0].chars().collect();

        println!("len: {},  res_vec{}", strs.len(), res_vec.iter().collect());

        for i in 0..=strs.len() {
                      
            if strs[i].len() == 1 {
                return "".to_string();
            }

            for (idx, ch) in strs[i].chars().enumerate() {
                println!("{} {}", ch, res_vec[idx]);
                if res_vec[idx] != ch {
                    return res_vec[0..idx].iter().collect();
                }
            };

            if res_vec.len() == 0 {
                return "".to_string();
            }
        }
    
        res_vec.iter().collect()
    }
}

#[test]
pub fn test_longest_common_prefix() {
    assert_eq!(Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]), "a");
}*/