//! [LeetCode 977 — Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32]) -> Vec<i32> {
    todo!("implement opposite-end two pointers")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_negative_positive_and_mixed_inputs() {
        assert_eq!(solve(&[]), Vec::<i32>::new());
        assert_eq!(solve(&[-4, -2]), vec![4, 16]);
        assert_eq!(solve(&[1, 3]), vec![1, 9]);
        assert_eq!(solve(&[-7, -3, 0, 2, 5]), vec![0, 4, 9, 25, 49]);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_zeroes_duplicates_and_equal_absolute_values() {
        assert_eq!(solve(&[0]), vec![0]);
        assert_eq!(solve(&[0, 0, 0]), vec![0, 0, 0]);
        assert_eq!(solve(&[-2, -2, 2, 2]), vec![4, 4, 4, 4]);
        assert_eq!(solve(&[-5, -1, 1, 5]), vec![1, 1, 25, 25]);
        assert_eq!(solve(&[-10, -3, 1, 4, 8]), vec![1, 9, 16, 64, 100]);
    }
}
