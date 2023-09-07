// 416. Partition Equal Subset Sum
// https://leetcode.com/problems/partition-equal-subset-sum/

pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter()
            .to_owned()
            .sum::<i32>() as usize; // use usize because we will use it to index into a vector
        if sum % 2 != 0 {
            return false;
        }
        let sum = sum / 2;
        let mut possible = vec![vec![false; sum + 1]; nums.len() + 1];
        for i in 0..=nums.len() {
            for target_sum in 0..=sum {
                if i == 0 || target_sum == 0 {
                    possible[i][target_sum] = false;
                } else if nums[i - 1] as usize > target_sum {
                    possible[i][target_sum] = possible[i - 1][target_sum];
                } else if nums[i - 1] as usize == target_sum {
                    // exact match
                    possible[i][target_sum] = true;
                } else {
                    possible[i][target_sum] =
                        possible[i - 1][target_sum]
                        || possible[i - 1][target_sum - nums[i - 1] as usize];
                }
            }
        }
        possible[nums.len()][sum]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = vec![1, 5, 11, 5];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(result);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = vec![1, 2, 3, 5];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(!result);
    }

    #[test]
    pub fn example25() {
        // given
        let nums = vec![3, 3, 3, 4, 5];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(result);
    }

    #[test]
    pub fn example36() {
        // given
        let nums = vec![1, 1];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(result);
    }

    #[test]
    pub fn example37() {
        // given
        let nums = vec![1, 1, 1, 1];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(result);
    }

    #[test]
    // #[ignore]
    pub fn example38() {
        // given
        let nums = vec![4, 4, 4, 4, 4, 4, 4, 4, 8, 8, 8, 8, 8, 8, 8, 8, 12, 12, 12, 12, 12, 12, 12, 12, 16, 16, 16, 16, 16, 16, 16, 16, 20, 20, 20, 20, 20, 20, 20, 20, 24, 24, 24, 24, 24, 24, 24, 24, 28, 28, 28, 28, 28, 28, 28, 28, 32, 32, 32, 32, 32, 32, 32, 32, 36, 36, 36, 36, 36, 36, 36, 36, 40, 40, 40, 40, 40, 40, 40, 40, 44, 44, 44, 44, 44, 44, 44, 44, 48, 48, 48, 48, 48, 48, 48, 48, 52, 52, 52, 52, 52, 52, 52, 52, 56, 56, 56, 56, 56, 56, 56, 56, 60, 60, 60, 60, 60, 60, 60, 60, 64, 64, 64, 64, 64, 64, 64, 64, 68, 68, 68, 68, 68, 68, 68, 68, 72, 72, 72, 72, 72, 72, 72, 72, 76, 76, 76, 76, 76, 76, 76, 76, 80, 80, 80, 80, 80, 80, 80, 80, 84, 84, 84, 84, 84, 84, 84, 84, 88, 88, 88, 88, 88, 88, 88, 88, 92, 92, 92, 92, 92, 92, 92, 92, 96, 96, 96, 96, 96, 96, 96, 96, 97, 99];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(!result);
    }

    #[test]
    pub fn example39() {
        // given
        let nums = vec![2, 2, 1, 1];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(result);
    }

    #[test]
    pub fn example40() {
        // given
        let nums = vec![1, 1, 2, 5, 5, 5, 5];

        // when
        let result = Solution::can_partition(nums);

        // then
        assert!(result);
    }
}