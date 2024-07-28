use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

pub fn parse_binary_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut itr = nodes.iter();

    if let Some(Some(root_value)) = itr.next() {
        let root = Rc::new(RefCell::new(TreeNode::new(*root_value)));

        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::with_capacity(nodes.len());
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
}
