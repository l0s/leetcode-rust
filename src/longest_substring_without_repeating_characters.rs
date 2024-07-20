// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        } else if s.len() == 1 {
            return 1;
        }

        let mut length = 1usize; // maximum window length so far

        let mut start_window_index = 0usize;
        while start_window_index + length <= s.len() {
            let chars = &s[start_window_index..(start_window_index + length)];
            let unique_chars: HashSet<u8> = chars.as_bytes().iter().cloned().collect();
            if unique_chars.len() == length {
                length += 1;
            } else {
                start_window_index += 1;
            }
        }

        length as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "abcabcbb";

        // when
        let result = Solution::length_of_longest_substring(s.to_string());

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        // given
        let s = "bbbbb";

        // when
        let result = Solution::length_of_longest_substring(s.to_string());

        // then
        assert_eq!(result, 1);
    }

    #[test]
    fn example3() {
        // given
        let s = "pwwkew";

        // when
        let result = Solution::length_of_longest_substring(s.to_string());

        // then
        assert_eq!(result, 3);
    }
}
