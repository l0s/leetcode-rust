// 950. Reveal Cards In Increasing Order
// From: https://leetcode.com/problems/reveal-cards-in-increasing-order/

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        if deck.is_empty() || deck.len() == 1 {
            return deck;
        }
        let mut sorted = deck.clone();
        sorted.sort(); // O( n lg n )
        let mut sorted: VecDeque<i32> = sorted.into();

        let mut result = VecDeque::with_capacity(deck.len());

        while let Some(largest) = sorted.pop_back() {
            if let Some(last) = result.pop_back() {
                result.push_front(last);
            }
            result.push_front(largest);
        }

        result.into()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let deck = vec![17, 13, 11, 2, 3, 5, 7];

        // when
        let result = Solution::deck_revealed_increasing(deck);

        // then
        assert_eq!(result, vec![2, 13, 3, 11, 5, 17, 7]);
    }

    #[test]
    pub fn example2() {
        // given
        let deck = vec![1, 1000];

        // when
        let result = Solution::deck_revealed_increasing(deck);

        // then
        assert_eq!(result, vec![1, 1000]);
    }
}
