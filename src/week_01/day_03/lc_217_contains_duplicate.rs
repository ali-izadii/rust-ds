//! [LeetCode 217 — Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32]) -> bool {
    todo!("track seen values in a HashSet")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn covers_empty_distinct_and_repeated_inputs() {
        assert!(!solve(&[]));
        assert!(!solve(&[-2, 0, 4]));
        assert!(solve(&[1, 2, 3, 1]));
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn detects_adjacent_distant_negative_and_boundary_duplicates() {
        assert!(!solve(&[42]));
        assert!(solve(&[5, 5]));
        assert!(solve(&[-1, 2, 3, 4, -1]));
        assert!(solve(&[i32::MIN, 0, i32::MAX, i32::MIN]));
        assert!(!solve(&[i32::MIN, 0, i32::MAX]));
    }
}
