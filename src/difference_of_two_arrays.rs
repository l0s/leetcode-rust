// 2215. Find the Difference of Two Arrays
// https://leetcode.com/problems/find-the-difference-of-two-arrays

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let nums1 = nums1.into_iter().collect::<HashSet<i32>>();
        let nums2 = nums2.into_iter().collect::<HashSet<i32>>();

        let unique_to_nums1 = nums1.difference(&nums2).cloned().collect::<Vec<i32>>();
        let unique_to_nums2 = nums2.difference(&nums1).cloned().collect::<Vec<i32>>();

        vec![unique_to_nums1, unique_to_nums2]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums1 = [1, 2, 3];
        let nums2 = [2, 4, 6];

        // when
        let mut result = Solution::find_difference(nums1.into(), nums2.into());

        // then
        result[0].sort();
        result[1].sort();
        assert_eq!(result, vec![vec![1, 3], vec![4, 6]]);
    }

    #[test]
    pub fn example2() {
        // given
        let nums1 = [1, 2, 3, 3];
        let nums2 = [1, 1, 2, 2];

        // when
        let mut result = Solution::find_difference(nums1.into(), nums2.into());

        // then
        result[0].sort();
        result[1].sort();
        assert_eq!(result, vec![vec![3], vec![]]);
    }
}
