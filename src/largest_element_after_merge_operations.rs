// 2789. Largest Element in an Array after Merge Operations
// From: https://leetcode.com/problems/largest-element-in-an-array-after-merge-operations/

pub struct Solution;

impl Solution {
    pub fn max_array_value(nums: Vec<i32>) -> i64 {
        max(nums.iter().map(|small| *small as u64).collect()) as i64
    }
}

fn max(mut nums: Vec<u64>) -> u64 {
    let mut i = nums.len() - 1;
    while i > 0 {
        if nums[i] >= nums[i - 1] {
            nums[i] += nums[i - 1];
            nums.remove(i - 1);
        }
        i -= 1;
    }
    nums[0]
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = vec![2, 3, 7, 9, 3];

        // when
        let result = Solution::max_array_value(nums);

        // then
        assert_eq!(21, result);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = vec![5, 3, 3];

        // when
        let result = Solution::max_array_value(nums);

        // then
        assert_eq!(11, result);
    }

    #[test]
    pub fn example4() {
        // given
        let nums = vec![34, 95, 50, 12, 25, 100, 21, 3, 25, 16, 76, 73, 93, 46, 18];

        // when
        let result = Solution::max_array_value(nums);

        // then
        assert_eq!(623, result);
    }

    #[test]
    pub fn example49() {
        // given
        let nums = vec![
            68, 86, 34, 99, 4, 6, 24, 88, 26, 83, 2, 33, 37, 79, 30, 60, 56, 44, 53, 4, 86, 60, 13,
            81, 95, 28, 83, 24,
        ];

        // when
        let result = Solution::max_array_value(nums);

        // then
        assert_eq!(1362, result);
    }
}
