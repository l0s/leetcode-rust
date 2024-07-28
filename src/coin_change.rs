// 322. Coin Change
// https://leetcode.com/problems/coin-change

pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let coins = coins
            .into_iter()
            .map(|denomination| denomination as u32)
            .collect::<Vec<u32>>();
        if let Some(result) = Self::min_coins_required(&coins, amount as u32) {
            return result as i32;
        }
        -1
    }

    fn min_coins_required(denominations: &[u32], target_amount: u32) -> Option<u32> {
        if target_amount == 0 {
            return Some(0);
        }
        if denominations.is_empty() {
            return if target_amount == 0 { Some(0) } else { None };
        } else if denominations.len() == 1 {
            return if target_amount % denominations[0] == 0 {
                Some(target_amount / denominations[0])
            } else {
                None
            };
        }
        let mut minimum_coins_for_amount = vec![target_amount + 1; (target_amount + 1) as usize];
        minimum_coins_for_amount[0] = 0; // 0 coins required to reach a target of 0
        for target_amount in 1..=target_amount {
            for denomination in denominations
                .iter()
                .filter(|denomination| **denomination <= target_amount)
            {
                let remaining_amount = target_amount - denomination;
                let minimum_coins_for_remaining_amount =
                    minimum_coins_for_amount[remaining_amount as usize];
                minimum_coins_for_amount[target_amount as usize] = minimum_coins_for_amount
                    [target_amount as usize]
                    .min(1 + minimum_coins_for_remaining_amount);
            }
        }

        if minimum_coins_for_amount[target_amount as usize] > target_amount {
            None
        } else {
            Some(minimum_coins_for_amount[target_amount as usize])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let coins = [1, 2, 5];
        let amount = 11;

        // when
        let result = Solution::coin_change(coins.to_vec(), amount);

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        // given
        let coins = [2];
        let amount = 3;

        // when
        let result = Solution::coin_change(coins.to_vec(), amount);

        // then
        assert_eq!(result, -1);
    }

    #[test]
    fn example3() {
        // given
        let coins = [1];
        let amount = 0;

        // when
        let result = Solution::coin_change(coins.to_vec(), amount);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example32() {
        // given
        let coins = [186, 419, 83, 408];
        let amount = 6249;

        // when
        let result = Solution::coin_change(coins.to_vec(), amount);

        // then
        assert_eq!(result, 20);
    }

    #[test]
    fn example40() {
        // given
        let coins = [411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422];
        let amount = 9864;

        // when
        let result = Solution::coin_change(coins.to_vec(), amount);

        // then
        assert_eq!(result, 24);
    }

    #[test]
    fn example133() {
        // given
        let coins = [3, 7, 405, 436];
        let amount = 8839;

        // when
        let result = Solution::coin_change(coins.to_vec(), amount);

        // then
        assert_eq!(result, 25);
    }
}
