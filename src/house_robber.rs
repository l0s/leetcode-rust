// 198. House Robber
// https://leetcode.com/problems/house-robber/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        max_amount(&nums, &mut HashMap::new())
    }
}

fn max_amount(nums: &[i32], cache: &mut HashMap<Vec<i32>, i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    } else if nums.len() == 1 {
        return nums[0];
    } else if let Some(cached_result) = cache.get(nums) {
        return *cached_result;
    }

    // rob first house
    let first_amount = nums[0];
    let remaining = max_amount(&nums[2..], cache);
    let first_option = first_amount + remaining;

    // skip first house
    let second_option = max_amount(&nums[1..], cache);

    let result = first_option.max(second_option);
    cache.insert(nums.to_vec(), result);
    result
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let nums = [1, 2, 3, 1];

        // when
        let result = Solution::rob(nums.to_vec());

        // then
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        // given
        let nums = [2, 7, 9, 3, 1];

        // when
        let result = Solution::rob(nums.to_vec());

        // then
        assert_eq!(result, 12);
    }
}
