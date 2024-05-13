// 1679. Max Number of K-Sum Pairs
// https://leetcode.com/problems/max-number-of-k-sum-pairs

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut numbers = nums
            .iter()
            .filter(|value| **value < k)
            .copied()
            .collect::<Vec<i32>>();
        let mut operations = 0;
        while let Some(number) = numbers.pop() {
            if let Some(index) =
                numbers
                    .iter()
                    .enumerate()
                    .find_map(|(index, value)| -> Option<usize> {
                        if number + *value == k {
                            Some(index)
                        } else {
                            None
                        }
                    })
            {
                operations += 1;
                numbers.remove(index);
            }
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [1, 2, 3, 4];
        let k = 5;

        // when
        let result = Solution::max_operations(nums.to_vec(), k);

        // then
        assert_eq!(result, 2);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [3, 1, 3, 4, 3];
        let k = 6;

        // when
        let result = Solution::max_operations(nums.to_vec(), k);

        // then
        assert_eq!(result, 1);
    }
}
