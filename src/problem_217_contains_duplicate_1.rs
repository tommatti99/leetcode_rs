/*
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.


Example 1:

Input: nums = [1,2,3,1]

Output: true

Explanation:

The element 1 occurs at the indices 0 and 3.

Example 2:

Input: nums = [1,2,3,4]

Output: false

Explanation:

All elements are distinct.

Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]

Output: true

*/

use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut h_map: HashMap<i32, usize> = HashMap::new();

        for num in nums.into_iter() {
            if let Some(_) = h_map.get(&num) {
                return true;
            }
            h_map.insert(num, 0);
        }
        false
    }
}

#[test]
pub fn test_contains_duplicate() {
    assert_eq!(Solution::contains_duplicate(vec![1,2,3,1]), true);
    assert_eq!(Solution::contains_duplicate(vec![1,2,3,4]), false);
    assert_eq!(Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
}