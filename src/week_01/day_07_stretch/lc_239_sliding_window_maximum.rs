//! [LeetCode 239 — Sliding Window Maximum](https://leetcode.com/problems/sliding-window-maximum/)

#![allow(unused_variables)]

pub fn solve(nums: &[i32], window_size: usize) -> Vec<i32> {
    todo!("maintain candidate indices in a monotonic deque")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Stretch exercise: enable after implementing solve"]
    fn handles_descending_duplicates_and_window_boundaries() {
        assert_eq!(
            solve(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(solve(&[5, 4, 3], 1), vec![5, 4, 3]);
        assert_eq!(solve(&[2, 2, 2], 2), vec![2, 2]);
        assert_eq!(solve(&[1, 2, 3], 3), vec![3]);
    }

    #[test]
    #[ignore = "Stretch exercise: enable after implementing solve"]
    fn handles_negative_increasing_expiring_and_repeated_maxima() {
        assert_eq!(solve(&[-4, -2, -5, -1], 2), vec![-2, -2, -1]);
        assert_eq!(solve(&[1, 2, 3, 4, 5], 2), vec![2, 3, 4, 5]);
        assert_eq!(solve(&[5, 4, 3, 2, 1], 2), vec![5, 4, 3, 2]);
        assert_eq!(solve(&[9, 1, 2, 3, 4], 3), vec![9, 3, 4]);
        assert_eq!(solve(&[1, 3, 3, 2, 3], 3), vec![3, 3, 3]);
        assert_eq!(solve(&[7], 1), vec![7]);
        assert_eq!(solve(&[-3, -1, -2], 3), vec![-1]);
    }
}
