//! Optional Week 1 stretch test-first scaffolds (LC 76, LC 239, LC 992).

#![allow(unused_variables)]

pub fn min_window(text: &str, target: &str) -> String {
    todo!("track required and satisfied character counts")
}

pub fn max_sliding_window(nums: &[i32], window_size: usize) -> Vec<i32> {
    todo!("maintain candidate indices in a monotonic deque")
}

pub fn subarrays_with_k_distinct(nums: &[i32], k: usize) -> i32 {
    todo!("derive exactly-k from two at-most-k counts")
}

#[cfg(test)]
mod tests {
    use super::{max_sliding_window, min_window, subarrays_with_k_distinct};

    #[test]
    #[ignore = "Stretch exercise: enable after implementing min_window"]
    fn min_window_handles_standard_no_match_empty_target_and_repeated_needs() {
        assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(min_window("a", "aa"), "");
        assert_eq!(min_window("abc", ""), "");
        assert_eq!(min_window("AAABBC", "AABC"), "AABBC");
    }

    #[test]
    #[ignore = "Stretch exercise: enable after implementing max_sliding_window"]
    fn max_sliding_window_handles_descending_duplicates_and_window_boundaries() {
        assert_eq!(
            max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(max_sliding_window(&[5, 4, 3], 1), vec![5, 4, 3]);
        assert_eq!(max_sliding_window(&[2, 2, 2], 2), vec![2, 2]);
        assert_eq!(max_sliding_window(&[1, 2, 3], 3), vec![3]);
    }

    #[test]
    #[ignore = "Stretch exercise: enable after implementing subarrays_with_k_distinct"]
    fn subarrays_with_k_distinct_handles_exact_counts_duplicates_and_empty_input() {
        assert_eq!(subarrays_with_k_distinct(&[1, 2, 1, 2, 3], 2), 7);
        assert_eq!(subarrays_with_k_distinct(&[1, 2, 1, 3, 4], 3), 3);
        assert_eq!(subarrays_with_k_distinct(&[1, 1, 1], 1), 6);
        assert_eq!(subarrays_with_k_distinct(&[], 1), 0);
    }
}
