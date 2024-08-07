// 46. Permutations
// https://leetcode.com/problems/permutations

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        permutations(&nums)
    }
}

// "1 <= nums.length <= 6"
const FACTORIALS: [usize; 7] = [1, 1, 2, 6, 24, 120, 720];

fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    if nums.is_empty() || nums.len() == 1 {
        return vec![nums.to_vec()];
    }
    let mut result = Vec::with_capacity(FACTORIALS[nums.len()]);
    for (i, first_entry) in nums.iter().enumerate() {
        let left = &nums[0..i];
        let right = &nums[i + 1..];
        for remaining in permutations(&[left, right].concat()) {
            result.push([vec![*first_entry], remaining].concat());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let nums = [1, 2, 3];

        // when
        let result = Solution::permute(nums.to_vec());

        // then
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn example2() {
        // given
        let nums = [0, 1];

        // when
        let result = Solution::permute(nums.to_vec());

        // then
        assert_eq!(result, vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn example3() {
        // given
        let nums = [1];

        // when
        let result = Solution::permute(nums.to_vec());

        // then
        assert_eq!(result, vec![vec![1]]);
    }
}
