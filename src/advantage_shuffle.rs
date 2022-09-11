// 870. Advantage Shuffle
// From: https://leetcode.com/problems/advantage-shuffle/

pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        best_permutation(&nums1, &nums2).permutation
    }
}

#[derive(PartialEq, Eq)]
struct Permutation {
    permutation: Vec<i32>,
    advantage: usize,
}

impl Ord for Permutation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.advantage.cmp(&other.advantage)
    }
}

impl PartialOrd for Permutation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.advantage.partial_cmp(&other.advantage)
    }
}

fn best_permutation(numbers_to_permute: &[i32], reference: &[i32]) -> Permutation {
    if numbers_to_permute.is_empty() {
        return Permutation {
            permutation: vec![],
            advantage: 0,
        };
    } else if numbers_to_permute.len() == 1 {
        return Permutation {
            permutation: vec![numbers_to_permute[0]],
            advantage: if numbers_to_permute[0] > reference[0] {
                1
            } else {
                0
            },
        };
    } else if advantage(numbers_to_permute, reference) == numbers_to_permute.len() {
        // this is one of the permutations with the highest possible advantage
        return Permutation {
            permutation: numbers_to_permute.to_vec(),
            advantage: numbers_to_permute.len(),
        };
    } else if numbers_to_permute
        .iter()
        .max()
        .expect("Non empty slice should have a maximum value")
        < reference
            .iter()
            .min()
            .expect("Non empty slice should have a minimum value")
    {
        // all possible permutations have a zero advantage
        return Permutation {
            permutation: numbers_to_permute.to_vec(),
            advantage: 0,
        };
    }

    let reference_remaining = &reference[1..];
    let mut heap = BinaryHeap::default();
    for i in 0..numbers_to_permute.len() {
        let first = numbers_to_permute[i];
        let advantage_modifier = if first > reference[0] { 1 } else { 0 };
        let remaining = [&numbers_to_permute[..i], &numbers_to_permute[(i + 1)..]].concat();
        let sub_permutation = best_permutation(&remaining, reference_remaining);
        let permutation = vec![vec![first], sub_permutation.permutation].concat();
        heap.push(Permutation {
            permutation,
            advantage: sub_permutation.advantage + advantage_modifier,
        });
    }

    heap.pop().expect("No permutations")
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
    use crate::advantage_shuffle::advantage;

    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums1 = vec![2, 7, 11, 15];
        let nums2 = vec![1, 10, 4, 11];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![2, 11, 7, 15], &nums2)
        );
    }

    #[test]
    pub fn example2() {
        // given
        let nums1 = vec![12, 24, 8, 32];
        let nums2 = vec![13, 25, 32, 11];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![24, 32, 8, 12], &nums2)
        );
    }

    #[test]
    pub fn example14() {
        // given
        let nums1 = vec![8, 2, 4, 4, 5, 6, 6, 0, 4, 7];
        let nums2 = vec![0, 8, 7, 4, 4, 2, 8, 5, 2, 0];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![2, 4, 8, 5, 6, 4, 0, 6, 4, 7], &nums2)
        );
    }

    #[test]
    pub fn example16() {
        // given
        let nums1 = vec![
            817556235, 232246148, 424291784, 13856114, 113666578, 513996259, 791310549, 341026786,
            319348491, 40812144,
        ];
        let nums2 = vec![
            99616046, 148514886, 206257807, 569848817, 358893433, 977727091, 284109958, 856279522,
            91774240, 527604738,
        ];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(
                &vec![
                    232246148, 424291784, 341026786, 817556235, 513996259, 13856114, 319348491,
                    40812144, 113666578, 791310549
                ],
                &nums2
            )
        );
    }
}
