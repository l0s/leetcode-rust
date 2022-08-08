// From https://leetcode.com/problems/lfu-cache/

use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

type Key = i32; // [0, 10^5]
type Value = i32; // [0, 10^9]

/// An entry in the Least-Frequently-Used Cache. It tracks how many times the
/// key has been used since it was (re)added. It is also a node in a doubly-
/// linked list of all the entries that share the same usage count.
#[derive(Clone)]
struct CacheEntry {
    key: Key,
    value: Value,
    usage_count: u16,
    /// the next least-recently-used entry with the same usage count
    next: Link,
    /// the next most-recently-used entry with the same usage count
    previous: Link,
}

/// A node in a doubly-linked list of entries with the same usage count
type Node = Rc<RefCell<CacheEntry>>;
/// A link to another entry with the same usage count
type Link = Option<Node>;

impl CacheEntry {
    pub fn new(key: Key, value: Value) -> Self {
        Self {
            key,
            value,
            usage_count: 1,
            next: None,
            previous: None,
        }
    }
}

impl Drop for CacheEntry {
    fn drop(&mut self) {
        self.previous.take();
        self.next.take();
    }
}

/// A container for all the cache entries that share the same usage count. It
/// is represented as a doubly-linked-list with the most-recently-used item at
/// the head and the least-recently-used item at the tail.
///
/// emulated https://rtoch.com/posts/rust-doubly-linked-list/
#[derive(Default)]
struct UsageCountBucket {
    /// The most-recently-used item
    head: Link,
    /// The least-recently-used item
    tail: Link,
    length: u16,
}

impl UsageCountBucket {
    /// Add a cache entry to the head of the list
    /// Parameters:
    /// - `entry` - a cache entry with the same usage count as the other nodes
    ///             in this bucket
    pub fn push_most_recent(&mut self, entry: Node) {
        assert!(entry.as_ref().borrow().next.is_none());
        assert!(entry.as_ref().borrow().previous.is_none());

        if let Some(head) = self.head.take() {
            head.borrow_mut().previous = Some(Rc::clone(&entry));
            entry.borrow_mut().next = Some(Rc::clone(&head));
            self.head = Some(entry);
            self.length += 1;
        } else {
            self.head = Some(Rc::clone(&entry));
            self.tail = Some(entry);
            self.length = 1;
        }
    }

    /// Remove and return the least-recently-used entry from this list
    /// Returns:
    /// - `Some(CacheEntry)` - the tail of the list if it is not empty
    /// - `None` - if the list is empty
    pub fn pop_least_recently_used(&mut self) -> Option<CacheEntry> {
        self.tail.take().map(|tail| {
            self.length -= 1;
            match tail.borrow_mut().previous.take() {
                Some(node) => {
                    node.borrow_mut().next.take();
                    self.tail = Some(node);
                }
                None => {
                    self.head.take();
                }
            }
            tail.as_ref().borrow().clone()
        })
    }

    /// Delete an entry from the list. Afterward, the list will have no
    /// reference to the entry and the entry will have no reference to items in
    /// the list.
    /// Parameters:
    /// - `entry` - the cache entry to remove
    pub fn remove(&mut self, entry: Node) {
        assert!(self.length >= 1);
        {
            let cache_entry = entry.as_ref().borrow();
            if let Some(left) = &cache_entry.previous {
                // not head
                if let Some(right) = &cache_entry.next {
                    // not tail, in the middle
                    left.as_ref().borrow_mut().next = Some(Rc::clone(right));
                    right.as_ref().borrow_mut().previous = Some(Rc::clone(left));
                } else {
                    // tail
                    left.borrow_mut().next.take();
                    self.tail = Some(Rc::clone(left));
                }
            } else {
                // head
                if let Some(right) = &cache_entry.next {
                    // not last remaining
                    right.borrow_mut().previous.take();
                    self.head = Some(Rc::clone(right));
                } else {
                    // last remaining
                    self.head.take();
                }
            }
        }
        self.length -= 1;
        {
            let mut cache_entry = entry.borrow_mut();
            cache_entry.previous.take();
            cache_entry.next.take();
        }
    }
}

impl Clone for UsageCountBucket {
    fn clone(&self) -> Self {
        Self {
            head: self.head.clone(),
            tail: self.tail.clone(),
            length: self.length,
        }
    }
}

