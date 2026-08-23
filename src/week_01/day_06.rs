//! Test-first scaffolds for advanced windows (LC 424, LC 904, LC 438).

#![allow(unused_variables)]

pub fn longest_at_most_k_distinct(s: &str, k: usize) -> usize {
    todo!("track counts in a variable-size window")
}

pub fn character_replacement(s: &str, k: usize) -> usize {
    todo!("track the highest character frequency in the window")
}

pub fn total_fruit(fruits: &[i32]) -> usize {
    todo!("find the longest window with at most two kinds")
}

pub fn find_anagrams(text: &str, pattern: &str) -> Vec<usize> {
    todo!("compare fixed-size frequency windows")
}

#[cfg(test)]
mod tests {
    use super::{character_replacement, find_anagrams, longest_at_most_k_distinct, total_fruit};

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing longest_at_most_k_distinct"]
    fn at_most_k_distinct_handles_empty_zero_k_and_shrinking_windows() {
        assert_eq!(longest_at_most_k_distinct("", 2), 0);
        assert_eq!(longest_at_most_k_distinct("abc", 0), 0);
        assert_eq!(longest_at_most_k_distinct("eceba", 2), 3);
        assert_eq!(longest_at_most_k_distinct("aa", 1), 2);
        assert_eq!(longest_at_most_k_distinct("abc", 5), 3);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing character_replacement"]
    fn character_replacement_covers_no_replacement_exact_replacement_and_repair() {
        assert_eq!(character_replacement("A", 0), 1);
        assert_eq!(character_replacement("ABAB", 2), 4);
        assert_eq!(character_replacement("AABABBA", 1), 4);
        assert_eq!(character_replacement("ABCDE", 0), 1);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing total_fruit"]
    fn total_fruit_handles_empty_one_kind_two_kinds_and_required_shrinking() {
        assert_eq!(total_fruit(&[]), 0);
        assert_eq!(total_fruit(&[1, 1, 1]), 3);
        assert_eq!(total_fruit(&[1, 2]), 2);
        assert_eq!(total_fruit(&[1, 2, 1]), 3);
        assert_eq!(total_fruit(&[0, 1, 2, 2]), 3);
        assert_eq!(total_fruit(&[1, 2, 3, 2, 2]), 4);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing find_anagrams"]
    fn find_anagrams_handles_overlaps_repeated_pattern_letters_and_no_match() {
        assert_eq!(find_anagrams("cbaebabacd", "abc"), vec![0, 6]);
        assert_eq!(find_anagrams("abab", "ab"), vec![0, 1, 2]);
        assert_eq!(find_anagrams("baa", "aa"), vec![1]);
        assert_eq!(find_anagrams("abc", "abcd"), Vec::<usize>::new());
        assert_eq!(find_anagrams("abc", "zz"), Vec::<usize>::new());
    }
}
