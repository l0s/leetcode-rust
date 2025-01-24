// 134. Gas Station
// https://leetcode.com/problems/gas-station/

pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        if gas.is_empty() {
            return -1;
        } else if gas.len() == 1 {
            return if cost[0] <= gas[0] { 0 } else { -1 };
        }
        let mut min_station = 0usize;
        let mut tank = gas[min_station];
        let mut min_fuel = tank;
        for station in 1..gas.len() {
            let fuel_to_station = cost[station - 1];
            tank -= fuel_to_station; // travel to station
            if tank < min_fuel {
                min_fuel = tank;
                min_station = station;
            }
            tank += gas[station]; // fill up at station
        }
        if min_fuel >= 0 {
            return 0;
        }
        let mut tank = gas[min_station];
        for offset in 1..=gas.len() {
            let fuel_to_next_station = cost[(min_station + offset - 1) % cost.len()];
            if tank < fuel_to_next_station {
                return -1;
            }
            tank -= fuel_to_next_station; // travel to station
            tank += gas[(min_station + offset) % gas.len()]; // fill up at station
        }
        min_station as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        // given
        let gas = [1, 2, 3, 4, 5];
        let cost = [3, 4, 5, 1, 2];

        // when
        let result = Solution::can_complete_circuit(gas.to_vec(), cost.to_vec());

        // then
        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        // given
        let gas = [2, 3, 4];
        let cost = [3, 4, 3];

        // when
        let result = Solution::can_complete_circuit(gas.to_vec(), cost.to_vec());

        // then
        assert_eq!(result, -1);
    }

    #[test]
    fn example37() {
        // given
        let gas = [3, 1, 1];
        let cost = [1, 2, 2];

        // when
        let result = Solution::can_complete_circuit(gas.to_vec(), cost.to_vec());

        // then
        assert_eq!(result, 0);
    }

    #[test]
    fn example38() {
        // given
        let gas = [4, 5, 3, 1, 4];
        let cost = [5, 4, 3, 4, 2];

        // when
        let result = Solution::can_complete_circuit(gas.to_vec(), cost.to_vec());

        // then
        assert_eq!(result, -1);
    }

    #[test]
    fn example39() {
        // given
        let gas = [4];
        let cost = [5];

        // when
        let result = Solution::can_complete_circuit(gas.to_vec(), cost.to_vec());

        // then
        assert_eq!(result, -1);
    }
}
