// 75. Sort Colors
// https://leetcode.com/problems/sort-colors/

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut reds = 0usize;
        let mut whites = 0usize;
        let mut blues = 0usize;
        for color in &mut *nums {
            let target = match color {
                0 => &mut reds,
                1 => &mut whites,
                2 => &mut blues,
                _ => panic!("Invalid colour"),
            };
            *target += 1;
        }
        for color in nums.iter_mut().take(reds) {
            *color = 0;
        }
        for color in nums.iter_mut().skip(reds).take(whites) {
            *color = 1;
        }
        for color in nums.iter_mut().skip(reds + whites).take(blues) {
            *color = 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let mut nums = vec![2, 0, 2, 1, 1, 0];

        // when
        Solution::sort_colors(&mut nums);

        // then
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    pub fn example2() {
        // given
        let mut nums = vec![2, 0, 1];

        // when
        Solution::sort_colors(&mut nums);

        // then
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
