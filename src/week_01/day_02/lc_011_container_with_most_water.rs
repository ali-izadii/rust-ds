//! [LeetCode 11 — Container With Most Water](https://leetcode.com/problems/container-with-most-water/)

#![allow(unused_variables)]

pub fn solve(heights: &[i32]) -> i32 {
    todo!("implement opposite-end two pointers")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn covers_standard_edge_and_equal_height_cases() {
        assert_eq!(solve(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(solve(&[1, 1]), 1);
        assert_eq!(solve(&[4, 4, 4, 4]), 12);
        assert_eq!(solve(&[]), 0);
        assert_eq!(solve(&[9]), 0);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_zero_heights_monotonic_inputs_and_interior_optimum() {
        assert_eq!(solve(&[0, 0]), 0);
        assert_eq!(solve(&[0, 5, 0]), 0);
        assert_eq!(solve(&[1, 2, 3, 4, 5]), 6);
        assert_eq!(solve(&[5, 4, 3, 2, 1]), 6);
        assert_eq!(solve(&[1, 10, 1, 10, 1]), 20);
        assert_eq!(solve(&[2, 3, 10, 5, 7, 8, 9]), 36);
    }
}
