// 2827. Number of Beautiful Integers in the Range
// https://leetcode.com/problems/number-of-beautiful-integers-in-the-range/

pub struct Solution;

impl Solution {
    pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
        eprintln!("-- number_of_beautiful_integers( {}, {}, {} )", low, high, k);
        let high = high.min( 10i32.pow(8));
        if let Some(first) = (low..=high.min(low + k)).find(|number| *number % k == 0) {
            eprintln!("-- first: {}", first);
            let mut result = 0;
            let mut number = first;
            while number <= high {
                let digits = extract_digits(number);
                if digits.len() % 2 != 0 {
                    // jump to the next order of magnitude
                    let new_min = 10i32.pow(digits.len() as u32);
                    if let Some(next_min) = (new_min..=high.min(new_min + k))
                        .find(|number| *number % k == 0) {
                        number = next_min;
                        continue;
                    } else {
                        break;
                    }
                }
                let even_digits = digits.iter().filter(|digit| *digit % 2 == 0).count();
                let odd_digits = digits.len() - even_digits;
                if even_digits == odd_digits {
                    result += 1;
                }
                number += k;
            }
            return result;
        }
        0
    }
}

fn extract_digits(mut number: i32) -> Vec<u8> {
    let mut result = vec![];
    while number >= 10 {
        let digit = number % 10;
        result.push(digit as u8);
        number /= 10;
    }
    result.push(number as u8);
    result
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let low = 10;
        let high = 20;
        let k = 3;

        // when
        let result = Solution::number_of_beautiful_integers(low, high, k);

        // then
        assert_eq!(result, 2);
    }

    #[test]
    fn example2() {
        // given
        let low = 1;
        let high = 10;
        let k = 1;

        // when
        let result = Solution::number_of_beautiful_integers(low, high, k);

        // then
        assert_eq!(result, 1);
    }

    #[test]
    fn example3() {
        // given
        let low = 5;
        let high = 5;
        let k = 2;

        // when
        let result = Solution::number_of_beautiful_integers(low, high, k);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example574() {
        // given
        let low = 349863935;
        let high = 772153463;
        let k = 11;

        // when
        let result = Solution::number_of_beautiful_integers(low, high, k);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example575() {
        // given
        let low = 410887384;
        let high = 531967917;
        let k = 19;

        // when
        let result = Solution::number_of_beautiful_integers(low, high, k);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example590() {
        // given
        let low = 1;
        let high = 1000000000;
        let k = 1;

        // when
        let result = Solution::number_of_beautiful_integers(low, high, k);

        // then
        assert_eq!(result, 24894045);
    }
}