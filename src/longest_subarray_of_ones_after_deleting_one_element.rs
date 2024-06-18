// 1493. Longest Subarray of 1's After Deleting One Element
// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if nums.is_empty() || nums.len() == 1 {
            return 0;
        }
        let mut left = 0usize; // left window boundary (inclusive)
        let mut right = left + 1; // right window boundary (exclusive)
        let mut result = 0usize;
        let mut count_zeroes = if nums[left] == 0 { 1usize } else { 0usize };
        while right < nums.len() {
            // expand the window to the right
            count_zeroes += if nums[right] == 0 { 1usize } else { 0usize };
            right += 1;

            // if the window has too many zeroes, shrink it by pulling in the left
            while count_zeroes > 1 && left < right {
                if nums[left] == 0 {
                    count_zeroes -= 1;
                }
                left += 1;
            }
            // count zeroes should be 0 or 1
            let window_size = right - left;
            let subarray_length_after_deleting_one_element = window_size - 1;
            if subarray_length_after_deleting_one_element > result {
                result = subarray_length_after_deleting_one_element;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [1, 1, 0, 1];

        // when
        let result = Solution::longest_subarray(nums.into());

        // then
        assert_eq!(result, 3);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [0, 1, 1, 1, 0, 1, 1, 0, 1];

        // when
        let result = Solution::longest_subarray(nums.into());

        // then
        assert_eq!(result, 5);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = [1, 1, 1];

        // when
        let result = Solution::longest_subarray(nums.into());

        // then
        assert_eq!(result, 2);
    }
}
