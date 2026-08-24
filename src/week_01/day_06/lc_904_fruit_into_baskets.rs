//! [LeetCode 904 — Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/)

#![allow(unused_variables)]

pub fn solve(fruits: &[i32]) -> usize {
    todo!("find the longest window with at most two kinds")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_one_kind_two_kinds_and_required_shrinking() {
        assert_eq!(solve(&[]), 0);
        assert_eq!(solve(&[1, 1, 1]), 3);
        assert_eq!(solve(&[1, 2]), 2);
        assert_eq!(solve(&[1, 2, 1]), 3);
        assert_eq!(solve(&[0, 1, 2, 2]), 3);
        assert_eq!(solve(&[1, 2, 3, 2, 2]), 4);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_late_best_windows_alternation_and_many_basket_changes() {
        assert_eq!(solve(&[7]), 1);
        assert_eq!(solve(&[1, 2, 1, 2, 1, 2]), 6);
        assert_eq!(solve(&[1, 2, 3, 4]), 2);
        assert_eq!(solve(&[3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
        assert_eq!(solve(&[0, 1, 6, 6, 4, 4, 6]), 5);
        assert_eq!(solve(&[1, 0, 1, 4, 1, 4, 1, 2, 3]), 5);
    }
}
