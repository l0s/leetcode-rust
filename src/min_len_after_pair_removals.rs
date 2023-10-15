// 2856. Minimum Array Length After Pair Removals
// https://leetcode.com/problems/minimum-array-length-after-pair-removals/

pub struct Solution;

impl Solution {
    ///
    /// Parameters:
    /// - `nums` - sorted in non-decreasing order, may have duplicates, of length between 1 and 10^5
    pub fn min_length_after_removals(nums: Vec<i32>) -> i32 {
        for length in (1..=nums.len()/2).rev() {
            let smallest = &nums[0..length];
            let largest = &nums[nums.len() - length..nums.len()];
            let mut valid = true;
            for i in 0..length {
                if smallest[i] >= largest[i] {
                    valid = false;
                    break;
                }
            }
            if valid {
                return (nums.len() - 2 * length) as i32;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = vec![1,3,4,9];

        // when
        let result = Solution::min_length_after_removals(nums);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = vec![2,3,6,9];

        // when
        let result = Solution::min_length_after_removals(nums);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = vec![1,1,2];

        // when
        let result = Solution::min_length_after_removals(nums);

        // then
        assert_eq!(result, 1);
    }

    #[test]
    pub fn example348() {
        // given
        let nums = vec![1,3,3,3,4];

        // when
        let result = Solution::min_length_after_removals(nums);

        // then
        assert_eq!(result, 1);
    }

    #[test]
    pub fn example516() {
        // given
        let nums = vec![1];

        // when
        let result = Solution::min_length_after_removals(nums);

        // then
        assert_eq!(result, 1);
    }
}