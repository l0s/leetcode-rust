// 155. Min Stack
// https://leetcode.com/problems/min-stack

use std::collections::{BTreeMap, VecDeque};

#[derive(Default)]
pub struct MinStack {
    stack: VecDeque<i32>,
    tree: BTreeMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push_back(val);
        self.tree
            .entry(val)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn pop(&mut self) {
        let popped = self.stack.pop_back().expect("Cannot pop");
        *self.tree.get_mut(&popped).expect("Missing count") -= 1;
        if *self.tree.get(&popped).expect("Missing count") == 0 {
            self.tree.remove(&popped);
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.back().expect("Empty stack")
    }

    pub fn get_min(&self) -> i32 {
        *self.tree.first_key_value().expect("Empty tree").0
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    use crate::min_stack::MinStack;

    #[test]
    fn example1() {
        // given
        let operations = [
            "MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin",
        ];
        let arguments = [
            vec![],
            vec![-2],
            vec![0],
            vec![-3],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        // when
        let result = perform_operations(&operations, &arguments);

        // then
        assert_eq!(
            result,
            [None, None, None, None, Some(-3), None, Some(0), Some(-2)].to_vec()
        );
    }

    #[test]
    fn example5() {
        // given
        let operations = [
            "MinStack", "push", "push", "push", "getMin", "pop", "getMin",
        ];
        let arguments = [vec![], vec![0], vec![1], vec![0], vec![], vec![], vec![]];

        // when
        let result = perform_operations(&operations, &arguments);

        // then
        assert_eq!(
            result,
            [None, None, None, None, Some(0), None, Some(0)].to_vec()
        );
    }

    fn perform_operations(operations: &[&str], arguments: &[Vec<i32>]) -> Vec<Option<i32>> {
        let mut result: Vec<Option<i32>> = vec![None; operations.len()];
        let mut stack = MinStack::new();
        for (i, operation) in operations.iter().enumerate() {
            let arguments = &arguments[i];

            match *operation {
                "MinStack" => {
                    stack = MinStack::new();
                    result[i] = None;
                }
                "push" => {
                    stack.push(arguments[0]);
                    result[i] = None;
                }
                "pop" => {
                    stack.pop();
                    result[i] = None;
                }
                "top" => result[i] = Some(stack.top()),
                "getMin" => result[i] = Some(stack.get_min()),
                _ => unreachable!(),
            }
        }
        result
    }
}
