//! [LeetCode 283 — Move Zeroes](https://leetcode.com/problems/move-zeroes/)

#![allow(unused_variables)]

pub fn solve(nums: &mut Vec<i32>) {
    if nums.len() <= 1 {
        return;
    }
    let mut write_index = 0;
    for read_index in 0..nums.len() {
        if nums[read_index] != 0 {
            nums.swap(write_index, read_index);
            write_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn preserves_relative_order_of_non_zero_values() {
        let mut nums = vec![0, 1, 0, 3, 12];
        solve(&mut nums);
        assert_eq!(nums, [1, 3, 12, 0, 0]);
    }

    #[test]
    fn handles_empty_all_zero_and_no_zero_inputs() {
        let mut empty = vec![];
        solve(&mut empty);
        assert_eq!(empty, []);

        let mut zeros = vec![0, 0, 0];
        solve(&mut zeros);
        assert_eq!(zeros, [0, 0, 0]);

        let mut non_zeroes = vec![-1, 2, 3];
        solve(&mut non_zeroes);
        assert_eq!(non_zeroes, [-1, 2, 3]);
    }

    #[test]
    fn handles_leading_trailing_consecutive_and_negative_values() {
        let mut leading = vec![0, 0, 1, 2];
        solve(&mut leading);
        assert_eq!(leading, [1, 2, 0, 0]);

        let mut trailing = vec![1, 2, 0, 0];
        solve(&mut trailing);
        assert_eq!(trailing, [1, 2, 0, 0]);

        let mut mixed = vec![-1, 0, -2, 0, 3, 0];
        solve(&mut mixed);
        assert_eq!(mixed, [-1, -2, 3, 0, 0, 0]);
    }
}
