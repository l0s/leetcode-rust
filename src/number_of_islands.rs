// 200. Number of Islands
// https://leetcode.com/problems/number-of-islands

pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // DFS starting at each known land cell
        let mut stack = VecDeque::new();
        let mut island_ids = Vec::with_capacity(grid.len());
        for (i, row) in grid.iter().enumerate() {
            island_ids.push(vec![None; row.len()]);
            for (j, cell) in row.iter().enumerate() {
                if *cell == '1' {
                    stack.push_back((i, j));
                }
            }
        }
        let mut island_count = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        while let Some((i, j)) = stack.pop_back() {
            if visited.contains(&(i, j)) {
                continue;
            }

            if island_ids[i][j].is_none() {
                // new island
                island_count += 1;
                island_ids[i][j] = Some(island_count);
            }
            for (x, y) in neighbours(&grid, i, j) {
                if visited.contains(&(x, y)) {
                    continue;
                }
                // neighbors are part of the same island
                island_ids[x][y] = Some(island_count);
                stack.push_back((x, y));
            }
            visited.insert((i, j));
        }

        island_count
    }
}

fn neighbours(grid: &[Vec<char>], x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut result = HashSet::with_capacity(4);
    if x > 0 && grid[x - 1][y] == '1' {
        result.insert((x - 1, y));
    }
    if x < grid.len() - 1 && grid[x + 1][y] == '1' {
        result.insert((x + 1, y));
    }
    if y > 0 && grid[x][y - 1] == '1' {
        result.insert((x, y - 1));
    }
    if y < grid[x].len() - 1 && grid[x][y + 1] == '1' {
        result.insert((x, y + 1));
    }
    result
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn example1() {
        // given
        let grid = [
            ["1", "1", "1", "1", "0"],
            ["1", "1", "0", "1", "0"],
            ["1", "1", "0", "0", "0"],
            ["0", "0", "0", "0", "0"],
        ];

        // when
        let result = Solution::num_islands(array_to_vec(&grid));

        // then
        assert_eq!(result, 1);
    }

    #[test]
    fn example2() {
        // given
        let grid = [
            ["1", "1", "0", "0", "0"],
            ["1", "1", "0", "0", "0"],
            ["0", "0", "1", "0", "0"],
            ["0", "0", "0", "1", "1"],
        ];

        // when
        let result = Solution::num_islands(array_to_vec(&grid));

        // then
        assert_eq!(result, 3);
    }

    fn array_to_vec<T>(grid: &[T]) -> Vec<Vec<char>>
    where
        T: AsRef<[&'static str]>,
    {
        grid.iter()
            .map(|array| {
                array
                    .as_ref()
                    .iter()
                    .map(|slice| slice.chars().next().unwrap())
                    .collect()
            })
            .collect()
    }
}
