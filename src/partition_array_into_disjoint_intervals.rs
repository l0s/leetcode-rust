// 915. Partition Array into Disjoint Intervals
// From: https://leetcode.com/problems/partition-array-into-disjoint-intervals/

pub struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut prefix_max = Vec::with_capacity(nums.len());
        let mut suffix_min = vec![i32::MAX; nums.len()];
        let mut running_max = i32::MIN;
        let mut running_min = i32::MAX;

        for i in (0..nums.len()).rev() {
            running_min = running_min.min(nums[i]);
            suffix_min[i] = running_min;
        }
        for num in &nums {
            running_max = running_max.max(*num);
            prefix_max.push(running_max);
        }
        for i in 1..nums.len() {
            if prefix_max[i - 1] <= suffix_min[i] {
                return i as i32;
            }
        }

        panic!("Input has no solution")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = vec![5, 0, 3, 8, 6];

        // when
        let result = Solution::partition_disjoint(nums);

        // then
        assert_eq!(result, 3);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = vec![1, 1, 1, 0, 6, 12];

        // when
        let result = Solution::partition_disjoint(nums);

        // then
        assert_eq!(result, 4);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = vec![1, 1];

        // when
        let result = Solution::partition_disjoint(nums);

        // then
        assert_eq!(result, 1);
    }
}
