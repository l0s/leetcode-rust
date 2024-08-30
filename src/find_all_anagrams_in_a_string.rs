// 438. Find All Anagrams in a String
// https://leetcode.com/problems/find-all-anagrams-in-a-string

pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }
        let word: DeconstructedWord = (&*p).into();
        let mut indices = Vec::with_capacity(s.len() - word.length);
        let mut window: DeconstructedWord = (&s[0..word.length]).into();
        let s_bytes = s.as_bytes();
        for i in 0..(s.len() - word.length + 1) {
            if window == word {
                indices.push(i);
            }
            window.remove_letter(s_bytes[i]);
            if i + word.length < s_bytes.len() {
                window.add_letter(s_bytes[i + word.length]);
            }
        }

        indices.into_iter().map(|index| index as i32).collect()
    }
}

#[derive(Eq, PartialEq)]
struct DeconstructedWord {
    letter_counts: Vec<usize>,
    length: usize,
}

impl DeconstructedWord {
    fn add_letter(&mut self, letter: u8) {
        self.letter_counts[(letter - b'a') as usize] += 1;
    }

    fn remove_letter(&mut self, letter: u8) {
        self.letter_counts[(letter - b'a') as usize] -= 1;
    }
}

impl From<&str> for DeconstructedWord {
    fn from(value: &str) -> Self {
        // "s and p consist of lowercase English letters."
        let mut letter_counts = vec![0; 26];

        for index in value.as_bytes().iter().map(|b| (b - b'a') as usize) {
            letter_counts[index] += 1;
        }

        Self {
            letter_counts,
            length: value.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "cbaebabacd";
        let p = "abc";

        // when
        let result = Solution::find_anagrams(s.to_string(), p.to_string());

        // then
        assert_eq!(result, vec![0, 6]);
    }

    #[test]
    fn example2() {
        // given
        let s = "abab";
        let p = "ab";

        // when
        let result = Solution::find_anagrams(s.to_string(), p.to_string());

        // then
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn example21() {
        // given
        let s = "aaaaaaaaaa";
        let p = "aaaaaaaaaaaaa";

        // when
        let result = Solution::find_anagrams(s.to_string(), p.to_string());

        // then
        assert!(result.is_empty());
    }
}
