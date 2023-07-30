// 1690. Stone Game VII
// From: https://leetcode.com/problems/stone-game-vii/

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let mut prefix_sums = Vec::with_capacity(stones.len() + 1);
        prefix_sums.insert(0, stones[0] as u32);

        for i in 0..stones.len() {
            prefix_sums.insert(i + 1, prefix_sums[i] + stones[i] as u32);
        }
        let state = GameState {
            stones: &stones,
            prefix_sums: &prefix_sums,
            start_inclusive: 0,
            end_exclusive: stones.len(),
        };
        let mut cache = HashMap::new();
        Player::Alice.best_outcome(&state, &mut cache).diff()
    }
}

pub struct GameState<'s> {
    stones: &'s [i32],
    prefix_sums: &'s[u32],
    start_inclusive: usize,
    end_exclusive: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default)]
pub struct Score {
    alice: u32,
    bob: u32,
}

/// The players in the game who may have different strategies
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Player {
    Alice,
    Bob,
}

/// A cache key for memoizing game state results
#[derive(Hash, Eq, PartialEq)]
pub struct Key {
    start_inclusive: usize,
    end_exclusive: usize,
    player: u8,
}

impl<'s> GameState<'s> {

    /// the number of stones
    pub fn len(&self) -> usize {
        self.end_exclusive - self.start_inclusive
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The game state after removing the left-most stone
    /// Returns:
    /// - the value of the stone removed
    /// - a new game state instance reflecting the removed stone
    pub fn take_left(&self) -> (i32, Self) {
        (
            self.stones[self.start_inclusive],
            Self {
                stones: self.stones,
                prefix_sums: self.prefix_sums,
                start_inclusive: self.start_inclusive + 1,
                end_exclusive: self.end_exclusive,
            },
        )
    }

    /// The game state after removing the right-most stone
    /// Returns:
    /// - the value of the stone removed
    /// - a new game state instance reflecting the removed stone
    pub fn take_right(&self) -> (i32, Self) {
        (
            self.stones[self.end_exclusive - 1],
            Self {
                stones: self.stones,
                prefix_sums: self.prefix_sums,
                start_inclusive: self.start_inclusive,
                end_exclusive: self.end_exclusive - 1,
            },
        )
    }

    /// The sum of the current tiles
    pub fn sum(&self) -> u32 {
        self.prefix_sums[self.end_exclusive] - self.prefix_sums[self.start_inclusive]
    }
}

impl Score {

    /// The absolute difference in scores, regardless of who won
    pub fn diff(&self) -> i32 {
        let result = if self.alice > self.bob {
            self.alice - self.bob
        } else /* self.alice <= self.bob */ {
            self.bob - self.alice
        };
        result as i32
    }

    /// Add points to a single player
    ///
    /// Parameters:
    /// - `player` - the player for whom to add points
    /// - `points` - the number of points the player just earned
    ///
    /// Returns:
    /// - A new instance reflecting the updated score
    pub fn increment(&self, player: &Player, points: u32) -> Self {
        match player {
            Player::Alice => Self {
                alice: self.alice + points,
                bob: self.bob,
            },
            Player::Bob => Self {
                alice: self.alice,
                bob: self.bob + points,
            },
        }
    }
}

impl Player {

    /// Given a game's current state, identify the best possible outcome for this player.
    ///
    /// Parameters:
    /// - `state` - the current set of stones, assuming it's the current player's turn
    /// - `cache` - a cache of game state to the best outcome, shared by all players
    pub fn best_outcome(&self, state: &GameState, cache: &mut HashMap<Key, Score>) -> Score {
        if state.is_empty() || state.len() == 1 {
            // take the only stone, no points awarded
            return Score::default();
        }

        let key = Key {
            start_inclusive: state.start_inclusive,
            end_exclusive: state.end_exclusive,
            player: *self as u8,
        };
        if cache.contains_key(&key) {
            return cache[&key];
        }

        let (_left_stone, option_a) = state.take_left();
        let my_points_a = option_a.sum();
        let outcome_a = self.next_player().best_outcome(&option_a, cache);
        let outcome_a = outcome_a.increment(self, my_points_a);

        let (_right_stone, option_b) = state.take_right();
        let my_points_b = option_b.sum();
        let outcome_b = self.next_player().best_outcome(&option_b, cache);
        let outcome_b = outcome_b.increment(self, my_points_b);

        let result = self.choose_outcome(outcome_a, outcome_b);
        cache.insert(key, result);
        result
    }

    fn next_player(&self) -> Self {
        match self {
            Self::Alice => Self::Bob,
            Self::Bob => Self::Alice,
        }
    }

    /// Choose the short-term outcome that best suits this player's long-term strategy
    fn choose_outcome(&self, x: Score, y: Score) -> Score {
        let (a, b) = match self {
            Self::Alice => (x.alice as i32 - x.bob as i32, y.alice as i32 - y.bob as i32),
            Self::Bob => (x.bob as i32 - x.alice as i32, y.bob as i32 - y.alice as i32),
        };
        if a > b {
            x
        } else {
            y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let stones = vec![5, 3, 1, 4, 2];

        // when
        let result = Solution::stone_game_vii(stones);

        // then
        assert_eq!(result, 6);
    }

    #[test]
    pub fn example2() {
        // given
        let stones = vec![7, 90, 5, 1, 100, 10, 10, 2];

        // when
        let result = Solution::stone_game_vii(stones);

        // then
        assert_eq!(result, 122);
    }

    #[test]
    pub fn example9() {
        // given
        let stones = vec![721, 979, 690, 84, 742, 873, 31, 323, 819, 22, 928, 866, 118, 843, 169, 818, 908, 832, 852, 480, 763, 715, 875, 629];

        // when
        let result = Solution::stone_game_vii(stones);

        // then
        assert_eq!(result, 7948);
    }
}