// 1732. Find the Highest Altitude
// https://leetcode.com/problems/find-the-highest-altitude

pub struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut current_altitude = 0;
        let mut result = current_altitude;

        for gain in &gain {
            current_altitude += *gain;
            result = result.max(current_altitude);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let gain = [-5, 1, 5, 0, -7];

        // when
        let result = Solution::largest_altitude(gain.into());

        // then
        assert_eq!(result, 1);
    }

    #[test]
    pub fn example2() {
        // given
        let gain = [-4, -3, -2, -1, 4, 3, 2];

        // when
        let result = Solution::largest_altitude(gain.into());

        // then
        assert_eq!(result, 0);
    }
}
