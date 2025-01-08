// 54. Spiral Matrix
// https://leetcode.com/problems/spiral-matrix

pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        } else if matrix.len() == 1 {
            return matrix[0].clone();
        } else if matrix[0].len() == 1 {
            return matrix.iter().map(|singleton| singleton[0]).collect();
        }
        let mut state = State {
            max_row: matrix.len() - 1,
            max_column: matrix[0].len() - 1,
            ..Default::default()
        };

        let mut result = Vec::with_capacity(state.max_row * state.max_column);
        while state.min_row <= state.max_row && state.min_column <= state.max_column {
            let (indices, new_state) = state.direction.consume_line(&state);
            for cursor in indices {
                result.push(matrix[cursor.0][cursor.1]);
            }
            state = new_state;
        }
        // consume the final tile
        result.push(matrix[state.cursor.0][state.cursor.1]);
        result
    }
}

#[derive(Default, Copy, Clone, Debug)]
struct State {
    cursor: (usize, usize),
    /// The top-most row (inclusive)
    min_row: usize,
    /// The bottom-most row (inclusive)
    max_row: usize,
    /// The left-most column (inclusive)
    min_column: usize,
    /// The right-most column (inclusive)
    max_column: usize,
    /// The direction in which to next consume elements
    direction: Direction,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    /// Consume the top-most row from left to right
    Right,
    /// Consume the right-most column from top to bottom
    Down,
    /// Consume the bottom-most row from right to left
    Left,
    /// Consume the left-most column from bottom to top
    Up,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

impl Direction {
    fn consume_line(&self, current_state: &State) -> (Vec<(usize, usize)>, State) {
        let origin = current_state.cursor;
        match self {
            Direction::Right => {
                let mut coordinates = Vec::with_capacity(
                    current_state
                        .max_column
                        .saturating_sub(current_state.min_column),
                );
                for y in origin.1..current_state.max_column {
                    coordinates.push((origin.0, y));
                }
                let new_state = State {
                    cursor: (origin.0, current_state.max_column),
                    min_row: current_state.min_row + 1, // consumed the top-most row
                    max_row: current_state.max_row,
                    min_column: current_state.min_column,
                    max_column: current_state.max_column,
                    direction: self.next(),
                };
                (coordinates, new_state)
            }
            Direction::Down => {
                let mut coordinates =
                    Vec::with_capacity(current_state.max_row.saturating_sub(current_state.min_row));
                for x in origin.0..current_state.max_row {
                    coordinates.push((x, origin.1));
                }
                let new_state = State {
                    cursor: (current_state.max_row, origin.1),
                    min_row: current_state.min_row,
                    max_row: current_state.max_row,
                    min_column: current_state.min_column,
                    max_column: current_state.max_column.saturating_sub(1), // consumed the right-most column
                    direction: self.next(),
                };
                (coordinates, new_state)
            }
            Direction::Left => {
                let mut coordinates = Vec::with_capacity(
                    current_state
                        .max_column
                        .saturating_sub(current_state.min_column),
                );
                for y in ((current_state.min_column + 1)..=origin.1).rev() {
                    coordinates.push((origin.0, y));
                }
                let new_state = State {
                    cursor: (origin.0, current_state.min_column),
                    min_row: current_state.min_row,
                    max_row: current_state.max_row.saturating_sub(1), // consumed the bottom-most row
                    min_column: current_state.min_column,
                    max_column: current_state.max_column,
                    direction: self.next(),
                };
                (coordinates, new_state)
            }
            Direction::Up => {
                let origin = current_state.cursor;
                let mut coordinates =
                    Vec::with_capacity(current_state.max_row.saturating_sub(current_state.min_row));
                for x in ((current_state.min_row + 1)..=origin.0).rev() {
                    coordinates.push((x, origin.1));
                }
                let new_state = State {
                    cursor: (current_state.min_row, origin.1),
                    min_row: current_state.min_row,
                    max_row: current_state.max_row,
                    min_column: current_state.min_column + 1, // consumed the left-most column
                    max_column: current_state.max_column,
                    direction: self.next(),
                };
                (coordinates, new_state)
            }
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::matrix_util::array_to_vec;

    #[test]
    fn example1_even_square() {
        // given
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

        // when
        let result = Solution::spiral_order(array_to_vec(&matrix));

        // then
        assert_eq!(&result, &[1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn example2_wide() {
        // given
        let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];

        // when
        let result = Solution::spiral_order(array_to_vec(&matrix));

        // then
        assert_eq!(&result, &[1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
    }

    #[test]
    fn example25() {
        // given
        let matrix = [[7], [9], [6]];

        // when
        let result = Solution::spiral_order(array_to_vec(&matrix));

        // then
        assert_eq!(&result, &[7, 9, 6]);
    }

    #[test]
    fn even_square() {
        // given
        let matrix = [[1, 2], [3, 4]];

        // when
        let result = Solution::spiral_order(array_to_vec(&matrix));

        // then
        assert_eq!(&result, &[1, 2, 4, 3]);
    }

    #[test]
    fn tall() {
        // given
        let matrix = [[1, 2], [3, 4], [5, 6]];

        // when
        let result = Solution::spiral_order(array_to_vec(&matrix));

        // then
        assert_eq!(&result, &[1, 2, 4, 6, 5, 3]);
    }
}
