/*
940. Distinct Subsequences II
From: https://leetcode.com/problems/distinct-subsequences-ii/
*/

pub struct Solution;

const C: usize = 10usize.pow(9) + 7;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        // 1 <= s.length <= 2000
        // s consists of lowercase English letters.
        // index is the character offset from 'a', the value is the last index in `s` in which it appears
        // a value greater than the length of `s` means the character hasn't appeared yet
        let mut character_last_index = vec![usize::MAX; 26usize];
        // index is the length of the subsequence
        // value is the number of distinct subsequences
        let mut subsequences_of_length = vec![usize::MAX; s.len() + 1];
        subsequences_of_length[0] = 1;

        for (i, c) in s.chars().enumerate() {
            // the most possible distinct subsequences that end with this character
            let max_distinct_subsequences = multiply(2, subsequences_of_length[i]);
            let character_index = c as usize - 'a' as usize;
            let last_index = character_last_index[character_index];
            subsequences_of_length[i + 1] = if (last_index as usize) < s.len() {
                // character appeared before
                // don't double-count previous distinct subsequences
                subtract(
                    max_distinct_subsequences,
                    subsequences_of_length[last_index] as usize,
                )
            } else {
                max_distinct_subsequences
            };
            character_last_index[character_index] = i;
        }
        (subsequences_of_length[s.len()] - 1) as i32
    }
}

fn multiply(x: usize, y: usize) -> usize {
    ((x % C) * (y % C)) % C
}

fn subtract(x: usize, y: usize) -> usize {
    (C + (x % C) - (y % C)) % C
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let s = "abc";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 7);
    }

    #[test]
    fn example2() {
        // given
        let s = "aba";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn example3() {
        // given
        let s = "aaa";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn example40() {
        // given
        let s = "glqkeogktv";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 983);
    }

    #[test]
    fn example50() {
        // given
        let s = "pcrdhwdxmqdznbenhwjsenjhvulyve";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 836817663);
    }

    #[test]
    fn example60() {
        // given
        let s = "zchmliaqdgvwncfatcfivphddpzjkgyygueikthqzyeeiebczqbqhdytkoawkehkbizdmcnilcjjlpoeoqqoqpswtqdpvszfaksn";

        // when
        let result = Solution::distinct_subseq_ii(s.to_owned());

        // then
        assert_eq!(result, 97915677);
    }
}
