// 605. Can Place Flowers
// https://leetcode.com/problems/can-place-flowers

pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let occupied: i32 = flowerbed.iter().sum();
        let available = flowerbed.len() - occupied as usize;
        if n as usize > available {
            // regardless of adjacency rules, there are not enough spots
            return false;
        }

        let mut flowerbed = flowerbed;
        let mut remaining = n as usize;
        for i in 0..flowerbed.len() {
            if remaining == 0 {
                // placed all the flowers
                break;
            }
            if flowerbed[i] == 0
                && (i == 0 || flowerbed[i - 1] == 0)
                && (i + 1 >= flowerbed.len() || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                remaining -= 1;
            }
        }

        remaining == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn example1() {
        // given
        let flowerbed = [1, 0, 0, 0, 1];
        let n = 1;

        // when
        let result = Solution::can_place_flowers(flowerbed.into(), n);

        // then
        assert!(result);
    }

    #[test]
    pub fn example2() {
        // given
        let flowerbed = [1, 0, 0, 0, 1];
        let n = 2;

        // when
        let result = Solution::can_place_flowers(flowerbed.into(), n);

        // then
        assert!(!result);
    }

    #[test]
    pub fn example9() {
        // given
        let flowerbed = [1, 0, 0, 0, 0, 1];
        let n = 2;

        // when
        let result = Solution::can_place_flowers(flowerbed.into(), n);

        // then
        assert!(!result);
    }
}
