//! Test-first scaffolds for sliding-window fundamentals (LC 643, LC 3, LC 209).

#![allow(unused_variables)]

pub fn max_average(nums: &[i32], window_size: usize) -> f64 {
    todo!("implement a fixed-size sliding window")
}

pub fn longest_unique_substring(s: &str) -> usize {
    todo!("implement a variable-size sliding window")
}

pub fn min_subarray_len(target: i32, nums: &[i32]) -> usize {
    todo!("implement a shrinking window for positive values")
}

#[cfg(test)]
mod tests {
    use super::{longest_unique_substring, max_average, min_subarray_len};

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing max_average"]
    fn max_average_handles_single_full_and_middle_windows() {
        assert_eq!(max_average(&[5], 1), 5.0);
        assert_eq!(max_average(&[-5, -2, -9], 3), -16.0 / 3.0);
        assert_eq!(max_average(&[1, 12, -5, -6, 50, 3], 4), 12.75);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing longest_unique_substring"]
    fn longest_unique_substring_handles_empty_repetition_and_window_restarts() {
        assert_eq!(longest_unique_substring(""), 0);
        assert_eq!(longest_unique_substring("bbbb"), 1);
        assert_eq!(longest_unique_substring("abcabcbb"), 3);
        assert_eq!(longest_unique_substring("pwwkew"), 3);
        assert_eq!(longest_unique_substring("abba"), 2);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing min_subarray_len"]
    fn min_subarray_len_handles_exact_single_entire_and_impossible_windows() {
        assert_eq!(min_subarray_len(7, &[2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(min_subarray_len(4, &[1, 4, 4]), 1);
        assert_eq!(min_subarray_len(11, &[1, 2, 3, 5]), 4);
        assert_eq!(min_subarray_len(100, &[1, 2, 3]), 0);
        assert_eq!(min_subarray_len(1, &[]), 0);
    }
}
