// 692. Top K Frequent Words
// https://leetcode.com/problems/top-k-frequent-words/

pub struct Solution;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let k = k as usize;
        let mut word_frequencies: HashMap<String, usize> = HashMap::with_capacity(words.len());
        let mut frequency_map: BTreeMap<Reverse<usize>, BTreeSet<String>> = BTreeMap::new();
        for word in words {
            let count = word_frequencies
                .entry(word.clone())
                .and_modify(|previous| *previous += 1)
                .or_insert(1usize);
            if *count > 1usize {
                // we've seen this word before
                if let Some(bucket) = frequency_map.get_mut(&Reverse(*count - 1)) {
                    bucket.remove(&word);
                    if bucket.is_empty() {
                        frequency_map.remove(&Reverse(*count - 1));
                    }
                }
            }
            let bucket = frequency_map.entry(Reverse(*count)).or_default();
            bucket.insert(word.clone());
            if frequency_map.len() > k {
                frequency_map.pop_last();
                // TODO can truncate word_frequencies based on "max possible frequency of remaining words"
            }
        }

        let mut result = Vec::with_capacity(k);
        for (_count, bucket) in frequency_map {
            for word in bucket.into_iter().take(k - result.len()) {
                result.push(word);
            }
            if result.len() >= k {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let words = ["i", "love", "leetcode", "i", "love", "coding"]
            .map(ToString::to_string)
            .to_vec();
        let k = 2;

        // when
        let result = Solution::top_k_frequent(words, k);

        // then
        assert_eq!(result, ["i", "love"].map(ToString::to_string).to_vec());
    }

    #[test]
    fn example2() {
        // given
        let words = [
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]
        .map(ToString::to_string)
        .to_vec();
        let k = 4;

        // when
        let result = Solution::top_k_frequent(words, k);

        // then
        assert_eq!(
            result,
            ["the", "is", "sunny", "day"]
                .map(ToString::to_string)
                .to_vec()
        );
    }

    #[test]
    fn example107() {
        // given
        let words = [
            "bhsmote",
            "sgmjynhhz",
            "mrdcroqasn",
            "mrdcroqasn",
            "jmgch",
            "amdmaic",
            "mrdcroqasn",
            "qyepsbmi",
            "mrdcroqasn",
            "sgmjynhhz",
            "qyepsbmi",
            "sgmjynhhz",
            "sytzfmd",
            "vtnhcbx",
            "sytzfmd",
            "czanzqtuwl",
            "amdmaic",
            "qyepsbmi",
            "czanzqtuwl",
            "qyepsbmi",
            "sgmjynhhz",
            "vtnhcbx",
            "sgmjynhhz",
            "tdtrobarr",
            "vtnhcbx",
            "sgmjynhhz",
            "czanzqtuwl",
            "amdmaic",
            "mrdcroqasn",
            "amdmaic",
            "qyepsbmi",
            "jmgch",
            "qyepsbmi",
            "sgmjynhhz",
            "sgmjynhhz",
            "sytzfmd",
            "bdobr",
            "sgmjynhhz",
            "qyepsbmi",
            "bdobr",
            "czanzqtuwl",
            "qyepsbmi",
            "tdtrobarr",
            "qyepsbmi",
            "sytzfmd",
            "tdtrobarr",
            "mrdcroqasn",
        ]
        .map(ToString::to_string)
        .to_vec();
        let k = 3;

        // when
        let result = Solution::top_k_frequent(words, k);

        // then
        assert_eq!(
            result,
            ["qyepsbmi", "sgmjynhhz", "mrdcroqasn"]
                .map(ToString::to_string)
                .to_vec()
        );
    }
}
