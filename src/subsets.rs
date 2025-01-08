// 78. Subsets
// https://leetcode.com/problems/subsets

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for len in 0..=nums.len() {
            result = [result, subsets_of_size(&nums, len)].concat();
        }
        result
    }
}

fn subsets_of_size(nums: &[i32], len: usize) -> Vec<Vec<i32>> {
    if len == 0 {
        return vec![vec![]];
    } else if len == nums.len() {
        return vec![nums.to_vec()];
    } else if len == 1 {
        return nums.iter().map(|item| vec![*item]).collect();
    }

    // 1 < len < nums.len()
    let mut result = vec![];
    // take the first item
    let first = nums[0];
    result.append(
        &mut subsets_of_size(&nums[1..], len - 1)
            .into_iter()
            .map(|mut subset| -> Vec<i32> {
                subset.insert(0, first);
                subset
            })
            .collect::<Vec<Vec<i32>>>(),
    );

    // skip the first item
    result.append(&mut subsets_of_size(&nums[1..], len));

    result
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn example1() {
        // given
        let nums = [1, 2, 3];

        // when
        let result = Solution::subsets(nums.to_vec());

        // then
        let result = HashSet::from_iter(result.into_iter());
        let expected = HashSet::from([
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        // given
        let nums = [0];

        // when
        let result = Solution::subsets(nums.to_vec());

        // then
        assert_eq!(result, vec![vec![], vec![0]]);
    }
}
