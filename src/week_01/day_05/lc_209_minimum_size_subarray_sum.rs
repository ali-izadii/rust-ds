//! [LeetCode 209 — Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)

#![allow(unused_variables)]

pub fn solve(target: i32, nums: &[i32]) -> usize {
    todo!("implement a shrinking window for positive values")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_exact_single_entire_and_impossible_windows() {
        assert_eq!(solve(7, &[2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(solve(4, &[1, 4, 4]), 1);
        assert_eq!(solve(11, &[1, 2, 3, 5]), 4);
        assert_eq!(solve(100, &[1, 2, 3]), 0);
        assert_eq!(solve(1, &[]), 0);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_prefix_suffix_interior_and_overshooting_windows() {
        assert_eq!(solve(5, &[5]), 1);
        assert_eq!(solve(5, &[6]), 1);
        assert_eq!(solve(5, &[4]), 0);
        assert_eq!(solve(6, &[6, 1, 1]), 1);
        assert_eq!(solve(6, &[1, 1, 6]), 1);
        assert_eq!(solve(8, &[1, 4, 4, 1]), 2);
        assert_eq!(solve(15, &[1, 2, 3, 4, 5]), 5);
        assert_eq!(solve(12, &[1, 2, 3, 4, 5]), 3);
    }
}
