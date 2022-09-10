// 870. Advantage Shuffle
// From: https://leetcode.com/problems/advantage-shuffle/

pub struct Solution;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let max_possible_advantage = nums1.len();
        let mut max_advantage = advantage(&nums1, &nums2);
        let mut most_advantageous_permutation = nums1.clone();

        for permutation in permutations(&nums1) {
            let advantage = advantage(&permutation, &nums2);
            if advantage > max_advantage {
                max_advantage = advantage;
                most_advantageous_permutation = permutation;
                if max_advantage == max_possible_advantage {
                    break;
                }
            }
        }

        most_advantageous_permutation
    }
}

fn permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    if nums.is_empty() {
        return vec![];
    } else if nums.len() == 1 {
        return vec![vec![nums[0]]];
    }
    let mut result = vec![];
    for i in 0..nums.len() {
        let first = nums[i];
        let remaining = [&nums[..i], &nums[(i + 1)..]].concat();
        for sub_permutation in permutations(&remaining) {
            result.push(vec![vec![first], sub_permutation].concat());
        }
    }
    result
}

fn advantage(nums1: &[i32], nums2: &[i32]) -> usize {
    let mut result = 0;
    for i in 0..nums1.len() {
        if nums1[i] > nums2[i] {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums1 = vec![2, 7, 11, 15];
        let nums2 = vec![1, 10, 4, 11];

        // when
        let result = Solution::advantage_count(nums1, nums2);

        // then
        assert_eq!(result, vec![2, 11, 7, 15]);
    }

    #[test]
    pub fn example2() {
        // given
        let nums1 = vec![12, 24, 8, 32];
        let nums2 = vec![13, 25, 32, 11];

        // when
        let result = Solution::advantage_count(nums1, nums2);

        // then
        assert_eq!(result, vec![24, 32, 8, 12]);
    }
}
