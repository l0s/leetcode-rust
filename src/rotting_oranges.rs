// 994. Rotting Oranges
// https://leetcode.com/problems/rotting-oranges/

pub struct Solution;

use std::collections::{HashSet, VecDeque};
use Cell::{FreshOrange, RottingOrange};

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut queue = VecDeque::new();
        let mut minutes_to_rot = Vec::with_capacity(grid.len());
        let mut count_fresh = 0usize;
        for (i, row) in grid.iter().enumerate() {
            minutes_to_rot.push(vec![i32::MAX; row.len()]);
            for (j, cell) in row.iter().enumerate() {
                if *cell == RottingOrange as i32 {
                    minutes_to_rot[i][j] = 0;
                    queue.push_back((i, j));
                } else if *cell == FreshOrange as i32 {
                    count_fresh += 1;
                }
            }
        }

        let mut visited = HashSet::new();
        let mut max_minutes = 0;
        while let Some((i, j)) = queue.pop_front() {
            if visited.contains(&(i, j)) {
                continue;
            }
            grid[i][j] = RottingOrange as i32;
            let time_to_rot_current = minutes_to_rot[i][j];
            max_minutes = max_minutes.max(time_to_rot_current);
            for (x, y) in neighboring_fresh_oranges(&grid, i, j) {
                let time_to_rot_neighbour = time_to_rot_current + 1;
                if time_to_rot_neighbour < minutes_to_rot[x][y] {
                    minutes_to_rot[x][y] = time_to_rot_neighbour;
                    queue.push_back((x, y));
                    count_fresh -= 1;
                } // else there was a faster way to rot this orange
            }
            visited.insert((i, j));
        }

        if count_fresh > 0 {
            return -1;
        }
        max_minutes
    }
}

#[repr(i32)]
enum Cell {
    FreshOrange = 1,
    RottingOrange = 2,
}

fn neighboring_fresh_oranges(grid: &[Vec<i32>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);
    if x > 0 && grid[x - 1][y] == FreshOrange as i32 {
        result.push((x - 1, y));
    }
    if x < grid.len() - 1 && grid[x + 1][y] == FreshOrange as i32 {
        result.push((x + 1, y));
    }
    let row = &grid[x];
    if y > 0 && row[y - 1] == FreshOrange as i32 {
        result.push((x, y - 1));
    }
    if y < row.len() - 1 && row[y + 1] == FreshOrange as i32 {
        result.push((x, y + 1));
    }
    result
}

#[cfg(test)]
mod tests {

    use super::Solution;
    use crate::matrix_util;

    #[test]
    fn example1() {
        // given
        let grid = [[2, 1, 1], [1, 1, 0], [0, 1, 1]];

        // when
        let result = Solution::oranges_rotting(matrix_util::array_to_vec(&grid));

        // then
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        // given
        let grid = [[2, 1, 1], [0, 1, 1], [1, 0, 1]];

        // when
        let result = Solution::oranges_rotting(matrix_util::array_to_vec(&grid));

        // then
        assert_eq!(result, -1);
    }

    #[test]
    fn example3() {
        // given
        let grid = [[0, 2]];

        // when
        let result = Solution::oranges_rotting(matrix_util::array_to_vec(&grid));

        // then
        assert_eq!(result, 0);
    }
}
