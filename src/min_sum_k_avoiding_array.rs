// 2829. Determine the Minimum Sum of a k-avoiding Array
// https://leetcode.com/problems/determine-the-minimum-sum-of-a-k-avoiding-array/

pub struct Solution;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut result = 0;
        let mut len = 0;
        let mut can_insert = [true; 51];

        let mut candidate = 1i32;
        while len < n {
            if (candidate as usize) < can_insert.len() && !can_insert[candidate as usize] {
                candidate += 1;
                continue;
            }
            result += candidate;
            len += 1;
            if k > candidate {
                can_insert[(k - candidate) as usize] = false;
            }
            candidate += 1;
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
        let n = 5;
        let k = 4;

        // when
        let result = Solution::minimum_sum(n, k);

        // then
        assert_eq!(result, 18);
    }

    #[test]
    pub fn example2() {
        // given
        let n = 2;
        let k = 6;

        // when
        let result = Solution::minimum_sum(n, k);

        // then
        assert_eq!(result, 3);
    }
}
