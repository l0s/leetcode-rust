// 53. Maximum Subarray
// https://leetcode.com/problems/maximum-subarray

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut global_maximum = i32::MIN;

        // maximum sum for sub arrays ending at each index
        let mut local_maximum = 0;

        for item in nums {
            // local max at index i is the max of the current item and the sum of the current item
            // and the local max at index i - 1
            local_maximum = item.max(item + local_maximum);
            global_maximum = global_maximum.max(local_maximum);
        }

        global_maximum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

        // when
        let result = Solution::max_sub_array(nums.to_vec());

        // the
        assert_eq!(result, 6);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [1];

        // when
        let result = Solution::max_sub_array(nums.to_vec());

        // the
        assert_eq!(result, 1);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = [5, 4, -1, 7, 8];

        // when
        let result = Solution::max_sub_array(nums.to_vec());

        // the
        assert_eq!(result, 23);
    }
}
