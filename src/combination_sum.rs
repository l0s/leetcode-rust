// 39. Combination Sum
// https://leetcode.com/problems/combination-sum

pub struct Solution;

use std::collections::Bound::Included;
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let candidates = BTreeSet::from_iter(candidates);
        solutions(&candidates, 1, target)
            .into_iter()
            .map(Vec::from_iter)
            .collect()
    }
}

fn solutions(
    candidates: &BTreeSet<i32>,
    smallest_candidate: i32,
    target: i32,
) -> HashSet<Vec<i32>> {
    if target <= 0 {
        return HashSet::new();
    }
    let mut result = HashSet::new();
    for candidate in candidates.range((Included(smallest_candidate), Included(target))) {
        let adjusted_target = target - *candidate;
        if adjusted_target == 0 {
            result.insert(vec![*candidate]);
            continue;
        }
        for remaining in solutions(
            candidates,
            *candidate.min(&adjusted_target),
            adjusted_target,
        ) {
            let mut solution = remaining;
            solution.push(*candidate);
            solution.sort();
            result.insert(solution);
        }
    }
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
        let candidates = [2, 3, 6, 7];
        let target = 7;

        // when
        let result = Solution::combination_sum(candidates.to_vec(), target);

        // then
        assert_eq!(
            HashSet::from_iter(result.into_iter()),
            HashSet::from([vec![2, 2, 3], vec![7]])
        );
    }

    #[test]
    fn example2() {
        // given
        let candidates = [2, 3, 5];
        let target = 8;

        // when
        let result = Solution::combination_sum(candidates.to_vec(), target);

        // then
        assert_eq!(
            HashSet::from_iter(result.into_iter()),
            HashSet::from([vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]])
        );
    }

    #[test]
    fn example3() {
        // given
        let candidates = [2];
        let target = 1;

        // when
        let result = Solution::combination_sum(candidates.to_vec(), target);

        // then
        assert!(result.is_empty());
    }
}
