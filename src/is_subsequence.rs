// 392. Is Subsequence
// https://leetcode.com/problems/is-subsequence

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut last_index = 0usize;
        'chars_to_find: for c in s.chars() {
            for (i, other) in t[last_index..t.len()].chars().enumerate() {
                if c == other {
                    last_index += i + 1;
                    continue 'chars_to_find;
                }
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let s = "abc";
        let t = "ahbgdc";

        // when
        let result = Solution::is_subsequence(s.to_string(), t.to_string());

        // then
        assert!(result);
    }

    #[test]
    pub fn example2() {
        // given
        let s = "axc";
        let t = "ahbgdc";

        // when
        let result = Solution::is_subsequence(s.to_string(), t.to_string());

        // then
        assert!(!result);
    }

    #[test]
    pub fn example15() {
        // given
        let s = "acb";
        let t = "ahbgdc";

        // when
        let result = Solution::is_subsequence(s.to_string(), t.to_string());

        // then
        assert!(!result);
    }

    #[test]
    pub fn example16() {
        // given
        let s = "aaaaaa";
        let t = "bbaaaa";

        // when
        let result = Solution::is_subsequence(s.to_string(), t.to_string());

        // then
        assert!(!result);
    }
}
