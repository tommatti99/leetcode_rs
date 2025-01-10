/*
219. Contains Duplicate II

Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

 
Example 1:

Input: nums = [1,2,3,1], k = 3
Output: true

Example 2:

Input: nums = [1,0,1,1], k = 1
Output: true

Example 3:

Input: nums = [1,2,3,1,2,3], k = 2
Output: false

 

Constraints:

    1 <= nums.length <= 105
    -109 <= nums[i] <= 109
    0 <= k <= 105

*/

use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut h_map: HashMap<i32, usize> = HashMap::new();
        let k: usize = k as usize;

        for (i, numb ) in nums.into_iter().enumerate() {
            if let Some(&j) = h_map.get(&numb) {
                if (i - j) <= k as usize{
                    return true;
                }
            }
            h_map.insert(numb, i);
            } 
    false
    }
}


