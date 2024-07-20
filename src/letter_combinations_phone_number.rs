// 17. Letter Combinations of a Phone Number
// From: https://leetcode.com/problems/letter-combinations-of-a-phone-number/

pub struct Solution;

const NUMBER_LETTERS: &[&[char]; 8] = &[
    &['a', 'b', 'c'],
    &['d', 'e', 'f'],
    &['g', 'h', 'i'],
    &['j', 'k', 'l'],
    &['m', 'n', 'o'],
    &['p', 'q', 'r', 's'],
    &['t', 'u', 'v'],
    &['w', 'x', 'y', 'z'],
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits_vec: Vec<char> = digits.chars().collect();
        combinations(&digits_vec)
            .iter()
            .map(|v| v.iter().cloned().collect())
            .collect()
    }
}

fn combinations(digits: &[char]) -> Vec<Vec<char>> {
    if digits.is_empty() {
        return vec![];
    }
    let first = digits[0];
    let remaining = &digits[1..];

    let prefixes = NUMBER_LETTERS[index(first)];
    let suffixes = combinations(remaining);
    if suffixes.is_empty() {
        return prefixes.iter().map(|prefix| vec![*prefix]).collect();
    }

    let mut result = Vec::with_capacity(prefixes.len() * suffixes.len());
    for prefix in prefixes {
        for suffix in &suffixes {
            result.push([vec![*prefix], suffix.clone()].concat());
        }
    }

    result
}

fn index(c: char) -> usize {
    c.to_digit(10)
        .unwrap_or_else(|| panic!("Unmapped key: {}", c)) as usize
        - 2
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let digits = "23".to_string();

        // when
        let result: BTreeSet<String> = Solution::letter_combinations(digits)
            .iter()
            .map(ToString::to_string)
            .collect();

        // then
        assert_eq!(
            result,
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
                .iter()
                .map(ToString::to_string)
                .collect()
        );
    }

    #[test]
    fn example2() {
        // given
        let digits = "".to_string();

        // when
        let result = Solution::letter_combinations(digits);

        // then
        assert!(result.is_empty());
    }

    #[test]
    fn example3() {
        // given
        let digits = "2".to_string();

        // when
        let result: BTreeSet<String> = Solution::letter_combinations(digits)
            .iter()
            .map(ToString::to_string)
            .collect();

        // then
        assert_eq!(
            result,
            ["a", "b", "c"].iter().map(ToString::to_string).collect()
        );
    }
}
