// 621. Task Scheduler
// https://leetcode.com/problems/task-scheduler

pub struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut jobs = HashMap::with_capacity(tasks.len());
        for task in tasks {
            jobs.entry(task)
                .and_modify(|job: &mut Job| job.instances += 1)
                .or_insert_with(|| Job {
                    task_id: task,
                    cooldown: 0,
                    instances: 1,
                });
        }
        let mut priority_queue = BinaryHeap::with_capacity(jobs.len());
        for job in jobs.values_mut() {
            priority_queue.push(job);
        }
        let mut result = 0;
        while !priority_queue.is_empty() {
            let mut next_priority_queue = BinaryHeap::with_capacity(priority_queue.len());
            if let Some(highest_priority) = priority_queue.pop() {
                if highest_priority.can_execute() {
                    highest_priority.execute(n as usize);
                    if !highest_priority.is_complete() {
                        next_priority_queue.push(highest_priority);
                    }
                } else {
                    highest_priority.decrease_cooldown();
                    next_priority_queue.push(highest_priority);
                }
                result += 1;
            }
            while let Some(next) = priority_queue.pop() {
                next.decrease_cooldown();
                next_priority_queue.push(next);
            }
            priority_queue = next_priority_queue;
        }

        result
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Job {
    task_id: char,
    cooldown: usize,
    instances: usize,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinHeap pops the "maximum" value first
        // prioritize the lowest cooldown
        // then prioritize the most instances
        self.cooldown
            .cmp(&other.cooldown)
            .reverse()
            .then_with(|| self.instances.cmp(&other.instances))
            .then_with(|| self.task_id.cmp(&other.task_id))
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Job {
    fn decrease_cooldown(&mut self) {
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }
    }

    fn execute(&mut self, cooldown: usize) {
        assert_eq!(self.cooldown, 0);
        assert!(self.instances > 0);
        self.instances -= 1;
        self.cooldown += cooldown;
    }

    fn can_execute(&self) -> bool {
        self.cooldown == 0
    }

    fn is_complete(&self) -> bool {
        self.instances == 0
    }
}

#[cfg(test)]
mod tests {
    use super::{Job, Solution};
    use std::cmp::Ordering::{Greater, Less};
    use std::collections::{BTreeSet, BinaryHeap};

    #[test]
    fn example1() {
        // given
        let tasks = ["A", "A", "A", "B", "B", "B"];
        let tasks = tasks
            .map(|s| s.chars().next().unwrap())
            .iter()
            .cloned()
            .collect();
        let n = 2;

        // when
        let result = Solution::least_interval(tasks, n);

        // then
        assert_eq!(result, 8);
    }

    #[test]
    fn example2() {
        // given
        let tasks = ["A", "C", "A", "B", "D", "B"];
        let tasks = tasks
            .map(|s| s.chars().next().unwrap())
            .iter()
            .cloned()
            .collect();
        let n = 1;

        // when
        let result = Solution::least_interval(tasks, n);

        // then
        assert_eq!(result, 6);
    }

    #[test]
    fn example3() {
        // given
        let tasks = ["A", "A", "A", "B", "B", "B"];
        let tasks = tasks
            .map(|s| s.chars().next().unwrap())
            .iter()
            .cloned()
            .collect();
        let n = 3;

        // when
        let result = Solution::least_interval(tasks, n);

        // then
        assert_eq!(result, 10);
    }

    #[test]
    fn prioritize_cool_jobs() {
        // given
        let many_instances_but_warm = Job {
            task_id: 'X',
            cooldown: 13,
            instances: 2_147_483_647,
        };
        let few_instances_but_cool = Job {
            task_id: 'Y',
            cooldown: 0,
            instances: 3,
        };

        // when
        let mut heap = BinaryHeap::new();
        heap.push(many_instances_but_warm);
        heap.push(few_instances_but_cool);

        // then
        assert_eq!(
            few_instances_but_cool.cmp(&many_instances_but_warm),
            Greater
        );
        assert_eq!(heap.pop().unwrap(), few_instances_but_cool);
    }

    #[test]
    fn prioritize_high_cardinality_jobs() {
        // given
        let many_instances = Job {
            task_id: 'X',
            cooldown: 13,
            instances: 2_147_483_647,
        };
        let few_instances = Job {
            task_id: 'Y',
            cooldown: 13,
            instances: 127,
        };

        // when
        let mut heap = BinaryHeap::with_capacity(2);
        heap.push(few_instances);
        heap.push(many_instances);
        let mut set = BTreeSet::new();
        set.insert(few_instances);
        set.insert(many_instances);

        // then
        assert_eq!(many_instances.cmp(&few_instances), Greater);
        assert_eq!(few_instances.cmp(&many_instances), Less);
        assert_eq!(set.pop_last().unwrap(), many_instances);
        assert_eq!(*heap.peek().unwrap(), many_instances);
        assert_eq!(heap.pop().unwrap(), many_instances);
    }
}
