//! [LeetCode 992 — Subarrays with K Different Integers](https://leetcode.com/problems/subarrays-with-k-different-integers/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32], k: usize) -> i32 {
    todo!("derive exactly-k from two at-most-k counts")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Stretch exercise: enable after implementing solve"]
    fn handles_exact_counts_duplicates_and_empty_input() {
        assert_eq!(solve(&[1, 2, 1, 2, 3], 2), 7);
        assert_eq!(solve(&[1, 2, 1, 3, 4], 3), 3);
        assert_eq!(solve(&[1, 1, 1], 1), 6);
        assert_eq!(solve(&[], 1), 0);
    }

    #[test]
    #[ignore = "Stretch exercise: enable after implementing solve"]
    fn handles_zero_k_too_many_k_all_unique_and_repeated_runs() {
        assert_eq!(solve(&[1, 2, 3], 0), 0);
        assert_eq!(solve(&[1, 2, 3, 4], 5), 0);
        assert_eq!(solve(&[1, 2, 3], 1), 3);
        assert_eq!(solve(&[1, 2, 3], 2), 2);
        assert_eq!(solve(&[1, 2, 3], 3), 1);
        assert_eq!(solve(&[1, 2, 1, 2], 2), 6);
        assert_eq!(solve(&[1, 1, 2, 2], 2), 4);
        assert_eq!(solve(&[1, 2, 1, 3, 4], 1), 5);
    }
}
