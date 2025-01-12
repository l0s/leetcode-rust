// 739. Daily Temperatures
// https://leetcode.com/problems/daily-temperatures/

pub struct Solution;

use std::cmp::Ordering;
use std::collections::VecDeque;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut queue: VecDeque<Record> = VecDeque::with_capacity(temperatures.len());
        let mut result = vec![0; temperatures.len()];

        for (day, temperature) in temperatures.into_iter().enumerate().rev() {
            let record = Record { day, temperature };
            while !queue.is_empty() && temperature >= queue.back().unwrap().temperature {
                queue.pop_back();
            }
            if let Some(last) = queue.back() {
                result[day] = (last.day - day) as i32;
            }
            queue.push_back(record);
        }

        result
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Record {
    day: usize,
    temperature: i32,
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.day
            .cmp(&other.day)
            .then_with(|| self.temperature.cmp(&other.temperature))
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let temperatures = [73, 74, 75, 71, 69, 72, 76, 73];

        // when
        let result = Solution::daily_temperatures(temperatures.to_vec());

        // then
        assert_eq!(&result, &[1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn example2() {
        // given
        let temperatures = [30, 40, 50, 60];

        // when
        let result = Solution::daily_temperatures(temperatures.to_vec());

        // then
        assert_eq!(&result, &[1, 1, 1, 0]);
    }

    #[test]
    fn example3() {
        // given
        let temperatures = [30, 60, 90];

        // when
        let result = Solution::daily_temperatures(temperatures.to_vec());

        // then
        assert_eq!(&result, &[1, 1, 0]);
    }
}
