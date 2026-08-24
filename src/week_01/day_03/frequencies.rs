//! [Week 1 Day 3 — Frequency-counter build exercise](../../../program/week_01_days_01_07.md#build)

#![allow(unused_variables)]

use std::collections::HashMap;

pub fn solve(nums: &[i32]) -> HashMap<i32, usize> {
    todo!("count with HashMap::entry")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn counts_empty_repeated_negative_and_distinct_values() {
        assert_eq!(solve(&[]), HashMap::new());
        assert_eq!(
            solve(&[2, -1, 2, 0, -1, 2]),
            HashMap::from([(2, 3), (-1, 2), (0, 1)])
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn counts_singletons_zero_and_integer_boundaries() {
        assert_eq!(solve(&[7]), HashMap::from([(7, 1)]));
        assert_eq!(solve(&[0, 0, 0, 0]), HashMap::from([(0, 4)]));
        assert_eq!(
            solve(&[i32::MIN, i32::MAX, i32::MIN]),
            HashMap::from([(i32::MIN, 2), (i32::MAX, 1)])
        );
    }
}
