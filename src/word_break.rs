// 139. Word Break
// https://leetcode.com/problems/word-break

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dictionary = Trie::default();
        for word in word_dict {
            dictionary.insert(&word);
        }

        let string = s.chars().collect::<Vec<char>>();
        let mut cache = HashMap::new();
        word_break(&string, &dictionary, &mut cache)
    }
}

fn word_break<'a>(
    string: &'a [char],
    dictionary: &Trie,
    cache: &mut HashMap<&'a [char], bool>,
) -> bool {
    if let Some(cached_result) = cache.get(string) {
        return *cached_result;
    }
    let mut node = dictionary;
    for (i, c) in string.iter().enumerate() {
        if let Some(t) = node.children.get(c) {
            if t.terminates && word_break(&string[i + 1..], dictionary, cache) {
                cache.insert(string, true);
                return true;
            }
            node = t;
        } else {
            cache.insert(string, false);
            return false;
        }
    }
    let result = node.terminates;
    cache.insert(string, result);
    result
}

pub struct Trie {
    terminates: bool,
    children: HashMap<char, Trie>,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            terminates: false,
            children: HashMap::with_capacity(26), // "s and word_dict[i] consist of only lowercase English letters"
        }
    }
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut chars = word.chars();
        let first_char = chars.next().expect("Word must have at least one character");
        let mut node = self.children.entry(first_char).or_default();
        for subsequent in chars {
            node = node.children.entry(subsequent).or_default();
        }
        node.terminates = true;
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "leetcode";
        let word_dict = ["leet", "code"];

        // when
        let result = Solution::word_break(
            s.to_string(),
            word_dict.map(|slice| slice.to_string()).to_vec(),
        );

        // then
        assert!(result);
    }

    #[test]
    fn example2() {
        // given
        let s = "applepenapple";
        let word_dict = ["apple", "pen"];

        // when
        let result = Solution::word_break(
            s.to_string(),
            word_dict.map(|slice| slice.to_string()).to_vec(),
        );

        // then
        assert!(result);
    }

    #[test]
    fn example3() {
        // given
        let s = "catsandog";
        let word_dict = ["cats", "dog", "sand", "and", "cat"];

        // when
        let result = Solution::word_break(
            s.to_string(),
            word_dict.map(|slice| slice.to_string()).to_vec(),
        );

        // then
        assert!(!result);
    }

    #[test]
    fn example36() {
        // given
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab";
        let word_dict = [
            "a",
            "aa",
            "aaa",
            "aaaa",
            "aaaaa",
            "aaaaaa",
            "aaaaaaa",
            "aaaaaaaa",
            "aaaaaaaaa",
            "aaaaaaaaaa",
        ];

        // when
        let result = Solution::word_break(
            s.to_string(),
            word_dict.map(|slice| slice.to_string()).to_vec(),
        );

        // then
        assert!(!result);
    }
}
