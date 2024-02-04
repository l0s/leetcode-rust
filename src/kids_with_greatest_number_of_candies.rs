// 1431. Kids With the Greatest Number of Candies
// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies

pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().expect("Expect at least one kid.");
        candies
            .iter()
            .map(|count| count + extra_candies >= *max)
            .collect::<Vec<bool>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let candies = [2, 3, 5, 1, 3];
        let extra_candies = 3;

        // when
        let result = Solution::kids_with_candies(candies.into(), extra_candies);

        // then
        assert_eq!(result, vec![true, true, true, false, true]);
    }

    #[test]
    pub fn example2() {
        // given
        let candies = [4, 2, 1, 1, 2];
        let extra_candies = 1;

        // when
        let result = Solution::kids_with_candies(candies.into(), extra_candies);

        // then
        assert_eq!(result, vec![true, false, false, false, false]);
    }

    #[test]
    pub fn example3() {
        // given
        let candies = [12, 1, 12];
        let extra_candies = 10;

        // when
        let result = Solution::kids_with_candies(candies.into(), extra_candies);

        // then
        assert_eq!(result, vec![true, false, true]);
    }
}
