//! [Week 1 Day 4 — Prefix-sum build exercise](../../../program/week_01_days_01_07.md#build-1)

#![allow(unused_variables)]

pub fn solve(nums: &[i32]) -> Vec<i64> {
    todo!("accumulate into i64 to avoid narrow sums")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_negative_and_large_totals() {
        assert_eq!(solve(&[]), Vec::<i64>::new());
        assert_eq!(solve(&[2, -5, 4]), vec![2, -3, 1]);
        assert_eq!(
            solve(&[i32::MAX, i32::MAX]),
            vec![i32::MAX as i64, 2 * i32::MAX as i64]
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_singletons_zeroes_and_alternating_signs() {
        assert_eq!(solve(&[7]), vec![7]);
        assert_eq!(solve(&[0, 0, 0]), vec![0, 0, 0]);
        assert_eq!(solve(&[5, -5, 5, -5]), vec![5, 0, 5, 0]);
        assert_eq!(solve(&[i32::MIN, i32::MAX]), vec![i32::MIN as i64, -1]);
    }
}
