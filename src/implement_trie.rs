// 208. Implement Trie (Prefix Tree)
// https://leetcode.com/problems/implement-trie-prefix-tree/

use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    root: TrieNode,
}

struct TrieNode {
    terminates: bool,
    children: HashMap<char, TrieNode>,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            terminates: false,
            children: HashMap::with_capacity(26), // "word and prefix consist only of lowercase English letters."
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: String) {
        let first_char = word
            .chars()
            .next()
            .expect("words must have at least 1 character");
        let mut node = self.root.children.entry(first_char).or_default();
        for c in word.chars().skip(1) {
            let children = &mut node.children;
            node = children.entry(c).or_default();
        }
        node.terminates = true;
    }

    pub fn search(&self, word: String) -> bool {
        if let Some(node) = self.node(&word) {
            return node.terminates;
        }
        false
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        if self.node(&prefix).is_some() {
            return true;
        }
        false
    }

    fn node(&self, prefix: &str) -> Option<&TrieNode> {
        let first_char = prefix
            .chars()
            .next()
            .expect("words must have at least 1 character");
        if let Some(root) = self.root.children.get(&first_char) {
            let mut node = root;
            for c in prefix.chars().skip(1) {
                if let Some(child) = node.children.get(&c) {
                    node = child;
                } else {
                    return None;
                }
            }
            return Some(node);
        }
        None
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn example1() {
        // given
        let mut trie = Trie::new();

        // when
        trie.insert("apple".to_string());

        // then
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));

        // when
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}
