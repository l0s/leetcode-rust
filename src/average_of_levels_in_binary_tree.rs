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
use std::collections::btree_map::Entry::{Occupied, Vacant};
use std::collections::BTreeMap;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut map = BTreeMap::default();
        build_map(root, 0, &mut map);
        map.iter()
            .map(|(_, value)| (value.1 as f64) / (value.0 as f64))
            .collect()
    }
}

fn build_map(
    root: Option<Rc<RefCell<TreeNode>>>,
    level: usize,
    result: &mut BTreeMap<usize, (usize, i64)>,
) {
    if let Some(root) = root {
        match result.entry(level) {
            Vacant(e) => {
                e.insert((1, root.borrow().val as i64));
            }
            Occupied(mut e) => {
                let mut bucket = e.get_mut();
                bucket.0 += 1;
                bucket.1 += root.borrow().val as i64;
            }
        };
        build_map(root.borrow().left.clone(), level + 1, result);
        build_map(root.borrow().right.clone(), level + 1, result);
    }
}
