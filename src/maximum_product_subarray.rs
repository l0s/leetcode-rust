// 152. Maximum Product Subarray
// https://leetcode.com/problems/maximum-product-subarray/

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let nums = compress(&nums);
        let mut result = i32::MIN;
        let mut products = vec![vec![i32::MIN; nums.len()]; nums.len()];
        for (start_index, first_item) in nums.iter().enumerate() {
            for (end_index, last_item) in nums.iter().enumerate().skip(start_index) {
                let product = if end_index == start_index {
                    *first_item
                } else if end_index == start_index + 1 {
                    first_item * last_item
                } else {
                    last_item * products[start_index][end_index - 1]
                };
                products[start_index][end_index] = product;
                result = result.max(product);
            }
        }
        result
    }
}

/// Collapse contiguous 0s or 1s as multiplying by those numbers any number of times will result in
/// the same value.
fn compress(multiplicands: &[i32]) -> Vec<i32> {
    // TODO add support for -1
    let mut index_pairs = vec![];
    let mut previous = multiplicands[0];
    let (mut start, mut end) = if previous == 0 || previous == 1 {
        (0, 0)
    } else {
        (usize::MAX, usize::MIN)
    };
    for (index, item) in multiplicands.iter().enumerate().skip(1) {
        // start of window
        if start > index && previous != 0 && previous != 1 && (*item == 0 || *item == 1) {
            start = index;
            end = index;
        }
        // continuation of window
        else if start < index && *item == previous && (*item == 0 || *item == 1) {
            end = index;
        }
        // end of window
        else if start < index && *item != previous {
            end = index;
            index_pairs.push((start, end));
            start = usize::MAX;
            end = usize::MIN;
            // check if we're starting a new window
            if *item == 0 || *item == 1 {
                start = index;
                end = index;
            }
        }
        // else, non window, do nothing

        previous = *item;
    }
    // check if this is the end of a window
    if start < multiplicands.len() - 1 && end > start {
        index_pairs.push((start, end + 1));
    }

    if index_pairs.is_empty() {
        return multiplicands.to_vec();
    }
    let (first_start, first_end) = index_pairs[0];
    // add in first batch of non-compressible integers
    let mut result = multiplicands[..first_start].to_vec();
    // add single instance of compressible integer
    result.push(multiplicands[first_start]);
    let mut last_end = first_end;
    for (start, end) in index_pairs.iter().skip(1) {
        result = [
            result,
            multiplicands[last_end..*start].to_vec(), // non-compressible integers
            vec![multiplicands[*start]],              // single instance of compressible integer
        ]
        .concat();
        last_end = *end;
    }
    result = [result, multiplicands[last_end..].to_vec()].concat();
    result
}

#[cfg(test)]
mod tests {
    use super::compress;
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let nums = [2, 3, -2, 4];

        // when
        let result = Solution::max_product(nums.to_vec());

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn example2() {
        // given
        let nums = [-2, 0, -1];

        // when
        let result = Solution::max_product(nums.to_vec());

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example155() {
        // given
        let nums = [-2];

        // when
        let result = Solution::max_product(nums.to_vec());

        // then
        assert_eq!(result, -2);
    }

    #[test]
    fn test_compress_full_input() {
        // given
        let original = [1, 1, 1, 1];

        // when
        let result = compress(&original);

        // then
        assert_eq!(result, [1].to_vec());
    }

    #[test]
    fn test_compress_prefix() {
        // given
        let original = [0, 0, 0, 0, 2, 4, 8, 16];

        // when
        let result = compress(&original);

        // then
        assert_eq!(result, [0, 2, 4, 8, 16].to_vec());
    }

    #[test]
    fn test_compress_suffix() {
        // given
        let original = [2, 4, 8, 16, 1, 1, 1, 1];

        // when
        let result = compress(&original);

        // then
        assert_eq!(result, [2, 4, 8, 16, 1].to_vec());
    }

    #[test]
    fn test_compress_infix() {
        // given
        let original = [-8, -4, -2, 0, 0, 0, 0, 2, 4, 8];

        // when
        let result = compress(&original);

        // then
        assert_eq!(result, [-8, -4, -2, 0, 2, 4, 8].to_vec());
    }
}
