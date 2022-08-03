// 2096. Step-By-Step Directions From a Binary Tree Node to Another
// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/

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

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        let graph: Graph = root.unwrap().into();
        graph.search(start_value, dest_value)
    }
}

pub struct Graph {
    up: HashMap<i32, i32>,
    left: HashMap<i32, i32>,
    right: HashMap<i32, i32>,
    vertices: HashSet<i32>,
}

impl Graph {
    pub fn search(&self, start: i32, destination: i32) -> String {
        // assert!(self.vertices.contains(&start));
        // assert!(self.vertices.contains(&destination));
        let mut distance = HashMap::new();
        let mut previous = HashMap::new();
        let mut move_to = HashMap::new();
        let mut queue = Vec::new(); // manually order in descending order of distance
        for vertex in &self.vertices {
            distance.insert(*vertex, usize::MAX);
            if *vertex != start {
                queue.push(*vertex);
            }
        }
        // give "start" the highest priority, lowest distance
        distance.insert(start, 0);
        queue.push(start);
        while let Some(vertex) = queue.pop() {
            if vertex == destination {
                eprintln!("Arrived at {}", vertex);
                let mut path = vec![];
                let mut cursor = vertex;
                while cursor != start {
                    path.insert(0,
                        *move_to.get(&cursor).expect(&format!("Not sure how we got to: {}", cursor)));
                    cursor = *previous.get(&cursor).expect(&format!("Not sure which node preceded: {}", cursor));
                }
                return path.iter().collect::<String>();
            }
            let mut neighbours = vec![];
            if let Some(neighbour) = self.up.get(&vertex) {
                neighbours.push((*neighbour, 'U'));
            }
            if let Some(neighbour) = self.left.get(&vertex) {
                neighbours.push((*neighbour, 'L'));
            }
            if let Some(neighbour) = self.right.get(&vertex) {
                neighbours.push((*neighbour, 'R'));
            }
            let vertex_distance = if *distance.get(&vertex).unwrap() == usize::MAX { usize::MAX } else {distance.get(&vertex).unwrap() + 1};
            for neighbour in neighbours {
                let neighbour_distance = vertex_distance + 1;
                if neighbour_distance < *distance.get(&neighbour.0).unwrap() {
                    distance.insert(neighbour.0, neighbour_distance);
                    previous.insert(neighbour.0, vertex);
                    move_to.insert(neighbour.0, neighbour.1);
                    eprintln!("Linked {} -> {}", vertex, neighbour.0);
                }
            }
            // sort in descending order of distance
            // O( n log n ) but only at most three items moved
            // similar to a "heapify" operation
            queue.sort_by(|x, y| distance.get(y).cmp(&distance.get(x)));
        }

        String::new()
    }
}

impl From<Rc<RefCell<TreeNode>>> for Graph {
    fn from(root: Rc<RefCell<TreeNode>>) -> Self {
        let mut up = HashMap::new();
        let mut left = HashMap::new();
        let mut right = HashMap::new();
        let mut vertices = HashSet::new();

        let mut discovered = HashSet::new();
        let mut stack = vec![];
        stack.push(root.clone());
        while let Some(node) = stack.pop() {
            let node_refcell = &*node;
            let node_ref = node_refcell.borrow();
            let value = node_ref.val;
            if !discovered.contains(&value) {
                discovered.insert(value);
                vertices.insert(value);
                if let Some(neighbor) = &node_ref.left {
                    let neighbor = neighbor.clone();
                    let neighbor_val = (*neighbor).borrow().val;
                    left.insert(node_ref.val, neighbor_val);
                    up.insert(neighbor_val, node_ref.val);
                    stack.push(neighbor.clone());
                }
                if let Some(neighbor) = &node_ref.right {
                    let neighbor = neighbor.clone();
                    let neighbor_val = (*neighbor).borrow().val;
                    right.insert(node_ref.val, neighbor_val);
                    up.insert(neighbor_val, node_ref.val);
                    stack.push(neighbor.clone());
                }
            }
        }
        Self {
            up,
            left,
            right,
            vertices,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::step_by_step_directions_from_a_binary_tree_node_to_another::TreeNode;
    use super::Solution;
    use serde_json;
    use std::fs;

    fn convert(slice: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        eprintln!("slice size: {}", slice.len());
        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = slice.iter()
            .map(|entry| -> Option<Rc<RefCell<TreeNode>>> {
                if let Some(value) = entry {
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val: *value,
                        left: None,
                        right: None,
                    })));
                }
                None
            }).collect();
        eprintln!("num nodes: {}", nodes.len());
        for (index, node) in nodes.iter().enumerate() {
            let left_index = (index * 2) + 1;
            let right_index = (index * 2) + 2;

            if node.is_none() {
                assert!(left_index >= nodes.len() || nodes.get(left_index).unwrap().is_none(),
                "index {} is None but has left child at index {} with value {:?}", index, left_index, nodes.get(left_index).unwrap());
                assert!(right_index >= nodes.len() || nodes.get(right_index).unwrap().is_none(),
                "index {} is None but has right child at index {} with value {:?}", index, right_index, nodes.get(right_index).unwrap());
                continue;
            }
            let node = node.clone();
            let node = node.unwrap();
            let mut node = node.borrow_mut();

            if let Some(left) = nodes.get(left_index) {
                node.left = left.to_owned();
            }
            if let Some(right) = nodes.get(right_index) {
                node.right = right.to_owned();
            }
        }
        nodes.get(0).unwrap().clone()
    }

    impl TreeNode {
        pub fn contains(&self, value: &i32) -> bool {
            if self.val == *value {
                return true;
            }
            if let Some(right) = self.right.clone() {
                if right.as_ref().borrow().contains(value){
                    return true;
                }
            }
            if let Some(left) = self.left.clone() {
                if left.as_ref().borrow().contains(value){
                    return true;
                }
            }
            false
        }

        pub fn size(&self) -> usize {
            let mut result = 1usize;
            if let Some(right) = self.right.clone() {
                result += right.as_ref().borrow().size();
            }
            if let Some(left) = self.left.clone() {
                result += left.as_ref().borrow().size();
            }
            result
        }
    }

    #[test]
    pub fn example1() {
        // given
        let root = vec![Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)];
        let root = convert(root);
        let start_value = 3;
        let dest_value = 6;

        // when
        let result = Solution::get_directions(root, start_value, dest_value);

        // then
        assert_eq!(result, "UURL");
    }

    #[test]
    pub fn example2() {
        // given
        let root = vec![Some(2), Some(1)];
        let root = convert(root);
        let start_value = 2;
        let dest_value = 1;

        // when
        let result = Solution::get_directions(root, start_value, dest_value);

        // then
        assert_eq!(result, "L");
    }

    #[test]
    pub fn example3() {
        // given
        let start_value = 29716;
        let dest_value = 54117;
        let string = fs::read_to_string("input/directions/example3.json").expect("Cannot find JSON");
        let root: Vec<Option<i32>> = serde_json::from_str(&string).unwrap();
        assert!(root.contains(&Some(dest_value)));
        assert_eq!(*root.get(root.len() - 1).unwrap(), Some(dest_value));
        eprintln!("Parsed JSON has {} array elements", root.len());
        let root = convert(root);
        // assert_eq!(root.clone().unwrap().as_ref().borrow().size(), 100_000);
        // assert!(root.clone().unwrap().as_ref().borrow().contains(&dest_value));

        // when
        let result = Solution::get_directions(root, start_value, dest_value);
        eprintln!("Path from {} to {} is {}", start_value, dest_value, result);

        // then
        assert_eq!(result, "RRRRRRRRRRRRRRRRR");
    }

}