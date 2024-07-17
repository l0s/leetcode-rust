// 57. Insert Interval
// https://leetcode.com/problems/insert-interval

pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        } else if intervals.len() == 1 {
            return match new_interval[0].cmp(&intervals[0][0]) {
                Ordering::Less => {
                    if Self::overlaps(&new_interval, &intervals[0]) {
                        vec![Self::merge(&new_interval, &intervals[0])]
                    } else {
                        vec![new_interval, intervals[0].clone()]
                    }
                }
                Ordering::Greater => {
                    if Self::overlaps(&intervals[0], &new_interval) {
                        vec![Self::merge(&intervals[0], &new_interval)]
                    } else {
                        vec![intervals[0].clone(), new_interval]
                    }
                }
                Ordering::Equal => {
                    // same left index
                    vec![Self::merge(&new_interval, &intervals[0])]
                }
            };
        } else if new_interval[0] <= intervals[0][0]
            && new_interval[1] >= intervals[intervals.len() - 1][1]
        {
            // new interval overlaps all intervals
            return vec![new_interval];
        }

        let (mut new_list, mut index) =
            match intervals.binary_search_by(|probe| probe[0].cmp(&new_interval[0])) {
                Ok(index) => {
                    let prefix = &intervals[0..index];
                    let suffix = &intervals[index + 1..intervals.len()];
                    let existing = &intervals[index];
                    let new_item = vec![existing[0], existing[1].max(new_interval[1])];
                    let new_list = [prefix, &[new_item], suffix].concat();
                    (new_list, index)
                }
                Err(index) => {
                    let prefix = &intervals[0..index];
                    let suffix = &intervals[index..intervals.len()];
                    let new_list = [prefix, &[new_interval], suffix].concat();
                    (new_list, index)
                }
            };
        // merge with item on left
        while index > 0 && Self::overlaps(&new_list[index - 1], &new_list[index]) {
            let merged = Self::merge(&new_list[index - 1], &new_list[index]);
            new_list[index - 1] = merged;
            new_list.remove(index);
            index -= 1;
        }
        // merge with item on right
        while index < new_list.len() - 1 && Self::overlaps(&new_list[index], &new_list[index + 1]) {
            let merged = Self::merge(&new_list[index], &new_list[index + 1]);
            new_list[index] = merged;
            new_list.remove(index + 1);
        }
        new_list
    }

    fn overlaps(left: &[i32], right: &[i32]) -> bool {
        right[0] <= left[1]
    }

    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        vec![left[0].min(right[0]), left[1].max(right[1])]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];

        // when
        let result = Solution::insert(intervals, new_interval);

        // then
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]])
    }

    #[test]
    pub fn example2() {
        // given
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];

        // when
        let result = Solution::insert(intervals, new_interval);

        // then
        assert_eq!(result, vec![vec![1, 2], vec![3, 10], vec![12, 16]])
    }

    #[test]
    pub fn example3() {
        // given
        let intervals = vec![];
        let new_interval = vec![5, 7];

        // when
        let result = Solution::insert(intervals, new_interval);

        // then
        assert_eq!(result, vec![vec![5, 7]])
    }

    #[test]
    pub fn example4() {
        // given
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 3];

        // when
        let result = Solution::insert(intervals, new_interval);

        // then
        assert_eq!(result, vec![vec![1, 5]])
    }

    #[test]
    pub fn example14() {
        // given
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![0, 0];

        // when
        let result = Solution::insert(intervals, new_interval);

        // then
        assert_eq!(result, vec![vec![0, 0], vec![1, 5]])
    }

    #[test]
    pub fn example15() {
        // given
        let intervals = vec![vec![1, 5], vec![6, 8]];
        let new_interval = vec![5, 6];

        // when
        let result = Solution::insert(intervals, new_interval);

        // then
        assert_eq!(result, vec![vec![1, 8]])
    }
}
