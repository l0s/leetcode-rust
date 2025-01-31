// 210. Course Schedule II
// https://leetcode.com/problems/course-schedule-ii/

pub struct Solution;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut courses = HashMap::with_capacity(num_courses as usize);
        for course_id in 0..num_courses {
            let course = Course::from(course_id);
            courses.insert(course_id, course);
        }
        for prerequisite_spec in prerequisites {
            let course_id = prerequisite_spec[0];
            let prerequisite_id = prerequisite_spec[1];
            courses
                .get_mut(&course_id)
                .expect("Invalid course")
                .prerequisites
                .insert(prerequisite_id);
        }

        // Sort the courses by the number of prerequisites.
        // In general, prioritize courses with fewer prerequisites.
        // In case of a tie, prioritize lower course IDs for consistency.
        let mut prerequisite_counts: BTreeMap<usize, BTreeSet<i32>> = BTreeMap::new();
        for course in courses.values() {
            prerequisite_counts
                .entry(course.prerequisites.len())
                .and_modify(|bucket| {
                    bucket.insert(course.id);
                })
                .or_insert_with(|| {
                    let mut bucket = BTreeSet::new();
                    bucket.insert(course.id);
                    bucket
                });
        }

        let mut result = Vec::with_capacity(num_courses as usize);
        let mut completed = HashSet::with_capacity(num_courses as usize);

        for bucket in prerequisite_counts.values() {
            // Depth-first-search to emit courses in prerequisite order
            let mut visiting = HashSet::with_capacity(num_courses as usize);
            let mut stack = VecDeque::with_capacity(num_courses as usize);
            for course_id in bucket.iter().rev() {
                if !completed.contains(course_id) {
                    stack.push_back(*course_id);
                }
            }

            while let Some(course_id) = stack.pop_back() {
                if visiting.contains(&course_id) {
                    // already visited all the prerequisites
                    if !completed.contains(&course_id) {
                        completed.insert(course_id);
                        result.push(course_id);
                    }
                    visiting.remove(&course_id);
                } else {
                    visiting.insert(course_id);
                    stack.push_back(course_id);
                    let course = courses.get(&course_id).expect("Unmapped course");
                    for prerequisite_id in course.prerequisites.iter().rev() {
                        if visiting.contains(prerequisite_id) {
                            // cycle detected
                            return vec![];
                        }
                        if !completed.contains(prerequisite_id) {
                            stack.push_back(*prerequisite_id);
                        }
                    }
                }
            }
        }

        result
    }
}

struct Course {
    id: i32,
    /// Courses that must be taken before this one.
    /// Although they may be taken in any order (relative to each other), we sort them by ID for
    /// consistency.
    prerequisites: BTreeSet<i32>,
}

impl From<i32> for Course {
    fn from(id: i32) -> Self {
        Self {
            id,
            prerequisites: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::matrix_util::array_to_vec;

    #[test]
    fn example1() {
        // given
        let num_courses = 2;
        let prerequisites = [[1, 0]];

        // when
        let result = Solution::find_order(num_courses, array_to_vec(&prerequisites));

        // then
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example2() {
        // given
        let num_courses = 4;
        let prerequisites = [[1, 0], [2, 0], [3, 1], [3, 2]];

        // when
        let result = Solution::find_order(num_courses, array_to_vec(&prerequisites));

        // then
        assert!(
            result == vec![0, 1, 2, 3] || result == vec![0, 2, 1, 3],
            "Expected {:?} or {:?}, but got {:?}",
            vec![0, 1, 2, 3],
            vec![0, 2, 1, 3],
            result
        )
    }

    #[test]
    fn example3() {
        // given
        let num_courses = 1;
        let prerequisites = vec![];

        // when
        let result = Solution::find_order(num_courses, prerequisites);

        // then
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn example38() {
        // given
        let num_courses = 7;
        let prerequisites = [
            [1, 0],
            [0, 3],
            [0, 2],
            [3, 2],
            [2, 5],
            [4, 5],
            [5, 6],
            [2, 4],
        ];

        // when
        let result = Solution::find_order(num_courses, array_to_vec(&prerequisites));

        // then
        assert_eq!(result, vec![6, 5, 4, 2, 3, 0, 1]);
    }
}
