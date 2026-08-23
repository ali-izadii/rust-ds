//! Test-first scaffolds for hashing fundamentals (LC 217, LC 1, and LC 242).

#![allow(unused_variables)]
#![allow(unused_variables)]

use std::collections::HashMap;

pub fn frequencies(nums: &[i32]) -> HashMap<i32, usize> {
    todo!("count with HashMap::entry")
}

pub fn contains_duplicate(nums: &[i32]) -> bool {
    todo!("track seen values in a HashSet")
}

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    todo!("store seen values and indices in a HashMap")
}

pub fn is_anagram(left: &str, right: &str) -> bool {
    todo!("compare character frequencies")
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{contains_duplicate, frequencies, is_anagram, two_sum};

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing frequencies"]
    fn frequencies_counts_empty_repeated_negative_and_distinct_values() {
        assert_eq!(frequencies(&[]), HashMap::new());
        assert_eq!(
            frequencies(&[2, -1, 2, 0, -1, 2]),
            HashMap::from([(2, 3), (-1, 2), (0, 1)])
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing contains_duplicate"]
    fn contains_duplicate_covers_empty_distinct_and_repeated_inputs() {
        assert!(!contains_duplicate(&[]));
        assert!(!contains_duplicate(&[-2, 0, 4]));
        assert!(contains_duplicate(&[1, 2, 3, 1]));
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing two_sum"]
    fn two_sum_handles_order_duplicates_negative_values_and_no_answer() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
        assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
        assert_eq!(two_sum(&[-3, 4, 3, 90], 0), Some((0, 2)));
        assert_eq!(two_sum(&[1, 2, 3], 7), None);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing is_anagram"]
    fn is_anagram_rejects_different_lengths_counts_and_case() {
        assert!(is_anagram("anagram", "nagaram"));
        assert!(is_anagram("aabbcc", "ccbbaa"));
        assert!(!is_anagram("rat", "car"));
        assert!(!is_anagram("a", "aa"));
        assert!(!is_anagram("Ab", "ab"));
    }
}
