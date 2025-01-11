// 230. Kth Smallest Element in a BST
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/

pub struct Solution;

use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if root.is_none() {
            return -1;
        }
        let mut cursor = root.clone();
        let mut stack = vec![];
        let mut encountered = 0usize;
        while !stack.is_empty() || cursor.is_some() {
            while let Some(node) = cursor.clone() {
                stack.push(Some(node.clone()));
                cursor = node.borrow().left.clone();
            }
            if let Some(Some(node)) = stack.pop() {
                encountered += 1;
                if encountered == k as usize {
                    return node.borrow().val;
                }
                cursor = node.borrow().right.clone();
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_node::parse_binary_tree;

    #[test]
    fn example1() {
        // given
        let root = [Some(3), Some(1), Some(4), None, Some(2)];
        let k = 1;

        // when
        let result = Solution::kth_smallest(parse_binary_tree(&root), k);

        // then
        assert_eq!(result, 1);
    }

    #[test]
    fn example2() {
        // given
        let root = [
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ];
        let k = 3;

        // when
        let result = Solution::kth_smallest(parse_binary_tree(&root), k);

        // then
        assert_eq!(result, 3);
    }
}
