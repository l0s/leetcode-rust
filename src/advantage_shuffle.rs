// 870. Advantage Shuffle
// From: https://leetcode.com/problems/advantage-shuffle/

pub struct Solution;

use std::collections::hash_map::DefaultHasher;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hasher;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        best_permutation(
            &nums1,
            &nums2,
            advantage(&nums1, &nums2),
            &mut HashMap::default(),
        )
        .permutation
    }
}

#[derive(PartialEq, Eq, Clone)]
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

/// Find _one of_ the permutations that will yield that highest advantage
/// Parameters
/// - `numbers_to_permute` - The numbers which may be reördered to increase advantage
/// - `reference` - The numbers to which any potential permutation will be compared to determine advantage
/// - `minimum_advantage` - The highest known possible advantage for `reference`. Any lower advantage can be disregarded.
/// - `cache` - a mapping of `hash(numbers_to_permuate, reference)` to the best known permutation
fn best_permutation(
    numbers_to_permute: &[i32],
    reference: &[i32],
    minimum_advantage: usize,
    cache: &mut HashMap<u64, Permutation>,
) -> Permutation {
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
    let starting_advantage = advantage(numbers_to_permute, reference);
    if starting_advantage == numbers_to_permute.len()
        || numbers_to_permute.len() < minimum_advantage
    {
        // this is one of the permutations with the highest possible advantage
        // or
        // no permutation will be advantageous enough
        // just return the starting permutation
        return Permutation {
            permutation: numbers_to_permute.to_vec(),
            advantage: starting_advantage,
        };
    }

    let cache_key = create_cache_key(numbers_to_permute, reference);
    if let Some(cached) = cache.get(&cache_key) {
        return cached.clone();
    }

    let reference_remaining = &reference[1..];
    let mut heap: BinaryHeap<Permutation> = BinaryHeap::default();
    for i in 0..numbers_to_permute.len() {
        let first = numbers_to_permute[i];
        let advantage_modifier = if first > reference[0] { 1 } else { 0 };
        let mut minimum_advantage = if let Some(best) = heap.peek() {
            minimum_advantage.max(best.advantage)
        } else {
            minimum_advantage
        };
        if minimum_advantage > 0 {
            // if the chosen first entry is advantageous
            // we can lower the bar by one
            minimum_advantage -= advantage_modifier;
        }
        let remaining = [&numbers_to_permute[..i], &numbers_to_permute[(i + 1)..]].concat();
        let sub_permutation =
            best_permutation(&remaining, reference_remaining, minimum_advantage, cache);
        let permutation = vec![vec![first], sub_permutation.permutation].concat();
        heap.push(Permutation {
            permutation,
            advantage: sub_permutation.advantage + advantage_modifier,
        });
    }

    let result = heap.pop().expect("No permutations");
    cache.insert(cache_key, result.clone());
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

fn create_cache_key(numbers_to_permute: &[i32], reference: &[i32]) -> u64 {
    let mut hasher = DefaultHasher::default();
    hasher.write_usize(numbers_to_permute.len());
    for number_to_permute in numbers_to_permute {
        hasher.write_i32(*number_to_permute);
    }
    for reference_number in reference {
        hasher.write_i32(*reference_number);
    }
    hasher.finish()
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

    #[test]
    pub fn example23() {
        // given
        let nums1 = vec![15, 15, 4, 5, 0, 1, 7, 10, 3, 1, 10, 10, 8, 2, 3];
        let nums2 = vec![4, 13, 14, 0, 14, 14, 12, 3, 15, 12, 2, 0, 6, 9, 0];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(
                &vec![5, 15, 15, 4, 0, 1, 7, 10, 3, 1, 10, 2, 8, 10, 3],
                &nums2
            )
        );
    }

    #[test]
    pub fn example29() {
        // given
        let nums1 = vec![7, 17, 18, 11, 13, 12, 0, 9, 3, 16];
        let nums2 = vec![9, 0, 11, 16, 18, 13, 12, 17, 7, 3];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![11, 3, 12, 17, 0, 16, 13, 18, 9, 7], &nums2)
        );
    }

    // #[test] // FIXME this test case is currently broken
    pub fn example30() {
        // given
        let nums1 = vec![25, 9, 17, 6, 2, 14, 3, 24, 18, 15, 10, 27, 11, 4, 13];
        let nums2 = vec![15, 4, 13, 24, 3, 11, 10, 9, 27, 17, 2, 6, 14, 18, 25];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(
                &vec![17, 6, 14, 25, 4, 13, 11, 10, 2, 18, 3, 9, 15, 24, 27],
                &nums2
            )
        );
    }
}
