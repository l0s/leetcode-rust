// 17. Letter Combinations of a Phone Number
// From: https://leetcode.com/problems/letter-combinations-of-a-phone-number/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // TODO can use an array
        let number_letters: HashMap<char, Vec<char>> = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        let digits_vec: Vec<char> = digits.chars().collect();
        combinations(&digits_vec, &number_letters)
            .iter()
            .map(|v| v.iter().cloned().collect())
            .collect()
    }
}

fn combinations(digits: &[char], number_letters: &HashMap<char, Vec<char>>) -> Vec<Vec<char>> {
    if digits.is_empty() {
        return vec![];
    }
    let first = digits[0];
    let remaining = &digits[1..];

    let prefixes = number_letters.get(&first).expect(&format!("Unmapped key: {}", first));
    let suffixes = combinations(remaining, number_letters);
    if suffixes.is_empty() {
        return prefixes.iter().map(|prefix| vec![*prefix]).collect()
    }

    let mut result = vec![];
    for prefix in prefixes {
        for suffix in &suffixes {
            let partial = vec![vec![*prefix], suffix.clone()].concat();
            result.push(partial);
        }
    }

    result
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
