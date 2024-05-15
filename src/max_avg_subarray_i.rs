// 643. Maximum Average Subarray I
// https://leetcode.com/problems/maximum-average-subarray-i/

pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        if nums.len() == 1 && k == 1 {
            return nums[0] as f64;
        }
        let k = k as usize;
        let mut current_sum = nums.iter().take(k).sum::<i32>() as f64;
        let mut max_sum = current_sum;

        for i in k..nums.len() {
            current_sum -= nums[i - k] as f64;
            current_sum += nums[i] as f64;
            max_sum = max_sum.max(current_sum);
        }

        max_sum / (k as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [1, 12, -5, -6, 50, 3];
        let k = 4;

        // when
        let result = Solution::find_max_average(nums.into(), k);

        // then
        assert!((result - 12.75000).abs() < 0.000001);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [5];
        let k = 1;

        // when
        let result = Solution::find_max_average(nums.into(), k);

        // then
        assert!(
            (result - 5.00000).abs() < 0.000001,
            "Expected 5.00000, got {}",
            result
        );
    }

    #[test]
    pub fn example109() {
        // given
        let nums = [0, 1, 1, 3, 3];
        let k = 4;

        // when
        let result = Solution::find_max_average(nums.into(), k);

        // then
        assert!(
            (result - 2.00000).abs() < 0.000001,
            "Expected 2.00000, got {}",
            result
        );
    }
}
