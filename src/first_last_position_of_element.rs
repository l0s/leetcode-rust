// 34. Find First and Last Position of Element in Sorted Array
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        if let Ok(match_index) = nums.binary_search(&target) {
            eprintln!("Found {} at {}", target, match_index);
            let mut left_index = match_index;
            while left_index > 0 && nums[left_index - 1] == target {
                left_index -= 1;
            }

            let mut right_index = match_index;
            while right_index + 1 < nums.len() && nums[right_index + 1] == target {
                right_index += 1;
            }

            vec![left_index as i32, right_index as i32]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;

        // when
        let result = Solution::search_range(nums, target);

        // then
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;

        // when
        let result = Solution::search_range(nums, target);

        // then
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = vec![];
        let target = 0;

        // when
        let result = Solution::search_range(nums, target);

        // then
        assert_eq!(result, vec![-1, -1]);
    }

    #[test]
    pub fn example5() {
        // given
        let nums = vec![1];
        let target = 1;

        // when
        let result = Solution::search_range(nums, target);

        // then
        assert_eq!(result, vec![0, 0]);
    }
}
