// 98. Validate Binary Search Tree
// https://leetcode.com/problems/validate-binary-search-tree

pub struct Solution;

use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.expect("Root node is required")
            .borrow()
            .is_valid_bst()
            .is_ok()
    }
}

impl TreeNode {
    fn is_valid_bst(&self) -> Result<(i32, i32), String> {
        let mut min = self.val;
        let mut max = self.val;
        if let Some(left) = &self.left {
            let left = left.borrow();
            let (left_min, left_max) = left.is_valid_bst()?;
            if left_max < self.val {
                min = min.min(left_min);
            } else {
                return Err(format!(
                    "At node {}, the left branch has a larger value, {}",
                    self.val, left_max
                ));
            }
        }
        if let Some(right) = &self.right {
            let right = right.borrow();
            let (right_min, right_max) = right.is_valid_bst()?;
            if right_min > self.val {
                max = max.max(right_max);
            } else {
                return Err(format!(
                    "At node {}, right right branch has a smaller value, {}",
                    self.val, right_min
                ));
            }
        }
        Ok((min, max))
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;
    use crate::tree_node::parse_binary_tree;

    #[test]
    fn example1() {
        // given
        let root = parse_binary_tree(&[Some(2), Some(1), Some(3)]);

        // when
        let result = Solution::is_valid_bst(root);

        // then
        assert!(result);
    }

    #[test]
    fn example2() {
        // given
        let root = parse_binary_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);

        // when
        let result = Solution::is_valid_bst(root);

        // then
        assert!(!result);
    }

    #[test]
    fn example73() {
        // given
        let root = parse_binary_tree(&[Some(2), Some(2), Some(2)]);

        // when
        let result = Solution::is_valid_bst(root);

        // then
        assert!(!result);
    }
}
