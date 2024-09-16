// 417. Pacific Atlantic Water Flow
// https://leetcode.com/problems/pacific-atlantic-water-flow

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let num_columns = heights[0].len();

        let flows_to_pacific = Self::calculate_flows_to_pacific(&heights, num_columns);
        let flows_to_atlantic = Self::calculate_flows_to_atlantic(&heights, num_columns);

        heights
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                let flows_to_pacific = flows_to_pacific[i].clone();
                let flows_to_atlantic = flows_to_atlantic[i].clone();
                row.into_iter().enumerate().filter_map({
                    move |(j, _height)| {
                        if flows_to_pacific[j] && flows_to_atlantic[j] {
                            Some((i, j))
                        } else {
                            None
                        }
                    }
                })
            })
            .map(|(x, y)| vec![x as i32, y as i32])
            .collect()
    }

    fn calculate_flows_to_pacific(heights: &[Vec<i32>], num_columns: usize) -> Vec<Vec<bool>> {
        let mut queue = VecDeque::with_capacity(heights.len() * num_columns);
        let mut result = vec![vec![false; num_columns]; heights.len()];
        result[0] = vec![true; num_columns];
        queue.append(&mut (0..num_columns).map(|j| (0usize, j)).collect());
        for (i, row) in result.iter_mut().enumerate() {
            row[0] = true;
            queue.push_back((i, 0usize));
        }

        Self::mark_sources(heights, num_columns, &mut queue, &mut result);

        result
    }

    fn calculate_flows_to_atlantic(heights: &[Vec<i32>], num_columns: usize) -> Vec<Vec<bool>> {
        let mut queue = VecDeque::with_capacity(heights.len() * num_columns);

        let mut result = vec![vec![false; num_columns]; heights.len()];
        result[heights.len() - 1] = vec![true; num_columns];
        queue.append(&mut (0..num_columns).map(|j| (heights.len() - 1, j)).collect());
        for (i, row) in result.iter_mut().enumerate() {
            row[num_columns - 1] = true;
            queue.push_back((i, num_columns - 1));
        }

        Self::mark_sources(heights, num_columns, &mut queue, &mut result);

        result
    }

    fn mark_sources(
        heights: &[Vec<i32>],
        num_columns: usize,
        queue: &mut VecDeque<(usize, usize)>,
        flows_to_ocean: &mut [Vec<bool>],
    ) {
        let mut visited = Vec::with_capacity(heights.len() * num_columns);
        while let Some((x, y)) = queue.pop_front() {
            if visited.contains(&(x, y)) {
                continue;
            }
            for (next_x, next_y) in Self::neighbors(x, y, heights, num_columns) {
                if heights[x][y] <= heights[next_x][next_y] {
                    flows_to_ocean[next_x][next_y] |= true;
                    if !visited.contains(&(next_x, next_y)) {
                        queue.push_back((next_x, next_y));
                    }
                }
            }

            visited.push((x, y));
        }
    }

    fn neighbors(
        x: usize,
        y: usize,
        heights: &[Vec<i32>],
        num_columns: usize,
    ) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < heights.len() - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < num_columns - 1 {
            result.push((x, y + 1));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::matrix_util::array_to_vec;

    #[test]
    fn example1() {
        // given
        let heights = [
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ];

        // when
        let result = Solution::pacific_atlantic(array_to_vec(&heights));

        // then
        assert_eq!(
            result,
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ]
        );
    }

    #[test]
    fn example2() {
        // given
        let heights = [[1]];

        // when
        let result = Solution::pacific_atlantic(array_to_vec(&heights));

        // then
        assert_eq!(result, vec![vec![0, 0]]);
    }

    #[test]
    fn example51() {
        // given
        let heights = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];

        // when
        let result = Solution::pacific_atlantic(array_to_vec(&heights));

        // then
        assert_eq!(
            result,
            vec![
                vec![0, 2],
                vec![1, 0],
                vec![1, 1],
                vec![1, 2],
                vec![2, 0],
                vec![2, 1],
                vec![2, 2]
            ]
        );
    }

    #[test]
    fn test_calculate_flows_to_pacific() {
        // given
        let heights = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];

        // when
        let result = Solution::calculate_flows_to_pacific(&array_to_vec(&heights), 3);

        // then
        assert!(result[0][2]);
        assert!(result[1][2]);
        assert!(result[2][2]);
        assert!(result[2][1]);
    }

    #[test]
    fn test_calculate_flows_to_atlantic() {
        // given
        let heights = [
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ];

        // when
        let result = Solution::calculate_flows_to_atlantic(&array_to_vec(&heights), 5);

        // then
        assert!(!result[0][1]);
    }
}
