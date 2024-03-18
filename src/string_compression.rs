// 443. String Compression
// https://leetcode.com/problems/string-compression/

pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() == 1 {
            return 1;
        }
        let mut result = Vec::with_capacity(chars.len());
        let mut c = chars[0];
        let mut count = 1u16; // 1 <= chars.length <= 2000
        for item in chars.iter().skip(1) {
            if *item == c {
                count += 1;
            } else {
                append_char(&mut result, c, count);
                c = *item;
                count = 1;
            }
        }
        append_char(&mut result, c, count);

        chars.clear();
        chars.append(&mut result);

        chars.len() as i32
    }
}

fn append_char(result: &mut Vec<char>, c: char, count: u16) {
    result.push(c);
    if count != 1 {
        result.append(&mut to_chars(count));
    }
}

fn to_chars(int: u16) -> Vec<char> {
    int.to_string().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];

        // when
        let result = Solution::compress(&mut chars);

        // then
        assert_eq!(result, 6);
        assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    pub fn example2() {
        // given
        let mut chars = vec!['a'];

        // when
        let result = Solution::compress(&mut chars);

        // then
        assert_eq!(result, 1);
        assert_eq!(chars, vec!['a']);
    }

    #[test]
    pub fn example3() {
        // given
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];

        // when
        let result = Solution::compress(&mut chars);

        // then
        assert_eq!(result, 4);
        assert_eq!(chars, vec!['a', 'b', '1', '2']);
    }
}
