// 5. Longest Palindromic Substring
// https://leetcode.com/problems/longest-palindromic-substring

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        longest_palindrome(&chars)
    }
}

fn longest_palindrome(string: &[char]) -> String {
    let mut result_start = 0usize;
    let mut result_length = 1; // 1 <= s.length <= 1000
    let mut palindrome_table = vec![vec![false; string.len()]; string.len()];
    // a single character is always a palindrome
    for (i, _c) in string.iter().enumerate() {
        palindrome_table[i][i] = true;
    }
    // find all palindromic pairs
    for i in 0..(string.len() - 1) {
        let j = i + 1;
        palindrome_table[i][j] = string[i] == string[j];
        if palindrome_table[i][j] {
            result_start = i;
            result_length = 2;
        }
    }

    // find palindromes longer than 2
    for substring_length in 3..=string.len() {
        for start_index in 0..(string.len() - substring_length + 1) {
            let end_index = start_index + substring_length - 1;
            // check if the inner string is a palindrome
            // if so, check if the outer string is also a palindrome
            palindrome_table[start_index][end_index] = palindrome_table[start_index + 1]
                [end_index - 1]
                && string[start_index] == string[end_index];
            if palindrome_table[start_index][end_index] && substring_length > result_length {
                result_start = start_index;
                result_length = substring_length;
            }
        }
    }

    string[result_start..(result_start + result_length)]
        .iter()
        .collect()
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "babad";

        // when
        let result = Solution::longest_palindrome(s.to_string());

        // then
        assert!(result == "bab".to_string() || result == "aba".to_string())
    }

    #[test]
    fn example2() {
        // given
        let s = "cbbd";

        // when
        let result = Solution::longest_palindrome(s.to_string());

        // then
        assert_eq!(result, "bb".to_string());
    }

    #[test]
    fn example45() {
        // given
        let s = "babaddtattarrattatddetartrateedredividerb";

        // when
        let result = Solution::longest_palindrome(s.to_string());

        // then
        assert_eq!(result, "ddtattarrattatdd".to_string());
    }

    #[test]
    fn example85() {
        // given
        let s = "a";

        // when
        let result = Solution::longest_palindrome(s.to_string());

        // then
        assert_eq!(result, "a".to_string());
    }

    #[test]
    fn example92() {
        // given
        let s = "ccc";

        // when
        let result = Solution::longest_palindrome(s.to_string());

        // then
        assert_eq!(result, "ccc".to_string());
    }
}
