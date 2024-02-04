// 334. Increasing Triplet Subsequence
// https://leetcode.com/problems/increasing-triplet-subsequence

pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut i = i32::MAX;
        let mut j = i32::MAX;
        for k in nums {
            if k <= i {
                i = k;
            } else if k <= j
            /* && k > i */
            {
                j = k; // now j > i
            } else
            /* k > i && k > j && j > i */
            {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [1, 2, 3, 4, 5];

        // when
        let result = Solution::increasing_triplet(nums.into());

        // then
        assert!(result);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [5, 4, 3, 2, 1];

        // when
        let result = Solution::increasing_triplet(nums.into());

        // then
        assert!(!result);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = [2, 1, 5, 0, 4, 6];

        // when
        let result = Solution::increasing_triplet(nums.into());

        // then
        assert!(result);
    }

    #[test]
    pub fn example8() {
        // given
        let nums = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];

        // when
        let result = Solution::increasing_triplet(nums.into());

        // then
        assert!(!result);
    }

    #[test]
    pub fn example64() {
        // given
        let nums = [1, 2, 1, 3];

        // when
        let result = Solution::increasing_triplet(nums.into());

        // then
        assert!(result);
    }
}
