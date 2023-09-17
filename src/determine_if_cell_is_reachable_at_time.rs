// 2849. Determine if a Cell Is Reachable at a Given Time
// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/

pub struct Solution;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        if (sx, sy) == (fx, fy) && t == 1 {
            return false;
        }
        let chebyshev_distance = (sx - fx).abs().max((sy - fy).abs());
        chebyshev_distance <= t
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let sx = 2;
        let sy = 4;
        let fx = 7;
        let fy = 7;
        let t = 6;

        // when
        let result = Solution::is_reachable_at_time(sx, sy, fx, fy, t);

        // then
        assert!(result)
    }

    #[test]
    fn example2() {
        // given
        let sx = 3;
        let sy = 1;
        let fx = 7;
        let fy = 3;
        let t = 3;

        // when
        let result = Solution::is_reachable_at_time(sx, sy, fx, fy, t);

        // then
        assert!(!result)
    }

    #[test]
    fn example666() {
        // given
        let sx = 1;
        let sy = 1;
        let fx = 2;
        let fy = 2;
        let t = 1;

        // when
        let result = Solution::is_reachable_at_time(sx, sy, fx, fy, t);

        // then
        assert!(result)
    }

    #[test]
    fn example697() {
        // given
        let sx = 1;
        let sy = 1;
        let fx = 1;
        let fy = 1;
        let t = 3;

        // when
        let result = Solution::is_reachable_at_time(sx, sy, fx, fy, t);

        // then
        assert!(result)
    }

    #[test]
    fn example714() {
        // given
        let sx = 1;
        let sy = 2;
        let fx = 1;
        let fy = 2;
        let t = 1;

        // when
        let result = Solution::is_reachable_at_time(sx, sy, fx, fy, t);

        // then
        assert!(!result)
    }
}