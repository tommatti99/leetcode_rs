/*
1. Two Sum

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]                                                                                                                                                                                                                                                                                                                                                               
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
                                     nmhjp´´                  nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´´cv 

Constraints:

    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
    Only one valid answer exists.

 
Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity? 
*/

use crate::Solution;

use std::{collections::HashMap, vec};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h_map: HashMap<i32, usize> = HashMap::new();

        for (idx, numb) in nums.into_iter().enumerate() {
            if let Some(&idx_2) = h_map.get(&(&target - numb)) {
                return vec![idx_2 as i32, idx as i32];
            }
            h_map.insert( numb, idx);
        }
        vec![0,0]
    }
}

#[test]
pub fn test_two_sum() {
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), [0,1]);    
    assert_eq!(Solution::two_sum(vec![3,2,4], 6), [1,2]);
    assert_eq!(Solution::two_sum(vec![3,3], 6), [0,1]);    
}