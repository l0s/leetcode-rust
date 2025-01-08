// 310. Minimum Height Trees
// https://leetcode.com/problems/minimum-height-trees

pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph: Graph = edges.into();
        loop {
            if graph.neighbors.len() == 1 || graph.neighbors.len() == 2 {
                break;
            }
            let leaves = graph.leaves();
            if leaves.is_empty() {
                break;
            }
            for leaf in leaves {
                graph.remove(&leaf);
            }
        }
        graph.neighbors.keys().cloned().collect()
    }
}

struct Graph {
    neighbors: HashMap<i32, HashSet<i32>>,
}

impl From<Vec<Vec<i32>>> for Graph {
    fn from(value: Vec<Vec<i32>>) -> Self {
        let mut neighbors = HashMap::with_capacity(value.len() * 2);
        for pair in value {
            assert_eq!(pair.len(), 2);
            neighbors
                .entry(pair[0])
                .and_modify(|bucket: &mut HashSet<i32>| {
                    bucket.insert(pair[1]);
                })
                .or_insert_with(|| HashSet::from([pair[1]]));
            neighbors
                .entry(pair[1])
                .and_modify(|bucket| {
                    bucket.insert(pair[0]);
                })
                .or_insert_with(|| HashSet::from([pair[0]]));
        }
        Self { neighbors }
    }
}

impl Graph {
    fn leaves(&self) -> Vec<i32> {
        self.neighbors
            .iter()
            .filter_map(|(node, neighbors)| {
                if neighbors.len() <= 1 {
                    Some(*node)
                } else {
                    None
                }
            })
            .collect()
    }

    fn remove(&mut self, node: &i32) {
        if let Some(neighbors) = self.neighbors.get(node).cloned() {
            for neighbor in neighbors {
                if let Some(bucket) = self.neighbors.get_mut(&neighbor) {
                    bucket.remove(node);
                }
            }
        }
        self.neighbors.remove(node);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::matrix_util::array_to_vec;

    #[test]
    fn example1() {
        // given
        let n = 4;
        let edges = [[1, 0], [1, 2], [1, 3]];
        let edges = array_to_vec(&edges);

        // when
        let result = Solution::find_min_height_trees(n, edges);

        // then
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn example2() {
        // given
        let n = 6;
        let edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]];
        let edges = array_to_vec(&edges);

        // when
        let mut result = Solution::find_min_height_trees(n, edges);
        result.sort();

        // then
        assert_eq!(result, vec![3, 4]);
    }

    #[test]
    fn example71() {
        // given
        let n = 1;
        let edges = vec![];

        // when
        let result = Solution::find_min_height_trees(n, edges);

        // then
        assert_eq!(result, vec![0]);
    }
}
