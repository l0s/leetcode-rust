// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut result = Vec::with_capacity(nums.len());
        for item in nums.clone() {
            if item != 0 {
                result.push(item);
            }
        }
        result.resize(nums.len(), 0);
        nums.clear();
        nums.append(&mut result);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let mut nums = vec![0, 1, 0, 3, 12];

        // when
        Solution::move_zeroes(&mut nums);

        // then
        assert_eq!(&nums, &[1, 3, 12, 0, 0]);
    }

    #[test]
    pub fn example2() {
        // given
        let mut nums = vec![0];

        // when
        Solution::move_zeroes(&mut nums);

        // then
        assert_eq!(&nums, &[0]);
    }
}
