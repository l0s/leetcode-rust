// 238. Product of Array Except Self
// https://leetcode.com/problems/product-of-array-except-self

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_product = 1i32;
        let mut prefix_products = vec![prefix_product; nums.len()];
        for (i, num) in nums.iter().enumerate() {
            prefix_product *= num;
            prefix_products[i] = prefix_product;
        }
        let mut postfix_product = 1i32;
        let mut postfix_products = vec![postfix_product; nums.len()];
        for (i, num) in nums.iter().enumerate().rev() {
            postfix_product *= num;
            postfix_products[i] = postfix_product;
        }
        (0..nums.len())
            .map(|i| -> i32 {
                let left = if i == 0 { 1 } else { prefix_products[i - 1] };
                let right = if i == postfix_products.len() - 1 {
                    1
                } else {
                    postfix_products[i + 1]
                };
                left * right
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums = [1, 2, 3, 4];
        /*

        */

        // when
        let result = Solution::product_except_self(nums.into());

        // then
        assert_eq!(&result, &[24, 12, 8, 6]);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = [-1, 1, 0, -3, 3];

        // when
        let result = Solution::product_except_self(nums.into());

        // then
        assert_eq!(&result, &[0, 0, 9, 0, 0]);
    }

    #[test]
    pub fn example18() {
        // given
        let nums = [5, 9, 2, -9, -9, -7, -8, 7, -9, 10];

        // when
        let result = Solution::product_except_self(nums.into());

        // then
        assert_eq!(
            &result,
            &[
                -51438240, -28576800, -128595600, 28576800, 28576800, 36741600, 32148900,
                -36741600, 28576800, -25719120
            ]
        );
    }
}
