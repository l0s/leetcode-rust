// 128. Longest Consecutive Sequence
// https://leetcode.com/problems/longest-consecutive-sequence/

pub struct Solution;

use std::collections::{HashMap, VecDeque};
use Entry::{Member, Root};

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        } else if nums.len() == 1 {
            return 1;
        }

        let mut disjoint_set = DisjointSet::default();
        let mut result = usize::MIN;
        for num in nums.into_iter() {
            let group_size = disjoint_set.insert(num);
            result = result.max(group_size);
        }

        result as i32
    }
}

enum Entry {
    Member { parent: i32 },
    Root { size: usize },
}

/// A union find data structure for grouping adjacent integers
#[derive(Default)]
struct DisjointSet {
    entries: HashMap<i32, Entry>,
}

impl DisjointSet {
    fn group_size(&self, entry: &Entry) -> usize {
        let mut cursor = entry;
        loop {
            match cursor {
                Member { parent } => {
                    let parent_entry = self.entries.get(parent).expect("Parent not found");
                    cursor = parent_entry;
                }
                Root { size } => {
                    return *size;
                }
            }
        }
    }

    /// Add a value to the set and merge with adjacent values
    ///
    /// Parameters:
    /// - `value` the new item to add to the set, which will be merged with adjacent items if any
    ///           exist.
    ///
    /// Returns:
    /// - The size of the group to which `value` was added
    pub fn insert(&mut self, value: i32) -> usize {
        if let Some(entry) = self.entries.get(&value) {
            return self.group_size(entry);
        }
        let mut root_value = value;
        let entry = Root { size: 1 };
        self.entries.insert(value, entry);
        if self.entries.contains_key(&(value - 1)) {
            root_value = self.merge(value - 1, root_value);
        }
        if self.entries.contains_key(&(value + 1)) {
            root_value = self.merge(root_value, value + 1);
        }
        if let Root { size } = self
            .entries
            .get(&root_value)
            .expect("Root entry is missing")
        {
            *size
        } else {
            panic!("Root entry is not not a Root type");
        }
    }

    /// Merge two groups together
    ///
    /// Parameters:
    /// - `x` a group member (may or may not be a root)
    /// - `y` a group member (may or may not be a root)
    ///
    /// Returns:
    /// - The root (representative member) of the combined group (may or may not be `x` or `y`)
    fn merge(&mut self, x: i32, y: i32) -> i32 {
        let (x_root, x_size) = self.find_root_and_compress(x);
        let (y_root, y_size) = self.find_root_and_compress(y);

        if x_root == y_root {
            return x_root;
        }

        // merge the smaller into the larger
        let (smaller_root, larger_root) = if x_size <= y_size {
            (x_root, y_root)
        } else {
            (y_root, x_root)
        };
        self.entries.insert(
            smaller_root,
            Member {
                parent: larger_root,
            },
        );
        self.entries.insert(
            larger_root,
            Root {
                size: x_size + y_size,
            },
        );
        larger_root
    }

    /// Find the root of a group (representative member) and ensure all the members in the path
    /// point directly to the root.
    ///
    /// Parameters:
    /// - `set` - the disjoint set to which `value` belongs
    /// - `value` - the member value
    ///
    /// Returns:
    /// - the value of the representative member of the group (Root node) and the size of the
    ///     group
    fn find_root_and_compress(&mut self, value: i32) -> (i32, usize) {
        let mut path = VecDeque::new();
        let mut root_value = value;
        let group_size: usize;
        loop {
            let entry = self
                .entries
                .get(&root_value)
                .unwrap_or_else(|| panic!("Value does not exist: {}", value));
            match entry {
                Member { parent } => {
                    path.push_back(root_value);
                    root_value = *parent;
                }
                Root { size } => {
                    group_size = *size;
                    break;
                }
            }
        }
        if !path.is_empty() {
            path.remove(path.len() - 1);
        }
        for value in &path {
            self.entries.insert(*value, Member { parent: root_value });
        }
        self.entries.insert(root_value, Root { size: group_size });
        (root_value, group_size)
    }
}

#[cfg(test)]
mod tests {
    use super::Entry::{Member, Root};
    use super::{DisjointSet, Solution};
    use std::collections::HashMap;

    #[test]
    fn example1() {
        // given
        let nums = [100, 4, 200, 1, 3, 2];

        // when
        let result = Solution::longest_consecutive(nums.to_vec());

        // then
        assert_eq!(result, 4);
    }

    #[test]
    fn example2() {
        // given
        let nums = [0, 3, 7, 2, 5, 8, 4, 6, 0, 1];

        // when
        let result = Solution::longest_consecutive(nums.to_vec());

        // then
        assert_eq!(result, 9);
    }

