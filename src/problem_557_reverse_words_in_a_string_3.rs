/*
557. Reverse Words in a String III

Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

 
Example 1:

Input: s = "Let's take LeetCode contest"
Output: "s'teL ekat edoCteeL tsetnoc"

Example 2:

Input: s = "Mr Ding"
Output: "rM gniD"

 

Constraints:

    1 <= s.length <= 5 * 104
    s contains printable ASCII characters.
    s does not contain any leading or trailing spaces.
    There is at least one word in s.
    All the words in s are separated by a single space.


*/
use crate::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result: Vec<String>  = Vec::new();
        let mut word: Vec<char> = Vec::new();
        let mut left: usize = 0;
        let mut right: usize = 1;
        let mut left_bf: char = ' ';
        let mut right_bf: char = ' ';
            
        let s_split: std::str::Split<'_, char> = s.split(' ');
        
        s_split.for_each(|x: &str| {
            word.clear();
            left = 0;
    
            let _ = x.chars().into_iter().for_each(|x| word.push(x));
            
            right = word.len() - 1;
            
            for _ in 0..=word.len() {

                if left > right {
                    break;
                }
                
                if left != right {
                    left_bf = word[left];
                    right_bf = word[right];
                    
                    word[right] = left_bf;
                    word[left] = right_bf;

                    left += 1;
                    right -= 1;
                }
            }
            
            result.push(word.clone().into_iter().collect::<String>());
            result.push(' '.to_string());
            
        });

        if result[result.len()-1] == ' '.to_string() {
            result.pop();
        }
        
        return result.into_iter().collect::<String>();
    }
}

#[test]
pub fn test_reverse_words() {
    assert_eq!(Solution::reverse_words("Let's take LeetCode contest".to_string()), "s'teL ekat edoCteeL tsetnoc".to_string());    
}
