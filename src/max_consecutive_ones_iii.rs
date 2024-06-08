// 1004. Max Consecutive Ones III
// https://leetcode.com/problems/max-consecutive-ones-iii/

pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        // maintain a sliding, variable size window
        let mut start = 0usize; // beginning index of the current window (inclusive)
        let mut end = start + 1; // end index of the current window (exclusive)
        let mut count_zeroes = if nums[start] == 0 { 1usize } else { 0usize };
        let mut count_ones = if nums[start] == 1 { 1usize } else { 0usize };
        let mut result = if count_zeroes <= k || count_ones == 1 {
            1usize
        } else {
            0usize
        };

        while end < nums.len() {
            if nums[end] == 1 {
                // do we have a valid window right now?
                if count_zeroes <= k {
                    // consume it
                    count_ones += 1;
                    end += 1;
                } else {
                    // slide over
                    count_ones += 1;
                    if nums[start] == 0 {
                        count_zeroes -= 1;
                    } else {
                        count_ones -= 1;
                    }
                    start += 1;
                }
            } else {
                // nums[end] == 0
                // can we convert it?
                if count_zeroes <= k {
                    // convert it
                    count_zeroes += 1;
                    end += 1;
                } else {
                    // slide over
                    if nums[start] == 0 {
                        count_zeroes -= 1;
                    } else {
                        count_ones += 1;
                    }
                    start += 1;
                }
            }

            let window_size = end - start;
            if count_zeroes <= k && window_size > result {
                result = window_size;
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
        let nums = [1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;

        // when
        let result = Solution::longest_ones(nums.into(), k);

        // then
        assert_eq!(result, 6);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;

        // when
        let result = Solution::longest_ones(nums.into(), k);

        // then
        assert_eq!(result, 10);
    }
}
