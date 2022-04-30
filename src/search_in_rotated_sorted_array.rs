// https://leetcode.com/problems/search-in-rotated-sorted-array/
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // find the pivot point so we can do a simple binary search
        let pivot = Self::pivot(&nums);
        let (left, right) = nums.split_at(pivot);
        // figure out which slice has our target and that slice's offset from the original list
        let (search_space, offset) =
            if !left.is_empty() && target >= left[0] && target <= left[left.len() - 1] {
                (left, 0i32)
            } else if !right.is_empty() && target >= right[0] && target <= right[right.len() - 1] {
                (right, pivot as i32)
            } else {
                (&[] as &[i32], 0i32)
            };
        // find the index in the slice in O( lg n ) time
        if let Ok(index) = search_space.binary_search(&target) {
            return index as i32 + offset;
        }
        -1
    }

    /// Find the pivot point in O( lg n ) time
    /// Params:
    /// - `nums` - a slice of distinct integers that are sorted and possibly rotated
    /// Returns: a value "pivot" such that `nums[..pivot]` and `nums[pivot..]` are each sorted
    fn pivot(nums: &[i32]) -> usize {
        if nums.len() == 1 {
            return 0;
        }
        let mut first_index = 0usize;
        let mut last_index = nums.len() - 1;
        while first_index < last_index {
            let first = nums[first_index];
            let last = nums[last_index];
            if last_index - first_index == 1 {
                return if first < last {
                    // already sorted
                    0usize
                } else {
                    // this is the pivot point
                    last_index
                };
            }
            if first < last {
                // full list is already sorted
                return 0;
            }
            // cut the search space in half
            let mid_index = ((last_index - first_index) / 2) + first_index;
            if mid_index - first_index >= 1 && first > nums[mid_index] {
                last_index = mid_index;
            } else {
                first_index = mid_index;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn pivot0() {
        // given
        let nums = vec![4, 5, 6, 7, 0, 1, 2];

        // when
        let result = Solution::pivot(&nums);

        // then
        assert_eq!(result, 4);
    }

    #[test]
    pub fn example1() {
        // given
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;

        // when
        let result = Solution::search(nums, target);

        // then
        assert_eq!(result, 4);
    }

    #[test]
    pub fn example2() {
        // given
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;

        // when
        let result = Solution::search(nums, target);

        // then
        assert_eq!(result, -1);
    }

    #[test]
    pub fn example3() {
        // given
        let nums = vec![1];
        let target = 0;

        // when
        let result = Solution::search(nums, target);

        // then
        assert_eq!(result, -1);
    }

    #[test]
    pub fn example4() {
        // given
        let nums = vec![1, 3];
        let target = 1;

        // when
        let result = Solution::search(nums, target);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    pub fn example5() {
        // given
        let nums = vec![3, 1];
        let target = 3;

        // when
        let result = Solution::search(nums, target);

        // then
        assert_eq!(result, 0);
    }

    #[test]
    pub fn example6() {
        // given
        let nums = vec![3, 5, 1];
        let target = 3;

        // when
        let result = Solution::search(nums, target);

        // then
        assert_eq!(result, 0);
    }
}
