// 981. Time Based Key-Value Store
// From: https://leetcode.com/problems/time-based-key-value-store/

use std::collections::{BTreeMap, HashMap};

type Key = String;
type Value = String;
type Timestamp = i32; // [1, 10^7]

#[derive(Default)]
pub struct TimeMap {
    delegate: HashMap<Key, BTreeMap<Timestamp, Value>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        let bucket = self.delegate.entry(key).or_default();
        bucket.insert(timestamp, value);
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(bucket) = self.delegate.get(&key) {
            return bucket
                .range(1..=timestamp)
                .next_back()
                .map(|tuple| tuple.1.clone())
                .unwrap_or_else(|| "".to_owned());
        }

        "".to_owned()
    }
}

/*
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
