// 62. Unique Paths
// https://leetcode.com/problems/unique-paths

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        let down_steps = (m - 1) as usize;
        let right_steps = (n - 1) as usize;

        let total_steps = down_steps + right_steps;
        choose(total_steps, down_steps) as i32
    }
}

/// "population_size choose sample_size" without integer overflow
/// https://blog.plover.com/math/choose.html
fn choose(mut population_size: usize, sample_size: usize) -> usize {
    if sample_size > population_size {
        return 0;
    }
    if sample_size > population_size / 2 {
        return choose(population_size, population_size - sample_size);
    }
    let mut result = 1;
    for divisor in 1..=sample_size {
        result *= population_size;
        population_size -= 1;
        result /= divisor;
    }
    result
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let m = 3;
        let n = 7;

        // when
        let result = Solution::unique_paths(m, n);

        // then
        assert_eq!(result, 28);
    }

    #[test]
    fn example2() {
        // given
        let m = 3;
        let n = 2;

        // when
        let result = Solution::unique_paths(m, n);

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn example38() {
        // given
        let m = 23;
        let n = 12;

        // when
        let result = Solution::unique_paths(m, n);

        // then
        assert_eq!(result, 193_536_720);
    }

    #[test]
    fn example62() {
        // given
        let m = 100;
        let n = 1;

        // when
        let result = Solution::unique_paths(m, n);

        // then
        assert_eq!(result, 1);
    }

    #[test]
    fn example63() {
        // given
        let m = 100;
        let n = 3;

        // when
        let result = Solution::unique_paths(m, n);

        // then
        assert_eq!(result, 5050);
    }
}
