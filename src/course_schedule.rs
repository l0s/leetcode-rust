// 207. Course Schedule
// https://leetcode.com/problems/course-schedule

pub struct Solution;

use std::collections::{HashMap, VecDeque};
use DfsState::{Unvisited, Visited, Visiting};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.is_empty() {
            return true;
        }

        // "There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`"
        let num_courses = num_courses as usize;
        let mut courses = HashMap::with_capacity(num_courses);
        let mut dfs_states = HashMap::with_capacity(num_courses);
        (0..num_courses)
            .map(|course_id| course_id.into())
            .for_each(|course: Course| {
                dfs_states.insert(course.id, Unvisited);
                courses.insert(course.id, course);
            });

        for prerequisite in &prerequisites {
            let course_id = prerequisite[0] as usize;
            let dependency_id = prerequisite[1] as usize;
            courses
                .entry(course_id)
                .and_modify(|course| course.dependencies.push(dependency_id));
        }

        for root in courses.values() {
            if dfs_states[&root.id] != Unvisited {
                // only consider unvisited nodes
                continue;
            }

            // find cycle using DFS
            let mut stack = VecDeque::with_capacity(num_courses);
            stack.push_back(root.id);
            while let Some(course_id) = stack.back() {
                if dfs_states[course_id] == Unvisited {
                    // exploring paths through node
                    dfs_states.insert(*course_id, Visiting);
                    let course = courses.get(course_id).expect("Missing course");
                    for dependency_id in &course.dependencies {
                        if dfs_states[dependency_id] == Visiting {
                            // cycle detected
                            return false;
                        }
                        if dfs_states[dependency_id] == Unvisited {
                            stack.push_back(*dependency_id);
                        }
                    }
                } else if dfs_states[course_id] == Visiting {
                    // returning back through node
                    dfs_states.insert(*course_id, Visited);
                    stack.pop_back();
                } else if dfs_states[course_id] == Visited {
                    stack.pop_back();
                }
            }
        }

        // no cycles found
        true
    }
}

#[derive(Eq, PartialEq)]
enum DfsState {
    Unvisited,
    Visiting,
    Visited,
}

struct Course {
    id: usize,
    dependencies: Vec<usize>,
}

impl From<usize> for Course {
    fn from(value: usize) -> Self {
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
