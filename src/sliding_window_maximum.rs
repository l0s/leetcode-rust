// 239. Sliding Window Maximum
// From: https://leetcode.com/problems/sliding-window-maximum/

pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = Vec::new();

        let mut window = Window::new(&nums[0..k]);
        result.push(window.max);
        for i in k..nums.len() {
            result.push(window.slide(nums[i]));
        }

        result
    }
}

struct Window {
    /// items in index order
    items: Vec<i32>,
    /// items in value order
    sorted: BTreeSet<i32>,
    frequencies: HashMap<i32, usize>,
    max: i32,
}

impl Window {
    pub fn new(items: &[i32]) -> Self {
        let mut frequencies = HashMap::with_capacity(items.len());
        let sorted = items.iter().map(|item| *item).collect();
        for item in items {
            frequencies.entry(*item)
                .and_modify(|count| *count += 1)
                .or_insert(1usize);
        }
        Self {
            items: items.to_vec(),
            frequencies,
            sorted,
            max: *items.iter().max().unwrap(),
        }
    }

    pub fn slide(&mut self, new_item: i32) -> i32 {
        let removed = self.items.remove(0);

        let count = self.frequencies.get_mut(&removed).unwrap();
        self.items.push(new_item);
        self.sorted.insert(new_item);
        if *count == 1 { // last instance
            self.sorted.remove(&removed);
            if removed == self.max {
                // self.max = *self.sorted.last().unwrap();

                // self.max = self.sorted.pop_last().unwrap();
                // self.sorted.insert(self.max);

                self.max = *self.sorted.iter().rev().next().unwrap();
            }
        }
        if new_item > self.max {
            self.max = new_item;
        }
        *count -= 1;

        self.frequencies.entry(new_item)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        self.max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;

        // when
        let result = Solution::max_sliding_window(nums, k);

        // then
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn example2() {
        // given
        let nums = vec![1];
        let k = 1;

        // when
        let result = Solution::max_sliding_window(nums, k);

        // then
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn example49() {
        // given
        let nums = vec![9, 10, 9, -7, -4, -8, 2, -6];
        let k = 5;

        // when
        let result = Solution::max_sliding_window(nums, k);

        // then
        assert_eq!(result, vec![10, 10 , 9, 2]);
    }
}