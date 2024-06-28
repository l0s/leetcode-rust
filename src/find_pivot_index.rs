// 724. Find Pivot Index
// https://leetcode.com/problems/find-pivot-index

pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1;
        } else if nums.len() == 1 && nums[0] == 0 {
            return 0;
        }
        let mut prefix_sum = vec![0; nums.len() + 1];
        prefix_sum[0] = 0;
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            prefix_sum[i + 1] = sum;
        }
        for candidate in 0..(prefix_sum.len() - 1) {
            let left_sum = prefix_sum[candidate] - prefix_sum[0];
            let right_sum = prefix_sum[nums.len()] - prefix_sum[candidate + 1];
            if left_sum == right_sum {
                return candidate as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [1, 7, 3, 6, 5, 6];

        // when
        let result = Solution::pivot_index(nums.into());

        // then
        assert_eq!(result, 3);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [1, 2, 3];

        // when
        let result = Solution::pivot_index(nums.into());

        // then
        assert_eq!(result, -1);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = [2, 1, -1];

        // when
        let result = Solution::pivot_index(nums.into());

        // then
        assert_eq!(result, 0);
    }

    #[test]
    pub fn example745() {
        // given
        let nums = [0];

        // when
        let result = Solution::pivot_index(nums.into());

        // then
        assert_eq!(result, 0);
    }
}
