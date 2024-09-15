// 211. Design Add and Search Words Data Structure
// https://leetcode.com/problems/design-add-and-search-words-data-structure

use std::collections::HashMap;

#[derive(Default)]
pub struct WordDictionary {
    root: Node,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_word(&mut self, word: String) {
        let mut current_node = &mut self.root;
        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.terminates = true;
    }

    pub fn search(&self, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        self.root.contains(&chars)
    }
}

/// Trie Node
#[derive(Default)]
struct Node {
    terminates: bool,
    children: HashMap<char, Node>,
}

impl Node {
    fn contains(&self, word: &[char]) -> bool {
        if word.is_empty() {
            return self.terminates;
        }
        let c = word[0];
        if c == '.' {
            self.children
                .values()
                .any(|child| child.contains(&word[1..]))
        } else if let Some(child) = self.children.get(&c) {
            child.contains(&word[1..])
        } else {
            false
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
#[cfg(test)]
mod tests {
    use super::WordDictionary;
    use Operation::{Add, New, Search};

    #[test]
    fn example1() {
        // given
        let operations = [
            "WordDictionary",
            "addWord",
            "addWord",
            "addWord",
            "search",
            "search",
            "search",
            "search",
        ];
        let parameters = [
            vec![],
            vec!["bad"],
            vec!["dad"],
            vec!["mad"],
            vec!["pad"],
            vec!["bad"],
            vec![".ad"],
            vec!["b.."],
        ];
        let operations = operations.map(|operation| operation.into());

        // when
        let result = execute(&operations, &parameters);

        // then
        assert_eq!(
            result,
            vec![
                None,
                None,
                None,
                None,
                Some(false),
                Some(true),
                Some(true),
                Some(true)
            ]
        );
    }

    fn execute(operations: &[Operation], parameters: &[Vec<&str>]) -> Vec<Option<bool>> {
        assert_eq!(operations.len(), parameters.len());
        let mut result = vec![];
        let mut dictionary = WordDictionary::new();
        for (operation, parameters) in operations.iter().zip(parameters.iter()) {
            let outcome = match operation {
                New => {
                    dictionary = WordDictionary::new();
                    None
                }
                Add => {
                    assert_eq!(parameters.len(), 1);
                    let parameter = parameters[0];
                    dictionary.add_word(parameter.to_string());
                    None
                }
                Search => {
                    assert_eq!(parameters.len(), 1);
                    let parameter = parameters[0];
                    Some(dictionary.search(parameter.to_string()))
                }
            };
            result.push(outcome);
        }
        result
    }

    enum Operation {
        New,
        Add,
        Search,
    }

    impl From<&str> for Operation {
        fn from(value: &str) -> Self {
            match value {
                "WordDictionary" => New,
                "addWord" => Add,
                "search" => Search,
                _ => panic!("Unsupported operation: {}", value),
            }
        }
    }
}
