// 121. Best Time to Buy and Sell Stock
// From: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0i32;

        let mut best_purchase_price = prices[0];
        for day in 1..prices.len() {
            let market_price = prices[day];
            best_purchase_price = best_purchase_price.min(market_price);
            let potential_profit = market_price - best_purchase_price;
            result = result.max(potential_profit);
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
        let prices = vec![7, 1, 5, 3, 6, 4];

        // when
        let result = Solution::max_profit(prices);

        // then
        assert_eq!(result, 5);
    }

    #[test]
    pub fn example2() {
        // given
        let prices = vec![7, 6, 4, 3, 1];

        // when
        let result = Solution::max_profit(prices);

        // then
        assert_eq!(result, 0);
    }
}