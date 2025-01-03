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

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut ac_ext: i32 = 0;
        let mut ac_int: i32 = 0;
        let mut result: bool = false; 

        nums.clone().into_iter().for_each(|x| {
            ac_int = 0;

            nums.clone().into_iter().for_each(|y| {

                if x == y && ac_int != ac_ext  {
                    if (ac_ext - ac_int).abs() <= k {
                        println!("{}, {}, {}, {}", x, y, ac_int, ac_ext);
                        result = true;
                        return;
                    }
                }
                ac_int += 1;
                });
                
                if result {
                    return;
                }
            ac_ext += 1;
            
            });

        result
    }
}