// 146. LRU Cache
// https://leetcode.com/problems/lru-cache/

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

pub struct LRUCache {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    /// The most-recently-used item
    head: Option<Rc<RefCell<Node>>>,
    /// The least-recently-used item
    tail: Option<Rc<RefCell<Node>>>,
    capacity: usize,
}

/// An entry in a doubly-linked list
#[derive(Eq, Clone)]
struct Node {
    key: i32,
    value: i32,
    previous: Option<Rc<RefCell<Self>>>,
    next: Option<Rc<RefCell<Self>>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::with_capacity(capacity as usize),
            head: None,
            tail: None,
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            let result = node.clone().borrow().value;
            self.mark_recently_used(node.clone());
            return result;
        }
        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let mut most_recently_used = None;
        match self.map.entry(key) {
            Entry::Occupied(occupied_entry) => {
                let node = occupied_entry.get();
                // update value
                node.clone().borrow_mut().value = value;
                most_recently_used = Some(node.clone());
            }
            Entry::Vacant(vacant_entry) => {
                // insert value
                let node = Rc::new(RefCell::new(Node {
                    key,
                    value,
                    previous: None, // this will be the new head
                    next: self.head.clone(),
                }));
                vacant_entry.insert(node.clone());
                if self.map.len() == 1 {
                    // if adding the first element, also mark it as the tail
                    self.tail = Some(node.clone());
                }
                if let Some(current_head) = self.head.clone() {
                    current_head.borrow_mut().previous = Some(node.clone());
                }
                self.head = Some(node.clone());

                // if necessary, evict least-recently used item
                if self.map.len() > self.capacity {
                    let least_recently_used = self
                        .tail
                        .clone()
                        .expect("Least recently-used item should exist");
                    self.map.remove(&least_recently_used.clone().borrow().key);
                    if let Some(previous) = least_recently_used.clone().borrow().previous.clone() {
                        previous.borrow_mut().next = None;
                    }
                    self.tail = least_recently_used.clone().borrow().previous.clone();
                }

                if self.map.is_empty() {
                    self.head = None;
                    self.tail = None;
                }
            }
        }
        if let Some(node) = most_recently_used {
            self.mark_recently_used(node.clone());
        }
    }

    fn mark_recently_used(&mut self, node: Rc<RefCell<Node>>) {
        let mut update_head = false;
        if let Some(previous) = node.borrow().previous.clone() {
            previous.borrow_mut().next = node.borrow().next.clone();
            // If this was the least-recently-used item, be sure to update the tail
            if self
                .tail
                .clone()
                .expect("non-empty cache should have a least-recently-used item")
                == node
            {
                self.tail = Some(previous.clone());
            } else {
                node.borrow()
                    .next
                    .clone()
                    .expect("non-tail node should have a subsequent item")
                    .borrow_mut()
                    .previous = Some(previous.clone());
            }
            update_head |= true;
        } // else entry was already the most-recently-used
        if update_head {
            node.borrow_mut().next = self.head.clone();
            node.borrow_mut().previous = None;
            self.head
                .clone()
                .expect("non-empty cache should have a most-recently-used item")
                .borrow_mut()
                .previous = Some(node.clone());
            self.head = Some(node.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LRUCache, Node};
    use std::fmt::{Debug, Formatter};
    use std::str::FromStr;

    #[test]
    fn example1() {
        // given
        let operations = [
            "LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get",
        ]
        .map(|s| TestOperation::from_str(s).unwrap());
        let arguments = vec![
            vec![2],
            vec![1, 1],
            vec![2, 2],
            vec![1],
            vec![3, 3],
            vec![2],
            vec![4, 4],
            vec![1],
            vec![3],
            vec![4],
        ];
        let mut state = TestState {
            cache: LRUCache::new(0),
        };

        // when
        let result = execute_operations(&operations, &arguments, &mut state);

        // then
        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                Some(1),
                None,
                Some(-1),
                None,
                Some(-1),
                Some(3),
                Some(4)
            ]
        );
    }

    #[test]
    fn example10() {
        // given
        let operations = ["LRUCache", "put", "put", "put", "put", "get", "get"]
            .map(|s| TestOperation::from_str(s).unwrap());
        let arguments = vec![
            vec![2],
            vec![2, 1],
            vec![1, 1],
            vec![2, 3],
            vec![4, 1],
            vec![1],
            vec![2],
        ];
        let mut state = TestState {
            cache: LRUCache::new(0),
        };

        // when
        let result = execute_operations(&operations, &arguments, &mut state);

        // then
        assert_eq!(
            result,
            vec![None, None, None, None, None, Some(-1), Some(3)]
        );
    }

    #[test]
    fn example15() {
        // given
        let operations = [
            "LRUCache", "put", "put", "put", "put", "get", "get", "get", "get", "put", "get",
            "get", "get", "get", "get",
        ]
        .map(|s| TestOperation::from_str(s).unwrap());
        let arguments = vec![
            vec![3],
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![4],
            vec![3],
            vec![2],
            vec![1],
            vec![5, 5],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
        ];
        let mut state = TestState {
            cache: LRUCache::new(0),
        };

        // when
        let result = execute_operations(&operations, &arguments, &mut state);

        // then
        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                None,
                None,
                Some(4),
                Some(3),
                Some(2),
                Some(-1),
                None,
                Some(-1),
                Some(2),
                Some(3),
                Some(-1),
                Some(5)
            ]
        );
    }

    #[test]
    fn example16() {
        // given
        let operations = [
            "LRUCache", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get",
            "put", "get", "get", "get", "put", "put", "get", "put", "get",
        ]
        .map(|s| TestOperation::from_str(s).unwrap());
        let arguments = vec![
            vec![10],
            vec![7, 28],
            vec![7, 1],
            vec![8, 15],
            vec![6],
            vec![10, 27],
            vec![8, 10],
            vec![8],
            vec![6, 29],
            vec![1, 9],
            vec![6],
            vec![10, 7],
            vec![1],
            vec![2],
            vec![13],
            vec![8, 30],
            vec![1, 5],
            vec![1],
            vec![13, 2],
            vec![12],
        ];
        let mut state = TestState {
            cache: LRUCache::new(0),
        };

        // when
        let result = execute_operations(&operations, &arguments, &mut state);

        // then
        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                None,
                Some(-1),
                None,
                None,
                Some(10),
                None,
                None,
                Some(29),
                None,
                Some(9),
                Some(-1),
                Some(-1),
                None,
                None,
                Some(5),
                None,
                Some(-1),
            ]
        );
    }

    fn execute_operations(
        operations: &[TestOperation],
        arguments: &[Vec<i32>],
        state: &mut TestState,
    ) -> Vec<Option<i32>> {
        let mut result = Vec::with_capacity(operations.len());
        for (index, operation) in operations.iter().enumerate() {
            let operation_result = match operation {
                TestOperation::New => {
                    eprintln!("-- creating cache with capacity: {}", arguments[index][0]);
                    state.cache = LRUCache::new(arguments[index][0]);
                    None
                }
                TestOperation::Get => {
                    let key = arguments[index][0];
                    eprintln!("-- retrieving item with key: {}", key);
                    let result = Some(state.cache.get(key));
                    result
                }
                TestOperation::Put => {
                    let key = arguments[index][0];
                    let value = arguments[index][1];
                    eprintln!("-- putting {}: {}", key, value);
                    state.cache.put(key, value);
                    None
                }
            };
            validate_cache_structure(&state.cache);
            eprintln!("-- current state: {:?}", state.cache);
            result.push(operation_result);
        }
        result
    }

    #[test]
    fn getting_last_updates_tail() {
        // given
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.head.clone().unwrap().borrow().key, 2);
        assert_eq!(cache.tail.clone().unwrap().borrow().key, 1);

        // when
        let _ = cache.get(1);

        // then
        let most_recently_used = cache.head.clone().unwrap();
        assert_eq!(most_recently_used.clone().borrow().key, 1);
        assert!(
            matches!(most_recently_used.borrow().next.clone(), Some(next) if next.borrow().key == 2)
        );
        let least_recently_used = cache.tail.clone().unwrap();
        assert_eq!(least_recently_used.clone().borrow().key, 2);
        assert!(
            matches!(least_recently_used.borrow().previous.clone(), Some(previous) if previous.borrow().key == 1)
        );
        validate_cache_structure(&cache);
    }

    #[test]
    fn put_evicts_least_recently_used() {
        // given
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        let _ = cache.get(1);

        // when
        cache.put(3, 3);

        // then
        let most_recently_used = cache.head.clone().unwrap();
        assert_eq!(most_recently_used.clone().borrow().key, 3);
        assert!(
            matches!(most_recently_used.borrow().next.clone(), Some(next) if next.borrow().key == 1)
        );
        let least_recently_used = cache.tail.clone().unwrap();
        assert_eq!(least_recently_used.clone().borrow().key, 1);
        assert!(matches!(least_recently_used.clone().borrow().next, None));
        assert!(
            matches!(least_recently_used.borrow().previous.clone(), Some(previous) if previous.borrow().key == 3)
        );
        validate_cache_structure(&cache);
    }

    #[test]
    fn getting_last_places_it_first() {
        // given
        let mut cache = LRUCache::new(3);
        cache.put(3, 3);
        cache.put(2, 2);
        cache.put(1, 1);
        eprintln!("-- initial cache state: {:?}", cache);

        // when
        let result = cache.get(3);
        eprintln!("-- updated cache state: {:?}", cache);

        // then
        assert_eq!(result, 3);
        assert!(
            matches!(cache.map.get(&1), Some(x) if x.borrow().key == 1 && x.borrow().value == 1)
        );
        assert!(
            matches!(cache.map.get(&2), Some(x) if x.borrow().key == 2 && x.borrow().value == 2)
        );
        assert!(
            matches!(cache.map.get(&3), Some(x) if x.borrow().key == 3 && x.borrow().value == 3)
        );
        assert!(matches!(cache.head, Some(ref x) if x.borrow().key == 3 && x.borrow().value == 3));
        assert!(matches!(cache.tail, Some(ref x) if x.borrow().key == 2 && x.borrow().value == 2));
        assert!(
            matches!(cache.head.clone().unwrap().borrow().next, Some(ref x) if x.borrow().key == 1 && x.borrow().value == 1)
        );
        assert!(
            matches!(cache.tail.clone().unwrap().borrow().previous, Some(ref x) if x.borrow().key == 1 && x.borrow().value == 1)
        );
        validate_cache_structure(&cache);
    }

    fn validate_cache_structure(cache: &LRUCache) {
        assert!(cache.map.len() <= cache.capacity);
        let mut linked_list_length = 0usize;
        let mut cursor = cache.head.clone();
        while let Some(node) = cursor.clone() {
            if let Some(previous) = node.borrow().previous.clone() {
                assert_ne!(previous, node);
            } else {
                assert_eq!(cache.head.clone(), cursor.clone());
            }
            if let Some(next) = node.borrow().next.clone() {
                assert_ne!(node, next);
            } else {
                assert_eq!(cache.tail.clone(), cursor.clone());
            }
            assert_eq!(
                cache
                    .map
                    .get(&node.borrow().key)
                    .expect("Node from linked list is not in map")
                    .clone(),
                node
            );
            cursor = node.borrow().next.clone();
            linked_list_length += 1;
        }
        assert_eq!(linked_list_length, cache.map.len());
    }

    struct TestState {
        cache: LRUCache,
    }

    #[derive(Copy, Clone)]
    enum TestOperation {
        New,
        Get,
        Put,
    }

    impl FromStr for TestOperation {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "LRUCache" => Ok(Self::New),
                "get" => Ok(Self::Get),
                "put" => Ok(Self::Put),
                _ => Err(()),
            }
        }
    }

    impl Debug for LRUCache {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut cursor = self.head.clone();
            while let Some(node) = cursor.clone() {
                let node = node.borrow();
                write!(f, "{:?}, ", node)?;
                cursor = node.next.clone();
            }
            write!(f, "]")
        }
    }

    impl Debug for Node {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Node {{ key: {}, value: {}, previous: {:?}, next: {:?} }}",
                self.key,
                self.value,
                self.previous.clone().map(|p| p.borrow().key),
                self.next.clone().map(|n| n.borrow().key)
            )
        }
    }
}
