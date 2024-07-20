// 973. K Closest Points to Origin
// https://leetcode.com/problems/k-closest-points-to-origin

pub struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        for point in points {
            heap.push(Point {
                x: point[0],
                y: point[1],
            });
            heap.shrink_to(k as usize);
        }
        heap.into_sorted_vec()
            .iter()
            .rev()
            .take(k as usize)
            .map(Point::as_vec)
            .collect()
    }
}

#[derive(Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_to_origin(&self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }

    fn as_vec(&self) -> Vec<i32> {
        vec![self.x, self.y]
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance_to_origin().cmp(&self.distance_to_origin())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let points = [vec![1, 3], vec![-2, 2]];
        let k = 1;

        // when
        let result = Solution::k_closest(points.to_vec(), k);

        // then
        assert_eq!(result, [vec![-2, 2]].to_vec());
    }

    #[test]
    fn example2() {
        // given
        let points = [vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;

        // when
        let result = Solution::k_closest(points.to_vec(), k);

        // then
        assert!(
            result.eq(&[vec![3, 3], vec![-2, 4]].to_vec())
                || result.eq(&[vec![-2, 4], vec![3, 3]].to_vec())
        )
    }
}
