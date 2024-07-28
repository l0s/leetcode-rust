// 102. Binary Tree Level Order Traversal
// https://leetcode.com/problems/binary-tree-level-order-traversal

pub struct Solution;

use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut to_visit = VecDeque::new();
        if let Some(root) = root {
            to_visit.push_back((root.clone(), 0usize));
            while let Some((node, level)) = to_visit.pop_front() {
                let node = node.borrow();
                match level.cmp(&result.len()) {
                    Ordering::Less => result[level].push(node.val),
                    Ordering::Equal => result.push(vec![node.val]),
                    Ordering::Greater => panic!(
                        "Cannot add level {} as there are only {} levels so far.",
                        level,
                        result.len()
                    ),
                }
                if let Some(left_child) = &node.left {
                    to_visit.push_back((left_child.clone(), level + 1));
                }
                if let Some(right_child) = &node.right {
                    to_visit.push_back((right_child.clone(), level + 1));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_node::parse_binary_tree;

    #[test]
    fn example1() {
        // given
        let root = parse_binary_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        // when
        let result = Solution::level_order(root);

        // then
        assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn example2() {
        // given
        let root = parse_binary_tree(&[Some(1)]);

        // when
        let result = Solution::level_order(root);

        // then
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn example3() {
        // given
        let root = parse_binary_tree(&[]);

        // when
        let result = Solution::level_order(root);

        // then
        assert!(result.is_empty())
    }
}
