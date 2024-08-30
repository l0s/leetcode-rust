// 79. Word Search
// https://leetcode.com/problems/word-search

pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() || board[0].is_empty() {
            return false;
        }
        if word.is_empty() {
            return true;
        }
        let word = word.chars().collect::<Vec<char>>();

        // BFS with multiple starting points
        let mut queue = VecDeque::new();

        // starting points are all the cells that match the first letter
        if let Some(first) = word.first() {
            let remaining = &word[1..];
            for (i, row) in board.iter().enumerate() {
                for (j, candidate) in row.iter().enumerate() {
                    if first == candidate {
                        let node = Node {
                            x: i,
                            y: j,
                            remaining,
                            visited: HashSet::from([(i, j)]),
                        };
                        if node.completes_word() {
                            return true;
                        }
                        queue.push_back(node)
                    }
                }
            }
        }

        while let Some(node) = queue.pop_front() {
            for neighbor in node.neighbors(&board) {
                if neighbor.completes_word() {
                    return true;
                }
                queue.push_back(neighbor);
            }
        }

        false
    }
}

struct Node<'a> {
    x: usize,
    y: usize,
    remaining: &'a [char],
    visited: HashSet<(usize, usize)>,
}

impl Node<'_> {
    fn completes_word(&self) -> bool {
        self.remaining.is_empty()
    }

    fn neighbors(&self, board: &[Vec<char>]) -> Vec<Self> {
        if self.completes_word() {
            return vec![];
        }
        let mut result = Vec::with_capacity(4);
        if self.x > 0
            && !self.visited.contains(&(self.x - 1, self.y))
            && board[self.x - 1][self.y] == self.remaining[0]
        {
            let mut visited = self.visited.clone();
            visited.insert((self.x - 1, self.y));
            result.push(Self {
                x: self.x - 1,
                y: self.y,
                remaining: &self.remaining[1..],
                visited,
            });
        }
        if self.x < board.len() - 1
            && !self.visited.contains(&(self.x + 1, self.y))
            && board[self.x + 1][self.y] == self.remaining[0]
        {
            let mut visited = self.visited.clone();
            visited.insert((self.x + 1, self.y));
            result.push(Self {
                x: self.x + 1,
                y: self.y,
                remaining: &self.remaining[1..],
                visited,
            });
        }
        let column = &board[self.x];
        if self.y > 0
            && !self.visited.contains(&(self.x, self.y - 1))
            && column[self.y - 1] == self.remaining[0]
        {
            let mut visited = self.visited.clone();
            visited.insert((self.x, self.y - 1));
            result.push(Self {
                x: self.x,
                y: self.y - 1,
                remaining: &self.remaining[1..],
                visited,
            });
        }
        if self.y < column.len() - 1
            && !self.visited.contains(&(self.x, self.y + 1))
            && column[self.y + 1] == self.remaining[0]
        {
            let mut visited = self.visited.clone();
            visited.insert((self.x, self.y + 1));
            result.push(Self {
                x: self.x,
                y: self.y + 1,
                remaining: &self.remaining[1..],
                visited,
            })
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
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ];
        let board = array_to_vec(&board);
        let word = "ABCCED";

        // when
        let result = Solution::exist(board, word.to_string());

        // then
        assert!(result);
    }

    #[test]
    fn example2() {
        // given
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ];
        let board = array_to_vec(&board);
        let word = "SEE";

        // when
        let result = Solution::exist(board, word.to_string());

        // then
        assert!(result);
    }

    #[test]
    fn example3() {
        // given
        let board = [
            ['A', 'B', 'C', 'E'],
            ['S', 'F', 'C', 'S'],
            ['A', 'D', 'E', 'E'],
        ];
        let board = array_to_vec(&board);
        let word = "ABCB";

        // when
        let result = Solution::exist(board, word.to_string());

        // then
        assert!(!result);
    }

    #[test]
    fn example85() {
        // given
        let board = [['a']];
        let board = array_to_vec(&board);
        let word = "a";

        // when
        let result = Solution::exist(board, word.to_string());

        // then
        assert!(result);
    }
}
