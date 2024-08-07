// 56. Merge Intervals
// https://leetcode.com/problems/merge-intervals

pub struct Solution;

use std::collections::VecDeque;
use std::ops::RangeInclusive;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() || intervals.len() == 1 {
            return intervals;
        }
        let mut ranges = intervals
            .into_iter()
            .map(|interval| interval[0]..=interval[1])
            .collect::<Vec<RangeInclusive<i32>>>();
        ranges.sort_unstable_by(|x, y| x.start().cmp(y.start()).then(x.end().cmp(y.end())));
        let mut ranges = ranges
            .into_iter()
            .collect::<VecDeque<RangeInclusive<i32>>>();
        let mut merged_ranges = Vec::with_capacity(ranges.len());
        if let Some(mut current) = ranges.pop_front() {
            while let Some(next) = ranges.pop_front() {
                if overlaps(&current, &next) {
                    current = merge(&current, &next);
                } else {
                    merged_ranges.push(current.clone());
                    current = next;
                }
            }
            merged_ranges.push(current);
        }
        merged_ranges
            .into_iter()
            .map(|range| vec![*range.start(), *range.end()])
            .collect()
    }
}

fn overlaps(x: &RangeInclusive<i32>, y: &RangeInclusive<i32>) -> bool {
    x.end() >= y.start()
}

fn merge(x: &RangeInclusive<i32>, y: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    *x.start()..=*(y.end().max(x.end()))
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let intervals = [[1, 3], [2, 6], [8, 10], [15, 18]];

        // when
        let result = Solution::merge(intervals.map(|interval| interval.to_vec()).to_vec());

        // then
        assert_eq!(result, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }

    #[test]
    fn example2() {
        // given
        let intervals = [[1, 4], [4, 5]];

        // when
        let result = Solution::merge(intervals.map(|interval| interval.to_vec()).to_vec());

        // then
        assert_eq!(result, vec![vec![1, 5]]);
    }

    #[test]
    fn example11() {
        // given
        let intervals = [[1, 4], [0, 4]];

        // when
        let result = Solution::merge(intervals.map(|interval| interval.to_vec()).to_vec());

        // then
        assert_eq!(result, vec![vec![0, 4]]);
    }

    #[test]
    fn example118() {
        // given
        let intervals = [[1,4],[2,3]];

        // when
        let result = Solution::merge(intervals.map(|interval| interval.to_vec()).to_vec());

        // then
        assert_eq!(result, vec![vec![1, 4]]);
    }
}
