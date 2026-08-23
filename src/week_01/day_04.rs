//! Test-first scaffolds for prefix state and hashing (LC 49, LC 128, LC 560).

#![allow(unused_variables)]

pub fn prefix_sum(nums: &[i32]) -> Vec<i64> {
    todo!("accumulate into i64 to avoid narrow sums")
}

pub fn subarray_sum(nums: &[i32], target: i32) -> i32 {
    todo!("count prior prefix sums with a HashMap")
}

pub fn group_anagrams(words: &[&str]) -> Vec<Vec<String>> {
    todo!("group words by a canonical key")
}

pub fn longest_consecutive(nums: &[i32]) -> usize {
    todo!("find sequence starts with a HashSet")
}

#[cfg(test)]
mod tests {
    use super::{group_anagrams, longest_consecutive, prefix_sum, subarray_sum};

    fn normalized(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut groups {
            group.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing prefix_sum"]
    fn prefix_sum_handles_empty_negative_and_large_totals() {
        assert_eq!(prefix_sum(&[]), Vec::<i64>::new());
        assert_eq!(prefix_sum(&[2, -5, 4]), vec![2, -3, 1]);
        assert_eq!(
            prefix_sum(&[i32::MAX, i32::MAX]),
            vec![i32::MAX as i64, 2 * i32::MAX as i64]
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing subarray_sum"]
    fn subarray_sum_counts_overlapping_zero_and_negative_sum_subarrays() {
        assert_eq!(subarray_sum(&[1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(&[1, -1, 0], 0), 3);
        assert_eq!(subarray_sum(&[0, 0, 0], 0), 6);
        assert_eq!(subarray_sum(&[], 0), 0);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing group_anagrams"]
    fn group_anagrams_is_order_independent_and_handles_empty_words() {
        assert_eq!(
            normalized(group_anagrams(&["eat", "tea", "tan", "ate", "nat", "bat"])),
            normalized(vec![
                vec!["eat".into(), "tea".into(), "ate".into()],
                vec!["tan".into(), "nat".into()],
                vec!["bat".into()],
            ])
        );
        assert_eq!(
            normalized(group_anagrams(&["", ""])),
            vec![vec![String::new(), String::new()]]
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing longest_consecutive"]
    fn longest_consecutive_ignores_duplicates_and_handles_gaps_and_negatives() {
        assert_eq!(longest_consecutive(&[]), 0);
        assert_eq!(longest_consecutive(&[100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(longest_consecutive(&[1, 2, 0, 1]), 3);
        assert_eq!(longest_consecutive(&[-2, -1, 0, 4]), 3);
    }
}
