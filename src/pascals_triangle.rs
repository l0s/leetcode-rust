// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(num_rows as usize);
        let empty = vec![];
        for i in 0..(num_rows as usize) {
            let mut row = vec![1; i + 1];
            let previous = if i > 0 { &result[i - 1] } else { &empty };
            for j in 0..i {
                let left = if j > 0 { previous[j - 1] } else { 0 };
                let right = if j < previous.len() { previous[j] } else { 0 };
                row[j] = left + right;
            }
            result.push(row);
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
        let num_rows = 5;

        // when
        let result = Solution::generate(num_rows);

        // then
        assert_eq!(
            result,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    pub fn example2() {
        // given
        let num_rows = 1;

        // when
        let result = Solution::generate(num_rows);

        // then
        assert_eq!(result, vec![vec![1]]);
    }
}
