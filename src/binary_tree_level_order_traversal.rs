// 102. Binary Tree Level Order Traversal
// https://leetcode.com/problems/binary-tree-level-order-traversal

pub struct Solution;

// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq)]
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
}

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
    use super::{Solution, TreeNode};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    #[test]
    fn example1() {
        // given
        let root = parse(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        // when
        let result = Solution::level_order(root);

        // then
        assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn example2() {
        // given
        let root = parse(&[Some(1)]);

        // when
        let result = Solution::level_order(root);

        // then
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn example3() {
        // given
        let root = parse(&[]);

        // when
        let result = Solution::level_order(root);

        // then
        assert!(result.is_empty())
    }

    fn parse(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut itr = nodes.iter();

        if let Some(root_container) = itr.next() {
            if let Some(root_value) = root_container {
                let root = Rc::new(RefCell::new(TreeNode::new(*root_value)));

                let mut queue: VecDeque<Rc<RefCell<TreeNode>>> =
                    VecDeque::with_capacity(nodes.len());
                queue.push_back(root.clone());

                while let Some(node) = queue.pop_front() {
                    // left
                    if let Some(left_container) = itr.next() {
                        if let Some(left_value) = left_container {
                            let left_child = Rc::new(RefCell::new(TreeNode::new(*left_value)));
                            node.borrow_mut().left = Some(left_child.clone());
                            queue.push_back(left_child);
                        } else {
                            node.borrow_mut().left = None;
                        }
                    } else {
                        break; // end of array, no more nodes
                    }
                    // right
                    if let Some(right_container) = itr.next() {
                        if let Some(right_value) = right_container {
                            let right_child = Rc::new(RefCell::new(TreeNode::new(*right_value)));
                            node.borrow_mut().right = Some(right_child.clone());
                            queue.push_back(right_child);
                        } else {
                            node.borrow_mut().right = None;
                        }
                    } else {
                        break; // end of array, no more nodes
                    }
                }

                Some(root)
            } else {
                None
            }
        } else {
            None
        }
    }
}
