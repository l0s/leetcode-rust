// 1071. Greatest Common Divisor of Strings
// https://leetcode.com/problems/greatest-common-divisor-of-strings/

pub struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 {
            return str1;
        }
        let max_gcd_length = if str1.len() < str2.len() {
            str1.len()
        } else {
            str2.len()
        };
        for gcd_length in (1..=max_gcd_length).rev() {
            if str1.len() % gcd_length != 0 && str2.len() % gcd_length != 0 {
                continue;
            }
            // is it a divisor of both?
            let candidate_divisor = &str1[0..gcd_length];
            if candidate_divisor != &str2[0..gcd_length] {
                // not common
                continue;
            }
            if vec![candidate_divisor; str1.len() / gcd_length].concat() == str1
                && vec![candidate_divisor; str2.len() / gcd_length].concat() == str2
            {
                return candidate_divisor.to_string();
            } // else not a divisor to at least one of the strings
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let str1 = "ABCABC";
        let str2 = "ABC";

        // when
        let result = Solution::gcd_of_strings(str1.to_string(), str2.to_string());

        // then
        assert_eq!(result, "ABC".to_string());
    }

    #[test]
    pub fn example2() {
        // given
        let str1 = "ABABAB";
        let str2 = "ABAB";

        // when
        let result = Solution::gcd_of_strings(str1.to_string(), str2.to_string());

        // then
        assert_eq!(result, "AB".to_string());
    }

    #[test]
    pub fn example3() {
        // given
        let str1 = "LEET";
        let str2 = "CODE";

        // when
        let result = Solution::gcd_of_strings(str1.to_string(), str2.to_string());

        // then
        assert_eq!(result, "".to_string());
    }
}
