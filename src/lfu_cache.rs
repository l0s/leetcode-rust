use std::collections::{BTreeMap, HashMap};

/// From: https://leetcode.com/problems/lfu-cache/

pub struct LFUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    usage_counts: HashMap<i32, usize>,
    keys_by_count: BTreeMap<usize, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: HashMap::default(),
            usage_counts: HashMap::default(),
            keys_by_count: BTreeMap::default(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(result) = self.map.get(&key) {
            // increment usage count
            let count = self.usage_counts.get_mut(&key).unwrap();
            let bucket = self.keys_by_count.get_mut(count).unwrap();
            if let Some(index) = Self::index_of(bucket, key) {
                bucket.remove(index);
            }

            *count += 1;

            let keys = self.keys_by_count.entry(*count).or_insert(vec![]);

            if let Some(index) = Self::index_of(keys, key) {
                keys.remove(index);
            }
            keys.insert(0, key);

            *result
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        let keys_by_count = self.keys_by_count.clone();
        if !self.map.contains_key(&key) && self.map.len() >= self.capacity {
            // need to evict the least-frequently-used item
            if let Some((min_count, least_frequently_used_keys)) = keys_by_count
                .iter()
                .find(|(_count, bucket)| !bucket.is_empty())
            {
                let key_to_evict = least_frequently_used_keys[least_frequently_used_keys.len() - 1];
                if least_frequently_used_keys.len() <= 1 {
                    self.keys_by_count.remove(min_count);
                } else {
                    self.keys_by_count
                        .get_mut(min_count)
                        .unwrap()
                        .remove(least_frequently_used_keys.len() - 1);
                }
                self.usage_counts.remove(&key_to_evict);
                self.map.remove(&key_to_evict);
            }
        }
        if let Some(_old_value) = self.map.insert(key, value) {
            // existing record
            let usage_count = self.usage_counts.get_mut(&key).unwrap();

            let old_bucket = self
                .keys_by_count
                .entry(*usage_count)
                .or_insert_with(|| vec![key]);
            if let Some(index) = Self::index_of(old_bucket, key) {
                old_bucket.remove(index);
            }

            *usage_count += 1;

            let new_bucket = self
                .keys_by_count
                .entry(*usage_count)
                .or_insert_with(|| vec![key]);
            if let Some(old_index) = Self::index_of(new_bucket, key) {
                new_bucket.remove(old_index);
            }
            new_bucket.insert(0, key);
        } else {
            // new record
            self.usage_counts.insert(key, 1);
            let bucket = self.keys_by_count.entry(1).or_insert(vec![]);
            bucket.insert(0, key);
        }
    }

    fn index_of(bucket: &[i32], key: i32) -> Option<usize> {
        for (index, candidate) in bucket.iter().enumerate() {
            if *candidate == key {
                return Some(index);
            }
        }
        None
    }
}

/*
Your LFUCache object will be instantiated and called as such:
let obj = LFUCache::new(capacity);
let ret_1: i32 = obj.get(key);
obj.put(key, value);
*/
