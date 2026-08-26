//! [LeetCode 217 — Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)

#![allow(unused_variables)]

use std::collections::HashSet;

pub fn solve(nums: Vec<i32>) -> bool {
    nums.len() != HashSet::<i32>::from_iter(nums).len()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn covers_empty_distinct_and_repeated_inputs() {
        assert!(!solve(vec![]));
        assert!(!solve(vec![-2, 0, 4]));
        assert!(solve(vec![1, 2, 3, 1]));
    }

    #[test]
    fn detects_adjacent_distant_negative_and_boundary_duplicates() {
        assert!(!solve(vec![42]));
        assert!(solve(vec![5, 5]));
        assert!(solve(vec![-1, 2, 3, 4, -1]));
        assert!(solve(vec![i32::MIN, 0, i32::MAX, i32::MIN]));
        assert!(!solve(vec![i32::MIN, 0, i32::MAX]));
    }
}
