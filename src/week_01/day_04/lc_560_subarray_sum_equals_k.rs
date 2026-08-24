//! [LeetCode 560 — Subarray Sum Equals K](https://leetcode.com/problems/subarray-sum-equals-k/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32], target: i32) -> i32 {
    todo!("count prior prefix sums with a HashMap")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn counts_overlapping_zero_and_negative_sum_subarrays() {
        assert_eq!(solve(&[1, 1, 1], 2), 2);
        assert_eq!(solve(&[1, -1, 0], 0), 3);
        assert_eq!(solve(&[0, 0, 0], 0), 6);
        assert_eq!(solve(&[], 0), 0);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_singletons_negative_targets_and_repeated_prefix_sums() {
        assert_eq!(solve(&[3], 3), 1);
        assert_eq!(solve(&[3], 0), 0);
        assert_eq!(solve(&[-1, -1, 1], -1), 3);
        assert_eq!(solve(&[1, 2, 3], 3), 2);
        assert_eq!(solve(&[3, 4, 7, 2, -3, 1, 4, 2], 7), 4);
        assert_eq!(solve(&[1, -1, 1, -1], 0), 4);
        assert_eq!(solve(&[1, 2, 3], 100), 0);
    }
}
