//! [LeetCode 128 — Longest Consecutive Sequence](https://leetcode.com/problems/longest-consecutive-sequence/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32]) -> usize {
    todo!("find sequence starts with a HashSet")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn ignores_duplicates_and_handles_gaps_and_negatives() {
        assert_eq!(solve(&[]), 0);
        assert_eq!(solve(&[100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(solve(&[1, 2, 0, 1]), 3);
        assert_eq!(solve(&[-2, -1, 0, 4]), 3);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_singletons_long_runs_unordered_values_and_integer_boundaries() {
        assert_eq!(solve(&[7]), 1);
        assert_eq!(solve(&[5, 5, 5]), 1);
        assert_eq!(solve(&[9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]), 7);
        assert_eq!(solve(&[-5, -4, -3, 10, 11]), 3);
        assert_eq!(solve(&[i32::MIN, i32::MIN + 1, i32::MAX]), 2);
        assert_eq!(solve(&[i32::MAX - 2, i32::MAX, i32::MAX - 1]), 3);
    }
}
