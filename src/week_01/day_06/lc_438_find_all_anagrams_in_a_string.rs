//! [LeetCode 438 — Find All Anagrams in a String](https://leetcode.com/problems/find-all-anagrams-in-a-string/)

#![allow(unused_variables)]

pub fn solve(text: &str, pattern: &str) -> Vec<usize> {
    todo!("compare fixed-size frequency windows")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_overlaps_repeated_pattern_letters_and_no_match() {
        assert_eq!(solve("cbaebabacd", "abc"), vec![0, 6]);
        assert_eq!(solve("abab", "ab"), vec![0, 1, 2]);
        assert_eq!(solve("baa", "aa"), vec![1]);
        assert_eq!(solve("abc", "abcd"), Vec::<usize>::new());
        assert_eq!(solve("abc", "zz"), Vec::<usize>::new());
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_exact_match_all_windows_repeated_text_and_edge_positions() {
        assert_eq!(solve("abc", "abc"), vec![0]);
        assert_eq!(solve("abc", "cba"), vec![0]);
        assert_eq!(solve("aaaaa", "aa"), vec![0, 1, 2, 3]);
        assert_eq!(solve("abababab", "aab"), vec![0, 2, 4]);
        assert_eq!(solve("xabcxxcbay", "abc"), vec![1, 6]);
        assert_eq!(solve("a", "a"), vec![0]);
        assert_eq!(solve("a", "b"), Vec::<usize>::new());
    }
}
