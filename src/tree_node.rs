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

pub fn serialize_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    let mut length = 0usize;
    while let Some(node) = queue.pop_front() {
        if let Some(node) = node {
            let node = node.borrow();
            result.push(Some(node.val));
            length = result.len();
            queue.push_back(node.left.clone());
            queue.push_back(node.right.clone());
        } else {
            result.push(None);
        }
    }
    result.truncate(length);

    result
}

#[cfg(test)]
mod tests {
    use super::{serialize_binary_tree, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn serialize_example1() {
        // given
        let mut root = TreeNode::new(1);
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(left)));
        root.right = Some(Rc::new(RefCell::new(right)));

        // when
        let result = serialize_binary_tree(Some(Rc::new(RefCell::new(root))));

        // then
        assert_eq!(result, vec![Some(1), Some(2), Some(3)]);
    }

    #[test]
    fn serialize_example2() {
        // given
        let mut level_0 = TreeNode::new(1);
        let level_1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let level_2 = Rc::new(RefCell::new(TreeNode::new(3)));

        level_0.right = Some(level_1.clone());
        level_1.borrow_mut().left = Some(level_2.clone());

        // when
        let result = serialize_binary_tree(Some(Rc::new(RefCell::new(level_0))));

        // then
        assert_eq!(result, vec![Some(1), None, Some(2), Some(3)]);
    }

    #[test]
    fn serialize_example3() {
        // given
        let five = Rc::new(RefCell::new(TreeNode::new(5)));
        let four = Rc::new(RefCell::new(TreeNode::new(4)));
        let three = Rc::new(RefCell::new(TreeNode::new(3)));
        let negative_one = Rc::new(RefCell::new(TreeNode::new(-1)));
        let seven = Rc::new(RefCell::new(TreeNode::new(7)));
        let two = Rc::new(RefCell::new(TreeNode::new(2)));
        let nine = Rc::new(RefCell::new(TreeNode::new(9)));

        five.borrow_mut().left = Some(four.clone());
        four.borrow_mut().left = Some(three.clone());
        three.borrow_mut().left = Some(negative_one.clone());
        five.borrow_mut().right = Some(seven.clone());
        seven.borrow_mut().left = Some(two.clone());
        two.borrow_mut().left = Some(nine);

        // when
        let result = serialize_binary_tree(Some(five));

        // then
        assert_eq!(
            result,
            vec![
                Some(5),
                Some(4),
                Some(7),
                Some(3),
                None,
                Some(2),
                None,
                Some(-1),
                None,
                Some(9)
            ]
        );
    }
}
