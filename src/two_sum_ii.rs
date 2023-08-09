// 167. Two Sum II - Input Array Is Sorted
// From: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let x = numbers[i];
            let target = target - x;
            let sub = &numbers[(i + 1)..numbers.len()];
            if let Ok(target_index) = sub.binary_search(&target) {
                return vec![i as i32 + 1i32, target_index as i32 + i as i32 + 2i32];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let numbers = vec![2, 7, 11, 15];
        let target = 9;

        // when
        let result = Solution::two_sum(numbers, target);

        // then
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    pub fn example2() {
        // given
        let numbers = vec![2, 3, 4];
        let target = 6;

        // when
        let result = Solution::two_sum(numbers, target);

        // then
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    pub fn example3() {
        // given
        let numbers = vec![-1, 0];
        let target = -1;

        // when
        let result = Solution::two_sum(numbers, target);

        // then
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    pub fn example20() {
        // given
        let numbers = vec![1, 2, 3, 4, 4, 9, 56, 90];
        let target = 8;

        // when
        let result = Solution::two_sum(numbers, target);

        // then
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    pub fn example21() {
        // given
        let numbers = vec![-1, -1, 1, 1, 1, 1];
        let target = -2;

        // when
        let result = Solution::two_sum(numbers, target);

        // then
        assert_eq!(result, vec![1, 2]);
    }
}