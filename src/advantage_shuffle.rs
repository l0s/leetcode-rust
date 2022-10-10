// 870. Advantage Shuffle
// From: https://leetcode.com/problems/advantage-shuffle/

pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut reference_map: BTreeMap<i32, Vec<usize>> = BTreeMap::default();
        for (index, value) in nums2.iter().enumerate() {
            // O( n )
            reference_map.entry(*value).or_default().push(index);
        }
        let mut numbers_to_permute = nums1;
        numbers_to_permute.sort(); // O( n lg n )
        let mut result = vec![0; nums2.len()];
        for (value, indices) in reference_map.iter() {
            // O( n ) for both loops together
            for index in indices {
                // Find a number that is just barely larger
                let index_to_use = match numbers_to_permute.binary_search(&(value + 1)) {
                    // O( lg n )
                    Ok(index) => {
                        // best case scenario, this is exactly one more than the reference number
                        index
                    }
                    Err(index) => {
                        if index >= numbers_to_permute.len() {
                            // no larger number found, just use the smallest available
                            0
                        } else {
                            // this is the smallest value larger than the reference
                            index
                        }
                    }
                };
                result[*index] = numbers_to_permute[index_to_use];
                numbers_to_permute.remove(index_to_use);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn advantage(nums1: &[i32], nums2: &[i32]) -> usize {
        let mut result = 0;
        for i in 0..nums1.len() {
            if nums1[i] > nums2[i] {
                result += 1;
            }
        }
        result
    }

    #[test]
    pub fn example1() {
        // given
        let nums1 = vec![2, 7, 11, 15];
        let nums2 = vec![1, 10, 4, 11];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![2, 11, 7, 15], &nums2)
        );
    }

    #[test]
    pub fn example2() {
        // given
        let nums1 = vec![12, 24, 8, 32];
        let nums2 = vec![13, 25, 32, 11];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![24, 32, 8, 12], &nums2)
        );
    }

    #[test]
    pub fn example14() {
        // given
        let nums1 = vec![8, 2, 4, 4, 5, 6, 6, 0, 4, 7];
        let nums2 = vec![0, 8, 7, 4, 4, 2, 8, 5, 2, 0];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![2, 4, 8, 5, 6, 4, 0, 6, 4, 7], &nums2)
        );
    }

    #[test]
    pub fn example16() {
        // given
        let nums1 = vec![
            817556235, 232246148, 424291784, 13856114, 113666578, 513996259, 791310549, 341026786,
            319348491, 40812144,
        ];
        let nums2 = vec![
            99616046, 148514886, 206257807, 569848817, 358893433, 977727091, 284109958, 856279522,
            91774240, 527604738,
        ];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(
                &vec![
                    232246148, 424291784, 341026786, 817556235, 513996259, 13856114, 319348491,
                    40812144, 113666578, 791310549
                ],
                &nums2
            )
        );
    }

    #[test]
    pub fn example23() {
        // given
        let nums1 = vec![15, 15, 4, 5, 0, 1, 7, 10, 3, 1, 10, 10, 8, 2, 3];
        let nums2 = vec![4, 13, 14, 0, 14, 14, 12, 3, 15, 12, 2, 0, 6, 9, 0];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(
                &vec![5, 15, 15, 4, 0, 1, 7, 10, 3, 1, 10, 2, 8, 10, 3],
                &nums2
            )
        );
    }

    #[test]
    pub fn example29() {
        // given
        let nums1 = vec![7, 17, 18, 11, 13, 12, 0, 9, 3, 16];
        let nums2 = vec![9, 0, 11, 16, 18, 13, 12, 17, 7, 3];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(&vec![11, 3, 12, 17, 0, 16, 13, 18, 9, 7], &nums2)
        );
    }

    #[test]
    pub fn example30() {
        // given
        let nums1 = vec![25, 9, 17, 6, 2, 14, 3, 24, 18, 15, 10, 27, 11, 4, 13];
        let nums2 = vec![15, 4, 13, 24, 3, 11, 10, 9, 27, 17, 2, 6, 14, 18, 25];

        // when
        let result = Solution::advantage_count(nums1, nums2.clone());

        // then
        assert_eq!(
            advantage(&result, &nums2),
            advantage(
                &vec![17, 6, 14, 25, 4, 13, 11, 10, 2, 18, 3, 9, 15, 24, 27],
                &nums2
            )
        );
    }
}
