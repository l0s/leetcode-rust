// 345. Reverse Vowels of a String
// https://leetcode.com/problems/reverse-vowels-of-a-string

pub struct Solution;

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let vowel_indices = chars.iter().enumerate().
            filter(|(_, character)| VOWELS.contains(character))
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        let mut result = chars.clone();
        for (i, vowel_index) in vowel_indices.iter().enumerate() {
            let new_placement = vowel_indices[vowel_indices.len() - i - 1];
            result[new_placement] = chars[*vowel_index];
        }
        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let s = "hello";

        // when
        let result = Solution::reverse_vowels(s.to_string());

        // then
        assert_eq!(&result, "holle");
    }

    #[test]
    pub fn example2() {
        // given
        let s = "leetcode";

        // when
        let result = Solution::reverse_vowels(s.to_string());

        // then
        assert_eq!(&result, "leotcede");
    }
}