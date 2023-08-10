// 239. Sliding Window Maximum
// From: https://leetcode.com/problems/sliding-window-maximum/

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::with_capacity(nums.len() - k + 1);

        let mut largest_item_indices = VecDeque::with_capacity(k);
        for (index, item) in nums.iter().take(k).enumerate() {
            // remove all items smaller than the newest one
            while !largest_item_indices.is_empty() && *item > nums[*largest_item_indices.back().unwrap()] {
                largest_item_indices.pop_back();
            }
            largest_item_indices.push_back(index);
        }
        result.push(nums[*largest_item_indices.front().unwrap()]);

        for index in k..nums.len() {
            let item = nums[index];
            // remove all items smaller than the new item in the window
            while !largest_item_indices.is_empty() && item > nums[*largest_item_indices.back().unwrap()] {
                largest_item_indices.pop_back();
            }
            largest_item_indices.push_back(index);

            // if the largest is outside the window, remove it
            if *largest_item_indices.front().unwrap() == index - k {
                largest_item_indices.pop_front();
            }
            result.push(nums[*largest_item_indices.front().unwrap()]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;

        // when
        let result = Solution::max_sliding_window(nums, k);

        // then
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn example2() {
        // given
        let nums = vec![1];
        let k = 1;

        // when
        let result = Solution::max_sliding_window(nums, k);

        // then
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn example49() {
        // given
        let nums = vec![9, 10, 9, -7, -4, -8, 2, -6];
        let k = 5;

        // when
        let result = Solution::max_sliding_window(nums, k);

        // then
        assert_eq!(result, vec![10, 10 , 9, 2]);
    }
}