    #[test]
    fn example67() {
        // given
        let nums = [1, -8, 7, -2, -4, -4, 6, 3, -4, 0, -7, -1, 5, 1, -9, -3];

        // when
        let result = Solution::longest_consecutive(nums.to_vec());

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn insert_merges_with_indirect_member() {
        // given
        let entries = HashMap::from([
            (
                -4,
                Root {
                    // existing independent root
                    size: 1,
                },
            ),
            (
                -2,
                Member {
                    // existing member, indirect child of root
                    parent: -1,
                },
            ),
            (-1, Member { parent: 1 }),
            (0, Member { parent: 1 }),
            (1, Root { size: 4 }),
        ]);
        let mut set = DisjointSet { entries };

        // when
        let result = set.insert(-3);

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn merge_compresses_paths() {
        // given
        let entries = HashMap::from([
            (
                -4,
                Root {
                    // existing isolated root
                    size: 1,
                },
            ),
            (
                -3,
                Root {
                    // new entry
                    size: 1,
                },
            ),
            (
                -2,
                Member {
                    // existing member of group, indirect child of root
                    parent: -1,
                },
            ),
            (-1, Member { parent: 1 }),
            (0, Member { parent: 1 }),
            (
                1,
                Root {
                    // existing root
                    size: 4,
                },
            ),
        ]);
        let mut set = DisjointSet { entries };

        // when
        let intermediate_root = set.merge(-4, -3);

        // then
        assert_eq!(
            set.group_size(set.entries.get(&-4).expect("-4 has no entry")),
            2
        );
        assert_eq!(
            set.group_size(set.entries.get(&-3).expect("-3 has no entry")),
            2
        );
        assert!(intermediate_root == -4 || intermediate_root == -3);

        // when
        let final_root = set.merge(-3, -2);

        // then
        assert!(final_root == intermediate_root || final_root == 1);
        assert_eq!(
            set.group_size(set.entries.get(&-3).expect("-3 has no entry")),
            6
        );
        assert_eq!(
            set.group_size(set.entries.get(&-2).expect("-2 has no entry")),
            6
        );
    }

    #[test]
    fn find_and_compress_compresses_path() {
        // given
        let entries = HashMap::from([
            (-4, Member { parent: -3 }),
            (
                -3,
                Root {
                    // new entry, recently merged with -4
                    size: 2,
                },
            ),
            (
                -2,
                Member {
                    // existing member of group, not direct child of root
                    parent: -1,
                },
            ),
            (-1, Member { parent: 1 }),
            (0, Member { parent: 1 }),
            (
                1,
                Root {
                    // existing root
                    size: 4,
                },
            ),
        ]);
        let mut set = DisjointSet { entries };

        // when
        let (root, size) = set.find_root_and_compress(-2);

        // then
        let compressed_entry = set.entries.get(&-2).expect("No entry for -2");
        match compressed_entry {
            Member { parent } => assert_eq!(*parent, 1),
            Root { .. } => panic!("Expected direct Member of Root"),
        }
        assert_eq!(root, 1);
        assert_eq!(size, 4);
    }

    #[test]
    fn merge_sizes_group() {
        // given
        let mut entries = HashMap::new();
        entries.insert(2, Root { size: 2 });
        entries.insert(3, Member { parent: 2 });
        entries.insert(0, Root { size: 1 });
        entries.insert(5, Root { size: 1 });
        entries.insert(4, Root { size: 1 });
        let mut set = DisjointSet { entries };

        // when
        let root = set.merge(3, 4);

        // then
        assert!(root == 2 || root == 3 || root == 4);
        let root_entry = set.entries.get(&root).expect("Root entry not found");
        if let Root { size } = root_entry {
            assert_eq!(*size, 3);
        } else {
            panic!("Root entry is not of type root");
        }
    }

    #[test]
    fn find_and_compress_sizes_group() {
        // given
        let mut entries = HashMap::new();
        entries.insert(2, Root { size: 2 });
        entries.insert(3, Member { parent: 2 });
        entries.insert(0, Root { size: 1 });
        entries.insert(5, Root { size: 1 });
        entries.insert(4, Root { size: 1 });
        let mut set = DisjointSet { entries };

        // when / then
        let (x_root, x_size) = set.find_root_and_compress(3);
        assert_eq!(x_root, 2);
        assert_eq!(x_size, 2);
        let (y_root, y_size) = set.find_root_and_compress(4);
        assert_eq!(y_root, 4);
        assert_eq!(y_size, 1);
    }
}
