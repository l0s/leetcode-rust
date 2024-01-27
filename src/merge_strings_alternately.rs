// 1768. Merge Strings Alternately
// https://leetcode.com/problems/merge-strings-alternately/

pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = Vec::with_capacity(word1.len() + word2.len());
        let mut word1 = word1.chars();
        let mut word2 = word2.chars();
        loop {
            match (word1.next(), word2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                }
                (Some(c1), None) => {
                    result.push(c1);
                    result.extend(word1);
                    break;
                }
                (None, Some(c2)) => {
                    result.push(c2);
                    result.extend(word2);
                    break;
                }
                (None, None) => break,
            }
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
        let word1 = "abc";
        let word2 = "pqr";

        // when
        let result = Solution::merge_alternately(word1.to_string(), word2.to_string());

        // then
        assert_eq!(result, "apbqcr".to_string());
    }

    #[test]
    pub fn example2() {
        // given
        let word1 = "ab";
        let word2 = "pqrs";

        // when
        let result = Solution::merge_alternately(word1.to_string(), word2.to_string());

        // then
        assert_eq!(result, "apbqrs".to_string());
    }

    #[test]
    pub fn example3() {
        // given
        let word1 = "abcd";
        let word2 = "pq";

        // when
        let result = Solution::merge_alternately(word1.to_string(), word2.to_string());

        // then
        assert_eq!(result, "apbqcd".to_string());
    }
}
