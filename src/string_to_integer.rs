// 8. String to Integer (atoi)
// https://leetcode.com/problems/string-to-integer-atoi

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // Remove any leading whitespace
        let s = s.trim_start_matches(|c: char| c.is_whitespace());
        // Determine sign
        let negative = s.starts_with('-');
        let explicitly_positive = s.starts_with('+');
        let s = if negative || explicitly_positive {
            &s[1..]
        } else {
            s
        };
        // Skip leading zeroes
        let s = s.trim_start_matches('0');
        // Parse remaining digits
        let digits = s
            .chars()
            .map(|c| c.to_digit(10))
            .take_while(Option::is_some)
            .map(Option::unwrap)
            .collect::<Vec<u32>>();
        if digits.len() > 10 {
            // overflow, round
            return if negative { i32::MIN } else { i32::MAX };
        }

        // Calculate value
        let mut positive_result = 0u32;
        for (index, digit) in digits.iter().enumerate() {
            let power = (digits.len() - index - 1) as u32;
            let multiplicand = 10u32.pow(power);
            if multiplicand >= 1_000_000_000 && *digit > 2 {
                // overflow, round
                return if negative { i32::MIN } else { i32::MAX };
            }
            positive_result += multiplicand * *digit;
            if positive_result > i32::MAX as u32 {
                if !negative {
                    return i32::MAX;
                } else if positive_result > i32::MAX as u32 + 1 {
                    return i32::MIN;
                }
            }
        }

        if negative {
            -(positive_result as i32)
        } else {
            positive_result as i32
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "42";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, 42);
    }

    #[test]
    fn example2() {
        // given
        let s = " -042";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, -42);
    }

    #[test]
    fn example3() {
        // given
        let s = "1337c0d3";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, 1337);
    }

    #[test]
    fn example4() {
        // given
        let s = "0-1";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example5() {
        // given
        let s = "words and 987";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example1038() {
        // given
        let s = "+-12";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example1045() {
        // given
        let s = "-6147483648";

        // when
        let result = Solution::my_atoi(s.to_string());

        // then
        assert_eq!(result, -2147483648);
    }
}
