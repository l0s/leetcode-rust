// 1207. Unique Number of Occurrences
// https://leetcode.com/problems/unique-number-of-occurrences/description/

pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // 1 <= arr.length <= 1000
        let mut occurrences = HashMap::with_capacity(arr.len());

        for item in arr {
            occurrences
                .entry(item)
                .and_modify(|counter| *counter += 1u16)
                .or_insert(1u16);
        }

        let mut counts = HashSet::with_capacity(occurrences.len());
        for count in occurrences.values() {
            if counts.contains(count) {
                return false;
            }
            counts.insert(*count);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let arr = [1, 2, 2, 1, 1, 3];

        // when
        let result = Solution::unique_occurrences(arr.into());

        // then
        assert!(result);
    }

    #[test]
    pub fn example2() {
        // given
        let arr = [1, 2];

        // when
        let result = Solution::unique_occurrences(arr.into());

        // then
        assert!(!result);
    }

    #[test]
    pub fn example3() {
        // given
        let arr = [-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];

        // when
        let result = Solution::unique_occurrences(arr.into());

        // then
        assert!(result);
    }
}
