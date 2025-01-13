/*
You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

Increment the large integer by one and return the resulting array of digits.

 

Example 1:

Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].

Example 2:

Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].

Example 3:

Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].

*/

use crate::Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits.clone();
        let mut carry = 0;

        for (idx, mut numb) in digits.clone().into_iter().rev().enumerate() {
            if idx == 0 {
                numb += 1;
            }

            if carry > 0 {
                numb += 1;
                carry = 0;
            }
            
            if numb > 9 {
                numb = numb - 10;
                digits[idx] = numb;
                carry = 1;

                if idx + 1 == digits.len() {
                    digits.push(1);
                }
            }

            digits[idx] = numb;
        
        }
        digits.into_iter().rev().collect()
    }
}

#[test]
pub fn test_plus_one() {
    assert_eq!(Solution::plus_one(vec![9,9]), vec![1,0,0]);
    assert_eq!(Solution::plus_one(vec![9]), vec![1,0]);
    assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
}