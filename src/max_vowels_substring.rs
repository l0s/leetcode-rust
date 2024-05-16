// 1456. Maximum Number of Vowels in a Substring of Given Length
// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length

pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let chars = s.chars().collect::<Vec<char>>();

        let mut vowel_count: usize = {
            let mut result = 0;

            let initial_window = &chars[0..k];
            for c in initial_window {
                if is_vowel(c) {
                    result += 1;
                }
            }

            result
        };
        let mut max_vowel_count = vowel_count;

        for i in k..chars.len() {
            let outgoing = chars[i - k];
            let incoming = chars[i];

            if is_vowel(&outgoing) {
                vowel_count -= 1;
            }
            if is_vowel(&incoming) {
                vowel_count += 1;
            }

            max_vowel_count = max_vowel_count.max(vowel_count);
        }

        max_vowel_count as i32
    }
}

fn is_vowel(c: &char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let s = "abciiidef";
        let k = 3;

        // when
        let result = Solution::max_vowels(s.into(), k);

        // then
        assert_eq!(result, 3);
    }

    #[test]
    pub fn example2() {
        // given
        let s = "aeiou";
        let k = 2;

        // when
        let result = Solution::max_vowels(s.into(), k);

        // then
        assert_eq!(result, 2);
    }

    #[test]
    pub fn example3() {
        // given
        let s = "leetcode";
        let k = 3;

        // when
        let result = Solution::max_vowels(s.into(), k);

        // then
        assert_eq!(result, 2);
    }
}
