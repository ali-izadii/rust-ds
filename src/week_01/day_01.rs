//! Test-first scaffolds for LC 283, LC 167, and LC 977.

#![allow(unused_variables)]
#![allow(unused_variables)]

pub fn move_zeroes(nums: &mut [i32]) {
    todo!("implement the two-pointer solution")
}

pub fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("implement opposite-end two pointers")
}

pub fn sorted_squares(nums: &[i32]) -> Vec<i32> {
    todo!("implement opposite-end two pointers")
}

#[cfg(test)]
mod arrays_and_two_pointers_tests {
    use super::{move_zeroes, sorted_squares, two_sum_sorted};

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing move_zeroes"]
    fn move_zeroes_preserves_relative_order_of_non_zero_values() {
        let mut nums = [0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, [1, 3, 12, 0, 0]);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing move_zeroes"]
    fn move_zeroes_handles_empty_all_zero_and_no_zero_inputs() {
        let mut empty = [];
        move_zeroes(&mut empty);
        assert_eq!(empty, []);

        let mut zeros = [0, 0, 0];
        move_zeroes(&mut zeros);
        assert_eq!(zeros, [0, 0, 0]);

        let mut non_zeroes = [-1, 2, 3];
        move_zeroes(&mut non_zeroes);
        assert_eq!(non_zeroes, [-1, 2, 3]);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing two_sum_sorted"]
    fn two_sum_sorted_finds_pairs_at_edges_and_with_duplicate_values() {
        assert_eq!(two_sum_sorted(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum_sorted(&[-5, -1, 0, 4, 10], 5), Some((0, 4)));
        assert_eq!(two_sum_sorted(&[1, 1, 3], 2), Some((0, 1)));
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing two_sum_sorted"]
    fn two_sum_sorted_reports_no_pair_for_short_or_impossible_inputs() {
        assert_eq!(two_sum_sorted(&[], 0), None);
        assert_eq!(two_sum_sorted(&[4], 8), None);
        assert_eq!(two_sum_sorted(&[1, 3, 5], 8), None);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing sorted_squares"]
    fn sorted_squares_handles_empty_negative_positive_and_mixed_inputs() {
        assert_eq!(sorted_squares(&[]), Vec::<i32>::new());
        assert_eq!(sorted_squares(&[-4, -2]), vec![4, 16]);
        assert_eq!(sorted_squares(&[1, 3]), vec![1, 9]);
        assert_eq!(sorted_squares(&[-7, -3, 0, 2, 5]), vec![0, 4, 9, 25, 49]);
    }
}

/// Retains one copy of every value in a sorted slice and returns the new length.
pub fn remove_duplicates(nums: &mut [i32]) -> usize {
    todo!("implement the two-pointer solution")
}

#[cfg(test)]
mod remove_duplicates_tests {
    use super::remove_duplicates;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing remove_duplicates"]
    fn handles_empty_and_single_element_inputs() {
        let mut empty = [];
        assert_eq!(remove_duplicates(&mut empty), 0);

        let mut one = [7];
        assert_eq!(remove_duplicates(&mut one), 1);
        assert_eq!(&one[..1], &[7]);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing remove_duplicates"]
    fn compacts_runs_without_changing_unique_order() {
        let mut nums = [-3, -3, -1, -1, -1, 0, 4, 4];
        let length = remove_duplicates(&mut nums);

        assert_eq!(length, 4);
        assert_eq!(&nums[..length], &[-3, -1, 0, 4]);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing remove_duplicates"]
    fn handles_all_equal_and_already_unique_inputs() {
        let mut equal = [2, 2, 2, 2];
        assert_eq!(remove_duplicates(&mut equal), 1);
        assert_eq!(&equal[..1], &[2]);

        let mut unique = [-2, 0, 5];
        assert_eq!(remove_duplicates(&mut unique), 3);
        assert_eq!(&unique[..3], &[-2, 0, 5]);
    }
}
