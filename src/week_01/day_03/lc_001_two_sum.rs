//! [LeetCode 1 — Two Sum](https://leetcode.com/problems/two-sum/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("store seen values and indices in a HashMap")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_order_duplicates_negative_values_and_no_answer() {
        assert_eq!(solve(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(solve(&[3, 3], 6), Some((0, 1)));
        assert_eq!(solve(&[-3, 4, 3, 90], 0), Some((0, 2)));
        assert_eq!(solve(&[1, 2, 3], 7), None);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_zero_negative_targets_and_non_adjacent_pairs() {
        assert_eq!(solve(&[0, 4, 3, 0], 0), Some((0, 3)));
        assert_eq!(solve(&[-8, -3, 4, 9], -11), Some((0, 1)));
        assert_eq!(solve(&[5, 1, 8, 2], 7), Some((0, 3)));
        assert_eq!(solve(&[1, 5, 3, 7], 10), Some((2, 3)));
        assert_eq!(solve(&[], 0), None);
        assert_eq!(solve(&[6], 12), None);
    }
}
