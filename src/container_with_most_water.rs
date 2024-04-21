// 11. Container With Most Water
// https://leetcode.com/problems/container-with-most-water

pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = heights.len() - 1;

        let mut max = 0i32;
        while left < right {
            let width: i32 = (right - left) as i32;
            let height = heights[left].min(heights[right]);
            let volume = width * height;
            max = max.max(volume);
            if heights[left] < heights[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let height = [1, 8, 6, 2, 5, 4, 8, 3, 7];

        // when
        let result = Solution::max_area(height.to_vec());

        // then
        assert_eq!(result, 49);
    }

    #[test]
    pub fn example2() {
        // given
        let height = [1, 1];

        // when
        let result = Solution::max_area(height.to_vec());

        // then
        assert_eq!(result, 1);
    }

    #[test]
    pub fn example29() {
        // given
        let height = [2, 3, 4, 5, 18, 17, 6];

        // when
        let result = Solution::max_area(height.to_vec());

        // then
        assert_eq!(result, 17);
    }
}
