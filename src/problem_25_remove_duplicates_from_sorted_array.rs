/*
Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

    Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
    Return k.

Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}

If all assertions pass, then your solution will be accepted.
*/

use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut h_map: HashMap<i32, usize> = HashMap::new();
        let mut remv: Vec<usize> = Vec::new();

        for (idx, vlr) in nums.into_iter().enumerate().rev() {
            if h_map.contains_key(vlr) {
                remv.push(idx);
            }
            h_map.insert(*vlr, idx);
        }
        
        for idx in remv {
            nums.remove(idx);
        }

        nums.len() as i32
    }
}

#[test]
pub fn test_remove_duplicates() {
    Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]);
}