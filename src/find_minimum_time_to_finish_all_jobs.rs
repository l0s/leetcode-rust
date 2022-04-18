// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs/
pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let mut jobs = jobs.iter().map(|job| *job as u32).collect::<Vec<_>>();
        jobs.sort_by(|x, y| y.cmp(x));
        let mut workers = vec![0u32; k as usize];
        Self::min_time_required(&jobs, &mut workers, u32::MAX) as i32
    }

    fn min_time_required(jobs: &[u32], workers: &mut [u32], optimal: u32) -> u32 {
        if jobs.is_empty() {
            // no decisions to make, either we already found the optimal solution or the max
            // workload is the optimal solution
            let max_workload = *workers.iter().max().unwrap();
            return max_workload.min(optimal);
        }
        let job = jobs[jobs.len() - 1];
        let remaining = &jobs[0..jobs.len() - 1];
        // try assigning the job to each worker
        let mut min = optimal;

        let mut unique_workloads: BTreeSet<u32> = BTreeSet::new();
        for i in 0..workers.len() {
            if workers[i] + job >= optimal {
                continue;
            }
            if unique_workloads.contains(&workers[i]) {
                continue;
            }
            // experiment with assigning job to this worker
            workers[i] += job;
            let local_minimum = Self::min_time_required(remaining, workers, min);
            min = min.min(local_minimum);

            // reset the worker state for the next experiment
            workers[i] -= job;
            unique_workloads.insert(workers[i]);
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let jobs = vec![3, 2, 3];
        let workers = 3;
        let result = Solution::minimum_time_required(jobs, workers);
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        let jobs = vec![1, 2, 4, 7, 8];
        let workers = 2;
        let result = Solution::minimum_time_required(jobs, workers);
        assert_eq!(result, 11);
    }

    #[test]
    fn example3() {
        let jobs = vec![5, 5, 4, 4, 4];
        let workers = 2;
        let result = Solution::minimum_time_required(jobs, workers);
        assert_eq!(result, 12);
    }

    #[test]
    fn example4() {
        let jobs = vec![46, 13, 54, 51, 38, 49, 54, 67, 26, 78, 33];
        let workers = 10;
        let result = Solution::minimum_time_required(jobs, workers);
        assert_eq!(result, 78);
    }
}
