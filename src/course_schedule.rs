// 207. Course Schedule
// https://leetcode.com/problems/course-schedule

pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.is_empty() {
            return true;
        }
        let mut courses = HashMap::with_capacity(num_courses as usize);
        (0i32..num_courses)
            .map(|course_id| course_id.into())
            .for_each(|course: Course| {
                courses.insert(course.id, course);
            });
        let num_courses = num_courses as usize;
        for prerequisite in &prerequisites {
            let course_id = prerequisite[0];
            let dependency_id = prerequisite[1];
            courses
                .entry(course_id)
                .and_modify(|course| course.dependencies.push(dependency_id))
                .or_insert(Course {
                    id: course_id,
                    dependencies: vec![dependency_id],
                });
        }

        // find cycles using DFS
        for root in prerequisites
            .iter()
            .map(|tuple| courses.get(&tuple[0]).expect("Missing course"))
        {
            let mut visited = HashSet::with_capacity(num_courses);
            let mut stack = VecDeque::with_capacity(num_courses);
            stack.push_back(root);
            while let Some(node) = stack.pop_back() {
                visited.insert(node.id);
                for dependency_id in &node.dependencies {
                    if *dependency_id == root.id {
                        // cycle detected
                        return false;
                    }
                    if !visited.contains(dependency_id) {
                        let dependency = courses.get(dependency_id).expect("Missing course");
                        stack.push_back(dependency);
                    }
                }
            }
        }

        true
    }
}

struct Course {
    id: i32,
    dependencies: Vec<i32>,
}

impl From<i32> for Course {
    fn from(value: i32) -> Self {
        Self {
            id: value,
            dependencies: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let num_courses = 2;
        let prerequisites = [[1, 0]];

        // when
        let result = Solution::can_finish(
            num_courses,
            prerequisites.iter().map(|array| array.to_vec()).collect(),
        );

        // then
        assert!(result);
    }

    #[test]
    fn example2() {
        // given
        let num_courses = 2;
        let prerequisites = [[1, 0], [0, 1]];

        // when
        let result = Solution::can_finish(
            num_courses,
            prerequisites.iter().map(|array| array.to_vec()).collect(),
        );

        // then
        assert!(!result);
    }

    #[test]
    fn example3() {
        // given
        let num_courses = 1;
        let prerequisites: Vec<Vec<i32>> = vec![];

        // when
        let result = Solution::can_finish(
            num_courses,
            prerequisites.iter().map(|array| array.to_vec()).collect(),
        );

        // then
        assert!(result);
    }

    #[test]
    fn example43() {
        // given
        let num_courses = 20;
        let prerequisites = [
            [0, 10],
            [3, 18],
            [5, 5],
            [6, 11],
            [11, 14],
            [13, 1],
            [15, 1],
            [17, 4],
        ];

        // when
        let result = Solution::can_finish(
            num_courses,
            prerequisites.iter().map(|array| array.to_vec()).collect(),
        );

        // then
        assert!(!result);
    }

    #[test]
    fn example44() {
        // given
        let num_courses = 5;
        let prerequisites = [[1, 4], [2, 4], [3, 1], [3, 2]];

        // when
        let result = Solution::can_finish(
            num_courses,
            prerequisites.iter().map(|array| array.to_vec()).collect(),
        );

        // then
        assert!(result);
    }
}
