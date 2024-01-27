// 859. Buddy Strings
// From: https://leetcode.com/problems/buddy-strings/

pub struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut s = s.as_bytes();
        let mut goal = goal.as_bytes();
        // strip common prefix
        for i in 0..s.len() {
            if s[i] != goal[i] {
                s = &s[i..s.len()];
                goal = &goal[i..goal.len()];
                break;
            }
        }
        // strip common suffix
        for i in (0..s.len()).rev() {
            if s[i] != goal[i] {
                s = &s[0..(i + 1)];
                goal = &goal[0..(i + 1)];
                break;
            }
        }
        for (i, c) in s.iter().enumerate() {
            for j in (i + 1)..s.len() {
                // swap i and j
                let modified = [
                    &s[0..i],
                    &[s[j]],
                    &s[(i + 1)..j],
                    &[*c],
                    &s[(j + 1)..s.len()],
                ]
                .concat();
                if modified.eq(&goal) {
                    return true;
                }
            }
        }
        false
    }
}
