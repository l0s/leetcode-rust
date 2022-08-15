/*
940. Distinct Subsequences II
From: https://leetcode.com/problems/distinct-subsequences-ii/
*/

pub struct Solution;

use std::collections::{HashMap, HashSet};
const C: usize = 10usize.pow(9) + 7;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        let mut cache = HashMap::default();
        let result = distinct_subsequences(&s, &mut cache);
        (result.len() % C) as i32
    }
}

fn distinct_subsequences(
    sequence: &str,
    cache: &mut HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    if sequence.is_empty() {
        return HashSet::default();
    } else if let Some(cached_result) = cache.get(sequence) {
        return cached_result.clone();
    }
    let mut set = HashSet::default();
    for index_to_delete in 0..sequence.len() {
        let prefix = &sequence[0..index_to_delete];
        let suffix = &sequence[(index_to_delete + 1)..sequence.len()];
        let subsequence = [prefix, suffix].concat();
        for subsequence in distinct_subsequences(&subsequence, cache) {
            set.insert(subsequence);
        }
    }
    set.insert(sequence.to_owned());
    cache.insert(sequence.to_owned(), set.clone());
    set
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "abc";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 7);
    }

    #[test]
    fn example2() {
        // given
        let s = "aba";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn example3() {
        // given
        let s = "aaa";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn example40() {
        // given
        let s = "glqkeogktv";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 983);
    }
}
