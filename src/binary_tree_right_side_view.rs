// 199. Binary Tree Right Side View
// https://leetcode.com/problems/binary-tree-right-side-view

pub struct Solution;

use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        // post order DFS to find right-most nodes at each level
        let mut processing_stack = VecDeque::new();
        let mut output_stack = VecDeque::new();
        processing_stack.push_back((root.unwrap().clone(), 0usize));
        let mut max_level = 0usize;
        while let Some((node, level)) = processing_stack.pop_back() {
            output_stack.push_back((node.clone(), level));
            max_level = max_level.max(level);
            let node = node.borrow();
            if let Some(left) = &node.left {
                processing_stack.push_back((left.clone(), level + 1));
            }
            if let Some(right) = &node.right {
                processing_stack.push_back((right.clone(), level + 1));
            }
        }
        let mut right_most = vec![0; max_level + 1];
        while let Some((node, level)) = output_stack.pop_back() {
            right_most[level] = node.borrow().val;
        }

        right_most
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_node::parse_binary_tree;

    #[test]
    fn example1() {
        // given
        let root = [Some(1), Some(2), Some(3), None, Some(5), None, Some(4)];
        let root = parse_binary_tree(&root);

        // when
        let result = Solution::right_side_view(root);

        // then
        assert_eq!(result, [1, 3, 4].to_vec());
    }

    #[test]
    fn example2() {
        // given
        let root = [Some(1), None, Some(3)];
        let root = parse_binary_tree(&root);

        // when
        let result = Solution::right_side_view(root);

        // then
        assert_eq!(result, [1, 3].to_vec());
    }

    #[test]
    fn example3() {
        // given
        let root = [];
        let root = parse_binary_tree(&root);

        // when
        let result = Solution::right_side_view(root);

        // then
        assert!(result.is_empty());
    }

    #[test]
    fn example38() {
        // given
        let root = [Some(1), Some(2)];
        let root = parse_binary_tree(&root);

        // when
        let result = Solution::right_side_view(root);

        // then
        assert_eq!(result, [1, 2].to_vec());
    }
}
