// 150. Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation

pub struct Solution;

use std::collections::VecDeque;
use Operation::{Add, Divide, Multiply, PushValue, Subtract};
use Token::{Literal, MinusSign, MultiplicationSign, Obelus, PlusSign};

enum Token {
    PlusSign,
    MinusSign, // In the future, this may be used for both unary and binary operations
    MultiplicationSign,
    Obelus,
    Literal(i32),
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    PushValue(i32),
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operations: VecDeque<Operation> = tokens
            .iter()
            .map(|string| {
                match string.as_str() {
                    "+" => PlusSign,
                    "-" => MinusSign,
                    "*" => MultiplicationSign,
                    "/" => Obelus, // In the future support both '➗' and '/'
                    _ => {
                        let value: i32 = string.parse().expect("Expected i32 literal");
                        Literal(value)
                    }
                }
            })
            .map(|token| {
                match token {
                    PlusSign => Add,
                    MinusSign => Subtract, // In the future, support this symbol as both negation and subtraction
                    MultiplicationSign => Multiply,
                    Obelus => Divide,
                    Literal(value) => PushValue(value),
                }
            })
            .collect();
        let mut stack = VecDeque::new();

        while let Some(operation) = operations.pop_front() {
            match operation {
                Add => {
                    let y = stack.pop_back().expect("Expected two operands, found 1");
                    let x = stack.pop_back().expect("Expected two operands, found 0");
                    stack.push_back(x + y);
                }
                Subtract => {
                    let y = stack.pop_back().expect("Expected two operands, found 1");
                    let x = stack.pop_back().expect("Expected two operands, found 0");
                    stack.push_back(x - y);
                }
                Multiply => {
                    let y = stack.pop_back().expect("Expected two operands, found 1");
                    let x = stack.pop_back().expect("Expected two operands, found 0");
                    stack.push_back(x * y);
                }
                Divide => {
                    let y = stack.pop_back().expect("Expected two operands, found 1");
                    let x = stack.pop_back().expect("Expected two operands, found 0");
                    stack.push_back(x / y);
                }
                PushValue(value) => {
                    stack.push_back(value);
                }
            }
        }

        stack.pop_back().expect("Incomplete expression")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let tokens = ["2", "1", "+", "3", "*"];

        // when
        let result = Solution::eval_rpn(tokens.map(String::from).iter().cloned().collect());

        // then
        assert_eq!(result, 9);
    }

    #[test]
    fn example2() {
        // given
        let tokens = ["4", "13", "5", "/", "+"];

        // when
        let result = Solution::eval_rpn(tokens.map(String::from).iter().cloned().collect());

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn example3() {
        // given
        let tokens = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];

        // when
        let result = Solution::eval_rpn(tokens.map(String::from).iter().cloned().collect());

        // then
        assert_eq!(result, 22);
    }
}