impl Drop for UsageCountBucket {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            node.borrow_mut().previous.take();
            self.head = node.borrow_mut().next.take();
        }
        self.tail.take();
    }
}

pub struct LFUCache {
    /// [0, 10^4]
    capacity: u16,
    /// A hash table for the core caching logic --
    /// The value is a node in a doubly-linked list.
    map: HashMap<Key, Node>,
    /// An index of usage counts to all the entries that match that count --
    /// The value groups Nodes found in `map`.
    keys_by_count: HashMap<u32, UsageCountBucket>,
    usage_counts: BTreeSet<u32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as u16,
            map: HashMap::default(),
            keys_by_count: HashMap::default(),
            usage_counts: BTreeSet::default(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(result) = self.map.get_mut(&key) {
            // increment usage count
            let old_usage_count = result.as_ref().borrow().usage_count;
            let old_bucket = self
                .keys_by_count
                .get_mut(&(old_usage_count as u32))
                .expect("Could not find bucket for usage count");
            old_bucket.remove(Rc::clone(result));
            if old_bucket.length == 0 {
                self.keys_by_count.remove(&(old_usage_count as u32));
                self.usage_counts.remove(&(old_usage_count as u32));
            }

            let new_usage_count = old_usage_count + 1;
            result.borrow_mut().usage_count = new_usage_count;
            self.usage_counts.insert(new_usage_count as u32);

            let new_bucket = self
                .keys_by_count
                .entry(new_usage_count as u32)
                .or_default();
            new_bucket.push_most_recent(Rc::clone(result));

            result.as_ref().borrow().value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // evict if necessary
        let mut key_to_remove: Option<Key> = None;
        if !self.map.contains_key(&key) && self.map.len() >= self.capacity as usize {
            // need to evict the least-frequently-used item
            let mut count_to_remove: Option<u32> = None;
            if let Some(min_count) = self.usage_counts.iter().next() {
                let least_frequently_used_keys = self
                    .keys_by_count
                    .get_mut(min_count)
                    .expect("Bucket missing for minimum usage count");
                if let Some(entry) = least_frequently_used_keys.pop_least_recently_used() {
                    key_to_remove = Some(entry.key);
                }
                if least_frequently_used_keys.length == 0 {
                    count_to_remove = Some(*min_count);
                }
            }
            if let Some(count_to_remove) = count_to_remove {
                self.keys_by_count.remove(&count_to_remove);
                self.usage_counts.remove(&count_to_remove);
            }
        }
        if let Some(key_to_remove) = key_to_remove {
            self.map.remove(&key_to_remove);
        }

        // insert mapping
        let entry = Rc::new(RefCell::new(CacheEntry::new(key, value)));
        if let Some(old_entry) = self.map.insert(key, Rc::clone(&entry)) {
            // existing record
            // increment usage count
            let old_usage_count = old_entry.as_ref().borrow().usage_count;
            let mut count_to_remove: Option<u16> = None;
            if let Some(old_bucket) = self.keys_by_count.get_mut(&(old_usage_count as u32)) {
                old_bucket.remove(Rc::clone(&old_entry));
                if old_bucket.length == 0 {
                    count_to_remove = Some(old_usage_count);
                }
            }
            if let Some(count_to_remove) = count_to_remove {
                self.keys_by_count.remove(&(count_to_remove as u32));
                self.usage_counts.remove(&(count_to_remove as u32));
            }

            let new_usage_count = old_usage_count + 1;
            self.usage_counts.insert(new_usage_count as u32);
            entry.borrow_mut().usage_count = new_usage_count;

            let new_bucket = self
                .keys_by_count
                .entry(new_usage_count as u32)
                .or_default();
            new_bucket.push_most_recent(entry);
        } else {
            // new record
            // initialise usage count
            let new_bucket = self
                .keys_by_count
                .entry(entry.as_ref().borrow().usage_count as u32)
                .or_default();
            new_bucket.push_most_recent(entry);
            self.usage_counts.insert(1);
        }
    }
}

/*
Your LFUCache object will be instantiated and called as such:
let obj = LFUCache::new(capacity);
let ret_1: i32 = obj.get(key);
obj.put(key, value);
*/
