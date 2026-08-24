//! [LeetCode 643 — Maximum Average Subarray I](https://leetcode.com/problems/maximum-average-subarray-i/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32], window_size: usize) -> f64 {
    todo!("implement a fixed-size sliding window")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_single_full_and_middle_windows() {
        assert_eq!(solve(&[5], 1), 5.0);
        assert_eq!(solve(&[-5, -2, -9], 3), -16.0 / 3.0);
        assert_eq!(solve(&[1, 12, -5, -6, 50, 3], 4), 12.75);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_all_negative_zero_duplicate_and_unit_windows() {
        assert_eq!(solve(&[-10, -2, -3, -4], 2), -2.5);
        assert_eq!(solve(&[0, 0, 0, 0], 2), 0.0);
        assert_eq!(solve(&[4, 4, 4, 4], 3), 4.0);
        assert_eq!(solve(&[-2, 8, 1], 1), 8.0);
        assert_eq!(solve(&[2, 4, 6, 8], 4), 5.0);
        assert_eq!(solve(&[9, 1, 2, 3, 9], 2), 6.0);
    }
}
