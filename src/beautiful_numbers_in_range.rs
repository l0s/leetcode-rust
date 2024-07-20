// 2827. Number of Beautiful Integers in the Range
// https://leetcode.com/problems/number-of-beautiful-integers-in-the-range/

pub struct Solution;

impl Solution {
    pub fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
        let low = low - 1;

        let mut solver = Solver::from(k as usize);
        let upper_count =
            solver.beautiful_numbers(&extract_digits(high), Counts::default(), true, 0, true, 0);

        let lower_count = if low == 0 {
            1
        } else {
            let mut solver = Solver::from(k as usize);
            solver.beautiful_numbers(&extract_digits(low), Counts::default(), true, 0, true, 0)
        };

        (upper_count - lower_count) as i32
    }
}

/// Inspired by:
/// https://leetcode.com/problems/number-of-beautiful-integers-in-the-range/solutions/3933279/c-digit-dp-intuition-explanation-clear-implementation/
struct Solver {
    /// All beautiful numbers must be divisible by this number
    divisor: usize,

    /// storage for intermediate solutions
    ///
    /// The outer vector will have one entry for each digit of the input number. e.g. for "8191",
    /// there will be four entries. There can be up to 10 digits. The middle vector will have one
    /// entry for each possible number of even numbers (up to the number of digits). The inner-most
    /// vector will have one entry for each possible number of odd numbers (up to the number of
    /// digits).
    state: Vec<Vec<Vec<Edge>>>,
}

impl From<usize> for Solver {
    fn from(divisor: usize) -> Self {
        assert_ne!(divisor, 0);
        let state = vec![vec![vec![Edge::default(); 10]; 10]; 10];
        Self { divisor, state }
    }
}

/// Storage for intermediate solutions for both boundary and non-boundary values.
#[derive(Default, Clone)]
struct Edge {
    /// intermediate values for boundary values
    yes: LeadingZero,
    /// intermediate values for non-boundary values
    no: LeadingZero,
}

impl Edge {
    /// Get the mutable storage for intermediate solutions
    ///
    /// Parameters:
    /// - `is_edge` - whether the number is the upper bound
    ///
    /// Returns:
    /// - mutable storage for intermediate solutions
    fn leading_zero(&mut self, is_edge: bool) -> &mut LeadingZero {
        if is_edge {
            &mut self.yes
        } else {
            &mut self.no
        }
    }
}

/// Storage for intermediate solutions broken out by whether the number being considered has a
/// leading zero.
#[derive(Clone)]
struct LeadingZero {
    /// The number of beautiful numbers for each remainder if there is a leading zero (up to a size
    /// of 21)
    yes: Vec<Option<usize>>,
    /// The number of beautiful numbers for each remainder if there is no leading zero (up to a size
    /// of 21)
    no: Vec<Option<usize>>,
}

impl LeadingZero {
    /// Get the intermediate storage based on whether there is a leading zero
    ///
    /// Parameters:
    /// - `has_leading_zero` - true if the string representation of the number has a leading zero
    ///
    /// Returns:
    /// - mutable vector of optional counts
    fn remainders(&mut self, has_leading_zero: bool) -> &mut Vec<Option<usize>> {
        if has_leading_zero {
            &mut self.yes
        } else {
            &mut self.no
        }
    }
}

impl Default for LeadingZero {
    fn default() -> Self {
        // k is between 1 and 20 (inclusive), so there are at most 21 distinct remainders
        let yes = vec![None; 21];
        let no = vec![None; 21];
        Self { yes, no }
    }
}

#[derive(Default, Clone, Copy)]
struct Counts {
    /// the number of even digits counted so far
    even: usize,
    /// the number of odd digits counted so far
    odd: usize,
}

impl Counts {
    pub fn increment_evens(&self) -> Self {
        Self {
            even: self.even + 1,
            odd: self.odd,
        }
    }

    pub fn increment_odds(&self) -> Self {
        Self {
            even: self.even,
            odd: self.odd + 1,
        }
    }
}

impl Solver {
    /// Calculate the number of beautiful numbers between 0 and `digits`
    ///
    /// Parameters:
    /// - digits - the upper bound, represented as individual base 10 digits, cannot exceed 10^9
    /// - counts - the numbers of even and odd digits counted so far
    /// - edge - true if the number under consideration is the upper boundary
    /// - index - the next index into `digits` to examine
    /// - leading_zero - true if the string representation of the number has a leading zero
    /// - remainder - the remainder after dividing the number by the divisor
    fn beautiful_numbers(
        &mut self,
        digits: &[u8],
        counts: Counts,
        edge: bool,
        index: usize,
        leading_zero: bool,
        remainder: usize,
    ) -> usize {
        if index == digits.len() {
            return if counts.even == counts.odd && remainder % self.divisor == 0 {
                1
            } else {
                0
            };
        }

        if let Some(count) = &self.state[index][counts.even][counts.odd]
            .leading_zero(edge)
            .remainders(leading_zero)[remainder]
        {
            return *count;
        }

        let mut result = 0;
        if edge {
            let digit = digits[index] as usize;
            for digit in 0..digit {
                let remainder = (digit * 10usize.pow((digits.len() - index - 1) as u32)
                    + remainder)
                    % self.divisor;
                result += self.beautiful_numbers_for_digit(
                    digits,
                    counts,
                    index,
                    leading_zero,
                    digit,
                    remainder,
                );
            }
            let remainder = ((digit * 10usize.pow((digits.len() - index - 1) as u32)) + remainder)
                % self.divisor;
            result += if digit % 2 == 0 {
                self.beautiful_numbers(
                    digits,
                    counts.increment_evens(),
                    true,
                    index + 1,
                    false,
                    remainder,
                )
            } else {
                self.beautiful_numbers(
                    digits,
                    counts.increment_odds(),
                    true,
                    index + 1,
                    false,
                    remainder,
                )
            };
        } else {
            for digit in 0..=9usize {
                let remainder = ((digit * 10usize.pow((digits.len() - index - 1) as u32))
                    + remainder)
                    % self.divisor;
                result += self.beautiful_numbers_for_digit(
                    digits,
                    counts,
                    index,
                    leading_zero,
                    digit,
                    remainder,
                );
            }
        }
        *self
            .state
            .get_mut(index)
            .unwrap()
            .get_mut(counts.even)
            .unwrap()
            .get_mut(counts.odd)
            .unwrap()
            .leading_zero(edge)
            .remainders(leading_zero)
            .get_mut(remainder)
            .unwrap() = Some(result);
        result
    }

    fn beautiful_numbers_for_digit(
        &mut self,
        digits: &[u8],
        counts: Counts,
        index: usize,
        leading_zero: bool,
        digit: usize,
        remainder: usize,
    ) -> usize {
        if digit % 2 == 0 {
            if leading_zero && digit == 0 {
                self.beautiful_numbers(digits, counts, false, index + 1, true, remainder)
            } else {
                self.beautiful_numbers(
                    digits,
                    counts.increment_evens(),
                    false,
                    index + 1,
                    false,
                    remainder,
                )
            }
        } else {
            self.beautiful_numbers(
                digits,
                counts.increment_odds(),
                false,
                index + 1,
                false,
                remainder,
            )
        }
    }
}

/// Convert a number into its base 10 digits
///
/// Parameters:
/// - `number` - must be positive
fn extract_digits(number: i32) -> Vec<u8> {
    let mut number = number;
    let mut result = vec![];
    while number >= 10 {
        let digit = number % 10;
        result.insert(0, digit as u8);
        number /= 10;
    }
    result.insert(0, number as u8);
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
