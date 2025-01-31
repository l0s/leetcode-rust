// 269. Alien Dictionary
// https://leetcode.com/problems/alien-dictionary/

pub struct Solution;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        if words.is_empty() {
            return "".to_string();
        } else if words.len() == 1 {
            return String::from_iter(words[0].chars()
                .collect::<BTreeSet<char>>()
                .iter());
        }
        // map preceding letters
        let mut letters = HashMap::new();
        for (i, preceding) in words[..(words.len() - 1)].iter().enumerate() {
            for following in words[(i + 1)..].iter() {
                let mut preceding_chars = preceding.chars();
                let mut following_chars = following.chars();
                let mut captured_precedence = false;
                loop {
                    match (preceding_chars.next(), following_chars.next()) {
                        (Some(preceding), Some(following)) => {
                            letters.entry(preceding).or_insert_with(|| Letter {
                                character: preceding,
                                follows: Default::default(),
                            });
                            letters.entry(following).or_insert_with(|| Letter {
                                character: following,
                                follows: Default::default(),
                            });
                            if captured_precedence {
                                continue;
                            }
                            // ignore matching prefixes
                            if preceding == following {
                                continue;
                            }
                            // at the first differing letter, encode precedence
                            letters
                                .get_mut(&following)
                                .expect("Unmapped letter")
                                .follows
                                .insert(preceding);
                            // we can't tell anything about subsequent letters
                            captured_precedence |= true;
                        }
                        (Some(character), None) => {
                            if !captured_precedence {
                                // prefixes matched, but the first string is longer than the second
                                // this is not a valid lexicographic sorting
                                return "".to_string();
                            }
                            letters.entry(character).or_insert_with(|| Letter {
                                character,
                                follows: Default::default(),
                            });
                        }
                        (None, Some(character)) => {
                            letters.entry(character).or_insert_with(|| Letter {
                                character,
                                follows: Default::default(),
                            });
                        }
                        (None, None) => break,
                    }
                }
            }
        }

        // emit topological order
        // prioritize the letters that have fewer preceding letters
        let mut buckets = BTreeMap::new();
        for letter in letters.values() {
            let preceding_count = letter.follows.len();
            buckets
                .entry(preceding_count)
                .and_modify(|bucket: &mut HashSet<char>| {
                    bucket.insert(letter.character);
                })
                .or_insert_with(|| HashSet::from([letter.character]));
        }

        let mut completed = HashSet::with_capacity(letters.len());
        let mut chars: Vec<char> = Vec::with_capacity(letters.len());

        for bucket in buckets.into_values() {
            let mut stack = VecDeque::new();
            for character in bucket {
                if !completed.contains(&character) {
                    stack.push_back(character);
                }
            }

            // Depth-First-Search to emit lexicographic order
            let mut visiting = HashSet::new();
            while let Some(next) = stack.pop_back() {
                if visiting.contains(&next) {
                    // we have already processed all dependencies for this letter
                    if !completed.contains(&next) {
                        chars.push(next);
                        completed.insert(next);
                    }
                    visiting.remove(&next);
                } else {
                    // put this back on the stack until we've processed all the preceding letters
                    stack.push_back(next);
                    visiting.insert(next);
                    let letter = letters.get(&next).expect("Unmapped letter");
                    for preceding in &letter.follows {
                        if visiting.contains(preceding) {
                            // cycle detected
                            return "".to_string();
                        }
                        if !completed.contains(preceding) {
                            stack.push_back(*preceding);
                        }
                    }
                }
            }
        }

        String::from_iter(chars.iter())
    }
}

struct Letter {
    character: char,
    /// The characters that _precede_ this letter in the alien dictionary
    follows: HashSet<char>,
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let words = ["wrt", "wrf", "er", "ett", "rftt"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result, "wertf".to_string());
    }

    #[test]
    fn example2() {
        // given
        let words = ["z", "x"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result, "zx".to_string());
    }

    #[test]
    fn example3() {
        // given
        let words = ["z", "x", "z"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn example86() {
        // given
        let words = ["z", "z"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result, "z".to_string());
    }

    #[test]
    fn example88() {
        // given
        let words = ["ab", "adc"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert!(result.contains('a'));
        assert!(result.contains('b'));
        assert!(result.contains('c'));
        assert!(result.contains('d'));
        let d_index = result.find('d').unwrap();
        let b_index = result.find('b').unwrap();
        assert!(d_index > b_index);
    }

    #[test]
    fn example120() {
        // given
        let words = ["wrt", "wrtkj"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert!(result.contains('w'));
        assert!(result.contains('r'));
        assert!(result.contains('t'));
        assert!(result.contains('k'));
        assert!(result.contains('j'));
    }

    #[test]
    fn example121() {
        // given
        let words = ["abc", "ab"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result, "".to_string());
    }

    #[test]
    fn example124() {
        // given
        let words = ["wnlb"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result.len(), 4);
        assert!(result.contains("w"));
        assert!(result.contains("n"));
        assert!(result.contains("l"));
        assert!(result.contains("b"));
    }

    #[test]
    fn example128() {
        // given
        let words = ["aba"];

        // when
        let result = Solution::alien_order(words.map(String::from).to_vec());

        // then
        assert_eq!(result, "ab".to_string());
    }
}
