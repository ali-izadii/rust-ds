//! [LeetCode 167 — Two Sum II](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("implement opposite-end two pointers")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn finds_pairs_at_edges_and_with_duplicate_values() {
        assert_eq!(solve(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(solve(&[-5, -1, 0, 4, 10], 5), Some((0, 4)));
        assert_eq!(solve(&[1, 1, 3], 2), Some((0, 1)));
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn reports_no_pair_for_short_or_impossible_inputs() {
        assert_eq!(solve(&[], 0), None);
        assert_eq!(solve(&[4], 8), None);
        assert_eq!(solve(&[1, 3, 5], 7), None);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_negative_targets_zero_and_pairs_away_from_the_edges() {
        assert_eq!(solve(&[-8, -5, -2, 1, 4], -10), Some((0, 2)));
        assert_eq!(solve(&[-4, -1, 0, 3, 8], 4), Some((0, 4)));
        assert_eq!(solve(&[1, 2, 4, 6, 10], 8), Some((1, 3)));
        assert_eq!(solve(&[0, 0], 0), Some((0, 1)));
    }
}
