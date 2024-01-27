// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/

pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    /// Parse LeetCode's representation of a tree
    pub fn parse(array: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = array.len();
        if len == 0 {
            return None;
        }
        let mut owned_nodes = array
            .iter()
            .map(|value| value.map(|value| Rc::new(RefCell::new(TreeNode::new(value)))))
            .collect::<Vec<Option<Rc<RefCell<TreeNode>>>>>();
        let mut has_left_child = Vec::with_capacity(len / 2);
        let mut has_right_child = Vec::with_capacity(len / 2);
        for (index, node) in owned_nodes.iter().enumerate() {
            if node.is_some() {
                let left_child_index = index * 2 + 1;
                if let Some(child) = owned_nodes.get(left_child_index) {
                    if child.is_some() {
                        has_left_child.push(index);
                    }
                }
                let right_child_index = index * 2 + 2;
                if let Some(child) = owned_nodes.get(right_child_index) {
                    if child.is_some() {
                        has_right_child.push(index);
                    }
                }
            }
        }

        for parent_index in has_left_child {
            let child_index = parent_index * 2 + 1;
            let child = owned_nodes.get(child_index).unwrap().clone();
            let mut parent = owned_nodes
                .get_mut(parent_index)
                .unwrap()
                .as_mut()
                .unwrap()
                .borrow_mut();
            parent.left = child;
        }
        for parent_index in has_right_child {
            let child_index = parent_index * 2 + 2;
            let child = owned_nodes.get(child_index).unwrap().clone();
            let mut parent = owned_nodes
                .get_mut(parent_index)
                .unwrap()
                .as_mut()
                .unwrap()
                .borrow_mut();
            parent.right = child;
        }

        owned_nodes[0].clone()
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        max_depth(&root, 0) as i32
    }
}

fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> usize {
    if let Some(root) = root {
        let root = root.borrow();
        let left = max_depth(&root.left, depth + 1);
        let right = max_depth(&root.right, depth + 1);
        left.max(right)
    } else {
        depth
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;

    #[test]
    pub fn example1() {
        // given
        let root = TreeNode::parse(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        // when
        let result = Solution::max_depth(root);

        // then
        assert_eq!(result, 3);
    }

    #[test]
    pub fn example2() {
        // given
        let root = TreeNode::parse(&[Some(1), None, Some(2)]);

        // when
        let result = Solution::max_depth(root);

        // then
        assert_eq!(result, 2);
    }
}
