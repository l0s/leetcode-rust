pub struct Solution;

use std::ops::Bound::Included;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut car: Car = capacity.into();
        // sort trips by starting location
        let trips = trips.iter().map(Trip::from).collect::<BTreeSet<Trip>>();
        for trip in trips {
            car.start_trip(trip);
            if car.exceeded_capacity() {
                return false;
            }
        }
        true
    }
}

pub struct Car {
    max_capacity: u16,
    current_capacity: u16,
    /// The current location of the car
    location: u16,
    /// The currently active trips keyed by destination
    active_trips: BTreeMap<u16, Vec<Trip>>,
}

impl Car {
    /// Start a trip. This _may_ overload the vehicle.
    pub fn start_trip(&mut self, trip: Trip) {
        let next_location = trip.from;
        // find all the active trips that must have been completed prior to starting this one
        let passed_locations = self
            .active_trips
            .range((Included(&self.location), Included(&next_location)))
            .map(|(dropoff_location, _ending_trips)| *dropoff_location)
            .collect::<Vec<u16>>();
        for passed_location in passed_locations {
            let passengers_dropped_off: u16 = self
                .active_trips
                .get(&passed_location)
                .unwrap()
                .iter()
                .map(|ending_trip| ending_trip.num_passengers as u16)
                .sum();
            self.current_capacity -= passengers_dropped_off;
            self.active_trips.remove(&passed_location);
        }
        self.current_capacity += trip.num_passengers as u16;
        self.active_trips.entry(trip.to).or_default().push(trip);
        self.location = next_location;
    }

    pub fn exceeded_capacity(&self) -> bool {
        self.current_capacity > self.max_capacity
    }
}

impl From<i32> for Car {
    fn from(capacity: i32) -> Self {
        Self {
            max_capacity: capacity as u16,
            current_capacity: 0,
            location: 0,
            active_trips: BTreeMap::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Trip {
    num_passengers: u8,
    from: u16,
    to: u16,
}

impl Ord for Trip {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from)
    }
}

impl PartialOrd for Trip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.from.cmp(&other.from))
    }
}

impl From<&Vec<i32>> for Trip {
    fn from(trip: &Vec<i32>) -> Self {
        Self {
            num_passengers: trip[0] as u8,
            from: trip[1] as u16,
            to: trip[2] as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let result = Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4);
        assert!(!result);
    }

    #[test]
    fn example_2() {
        let result = Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5);
        assert!(result);
    }

    #[test]
    fn example_3() {
        let result = Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11);
        assert!(result);
    }

    #[test]
    fn example_4() {
        let result = Solution::car_pooling(
            vec![vec![10, 5, 7], vec![10, 3, 4], vec![7, 1, 8], vec![6, 3, 4]],
            24,
        );
        assert!(result);
    }
}
