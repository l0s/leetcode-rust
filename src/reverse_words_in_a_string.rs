// 151. Reverse Words in a String
// https://leetcode.com/problems/reverse-words-in-a-string

pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut stack = vec![];
        s.split_whitespace()
            .for_each(|word| stack.insert(0, word));
        stack.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let s = "the sky is blue";

        // when
        let result = Solution::reverse_words(s.into());

        // then
        assert_eq!(&result, "blue is sky the");
    }

    #[test]
    pub fn example2() {
        // given
        let s = "  hello world  ";

        // when
        let result = Solution::reverse_words(s.into());

        // then
        assert_eq!(&result, "world hello");
    }

    #[test]
    pub fn example3() {
        // given
        let s = "a good   example";

        // when
        let result = Solution::reverse_words(s.into());

        // then
        assert_eq!(&result, "example good a");
    }
}