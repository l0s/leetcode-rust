// 236. Lowest Common Ancestor of a Binary Tree
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree

use crate::tree_node::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref()?;
        let root = root.unwrap();
        let p = p.unwrap();
        let q = q.unwrap();

        if root == p {
            return Some(p);
        } else if root == q {
            return Some(q);
        }

        #[allow(clippy::mutable_key_type)]
        let mut parents = HashMap::new();
        // BFS to identify parents
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut found_p = false;
        let mut found_q = false;
        while let Some(node) = queue.pop_front() {
            found_p |= node == p;
            found_q |= node == q;
            if found_p && found_q {
                break;
            }
            if let Some(left) = &node.borrow().left {
                queue.push_back(left.clone());
                parents.insert(Key(left.clone()), node.clone());
            }
            if let Some(right) = &node.borrow().right {
                queue.push_back(right.clone());
                parents.insert(Key(right.clone()), node.clone());
            }
        }

        let p_lineage = lineage(p, &parents);
        let q_lineage = lineage(q, &parents);

        Some(lowest_common_ancestor(&p_lineage, &q_lineage))
    }
}

#[derive(Eq, PartialEq)]
struct Key(Rc<RefCell<TreeNode>>);

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.0.borrow().val)
    }
}

#[allow(clippy::mutable_key_type)]
fn lineage(
    node: Rc<RefCell<TreeNode>>,
    parents: &HashMap<Key, Rc<RefCell<TreeNode>>>,
) -> Vec<Rc<RefCell<TreeNode>>> {
    let mut result = Vec::new();
    result.insert(0, node.clone());
    let mut current = node.clone();
    while let Some(parent) = parents.get(&Key(current.clone())) {
        result.insert(0, parent.clone());
        current = parent.clone();
    }
    result
}

fn lowest_common_ancestor(
    x: &[Rc<RefCell<TreeNode>>],
    y: &[Rc<RefCell<TreeNode>>],
) -> Rc<RefCell<TreeNode>> {
    assert!(!x.is_empty());
    assert!(!y.is_empty());
    assert_eq!(x[0], y[0]); // should have same root

    let mut result = x[0].clone();
    for i in 0..x.len().min(y.len()) {
        if x[i] == y[i] {
            result = x[i].clone();
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use crate::tree_node::parse_binary_tree;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn example1() {
        // given
        let root = [
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];
        let p = 5;
        let q = 1;

        let root = parse_binary_tree(&root).unwrap();
        let p = root.borrow().get_node(p);
        let q = root.borrow().get_node(q);

        // when
        let result = Solution::lowest_common_ancestor(Some(root), p, q);

        // then
        assert_eq!(result.unwrap().borrow().val, 3);
    }

    #[test]
    fn example2() {
        // given
        let root = [
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];
        let p = 5;
        let q = 4;

        let root = parse_binary_tree(&root).unwrap();
        let p = root.borrow().get_node(p);
        let q = root.borrow().get_node(q);

        // when
        let result = Solution::lowest_common_ancestor(Some(root), p, q);

        // then
        assert_eq!(result.unwrap().borrow().val, 5);
    }

    #[test]
    fn example3() {
        // given
        let root = [Some(1), Some(2)];
        let p = 1;
        let q = 2;

        let root = parse_binary_tree(&root).unwrap();
        let p = root.borrow().get_node(p);
        let q = root.borrow().get_node(q);

        // when
        let result = Solution::lowest_common_ancestor(Some(root), p, q);

        // then
        assert_eq!(result.unwrap().borrow().val, 1);
    }

    impl Clone for TreeNode {
        fn clone(&self) -> Self {
            Self {
                val: self.val,
                left: self.left.clone(),
                right: self.right.clone(),
            }
        }
    }

    impl TreeNode {
        fn get_node(&self, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if self.val == value {
                return Some(Rc::new(RefCell::new(self.clone())));
            }
            if let Some(left) = &self.left {
                if let Some(result) = left.borrow().get_node(value) {
                    return Some(result.clone());
                }
            }
            if let Some(right) = &self.right {
                if let Some(result) = right.borrow().get_node(value) {
                    return Some(result.clone());
                }
            }
            None
        }
    }
}
