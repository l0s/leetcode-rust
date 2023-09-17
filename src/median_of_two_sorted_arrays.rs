// 4. Median of Two Sorted Arrays
// https://leetcode.com/problems/median-of-two-sorted-arrays/

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        let mut iter1 = nums1.iter();
        let mut iter2 = nums2.iter();

        let mut x = iter1.next();
        let mut y = iter2.next();

        let mut merged = Vec::with_capacity(len);
        while x.is_some() && y.is_some() {
            if x.unwrap() < y.unwrap() {
                merged.push(x.unwrap());
                x = iter1.next();
            } else {
                merged.push(y.unwrap());
                y = iter2.next();
            }
            if merged.len() == len / 2 + 1 {
                return if len % 2 == 0 {
                    (*merged[merged.len() - 1] as f64 + *merged[merged.len() - 2] as f64) / 2f64
                } else {
                    *merged[merged.len() - 1] as f64
                };
            }
        }
        while x.is_some() {
            merged.push(x.unwrap());
            x = iter1.next();
        }
        while y.is_some() {
            merged.push(y.unwrap());
            y = iter2.next();
        }

        if len % 2 == 0 {
            let x = *merged[len / 2 - 1] as f64;
            let y = *merged[len / 2] as f64;
            (x + y) / 2f64
        } else {
            *merged[len / 2] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::median_of_two_sorted_arrays::Solution;

    #[test]
    pub fn example1() {
        // given
        let nums1 = vec![1,3];
        let nums2 = vec![2];

        // when
        let result = Solution::find_median_sorted_arrays(nums1, nums2);

        // then
        assert!((result - 2f64).abs() < 0.000001, "Expected {}, but got {}", 2f64, result);
    }

    #[test]
    pub fn example2() {
        // given
        let nums1 = vec![1,2];
        let nums2 = vec![3,4];

        // when
        let result = Solution::find_median_sorted_arrays(nums1, nums2);

        // then
        assert!((result - 2.5f64).abs() < 0.000001, "Expected {}, but got {}", 2.5f64, result);
    }
}