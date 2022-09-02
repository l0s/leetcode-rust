// 637. Average of Levels in Binary Tree
// From: https://leetcode.com/problems/average-of-levels-in-binary-tree/

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
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut map = Vec::default();
        build_map(&root, 0, &mut map);
        map.iter()
            .map(|value| (value.1 as f64) / (value.0 as f64))
            .collect()
    }
}

fn build_map(root: &Option<Rc<RefCell<TreeNode>>>, level: usize, result: &mut Vec<(usize, i64)>) {
    if let Some(root) = root {
        let root = root.borrow();
        if let Some((count, sum)) = result.get_mut(level) {
            *count += 1;
            *sum += root.val as i64;
        } else {
            result.push((1, root.val as i64));
        }
        build_map(&root.left, level + 1, result);
        build_map(&root.right, level + 1, result);
    }
}
