// 105. Construct Binary Tree from Preorder and Inorder Traversal
// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal

pub struct Solution;

use crate::tree_node::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut value_to_inorder_index = HashMap::new();
        for (index, value) in inorder.iter().enumerate() {
            value_to_inorder_index.insert(*value, index);
        }
        build(
            &value_to_inorder_index,
            &preorder,
            Rc::new(RefCell::new(0)),
            0,
            (inorder.len() - 1) as i64,
        )
    }
}

fn build(
    value_to_inorder_index: &HashMap<i32, usize>,
    preorder: &[i32],
    preorder_index: Rc<RefCell<usize>>,
    inorder_lower_bound: i64,
    inorder_upper_bound: i64,
) -> Option<Rc<RefCell<TreeNode>>> {
    if inorder_lower_bound > inorder_upper_bound {
        return None;
    }

    let root_value = preorder[*preorder_index.borrow()];
    *preorder_index.borrow_mut() += 1;
    let mut root_node = TreeNode::new(root_value);
    let root_index = *value_to_inorder_index
        .get(&root_value)
        .expect("Root value not found in inorder traversal") as i64;
    root_node.left = build(
        value_to_inorder_index,
        preorder,
        preorder_index.clone(),
        inorder_lower_bound,
        root_index - 1,
    );
    root_node.right = build(
        value_to_inorder_index,
        preorder,
        preorder_index.clone(),
        root_index + 1,
        inorder_upper_bound,
    );

    Some(Rc::new(RefCell::new(root_node)))
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::tree_node::parse_binary_tree;

    #[test]
    fn example1() {
        // given
        let preorder = [3, 9, 20, 15, 7];
        let inorder = [9, 3, 15, 20, 7];

        // when
        let result = Solution::build_tree(preorder.to_vec(), inorder.to_vec());

        // then
        assert_eq!(
            result,
            parse_binary_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)])
        );
    }

    #[test]
    fn example2() {
        // given
        let preorder = [-1];
        let inorder = [-1];

        // when
        let result = Solution::build_tree(preorder.to_vec(), inorder.to_vec());

        // then
        assert_eq!(result, parse_binary_tree(&[Some(-1)]));
    }
}
