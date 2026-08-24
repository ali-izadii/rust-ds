//! [LeetCode 76 — Minimum Window Substring](https://leetcode.com/problems/minimum-window-substring/)

#![allow(unused_variables)]

pub fn solve(text: &str, target: &str) -> String {
    todo!("track required and satisfied character counts")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Stretch exercise: enable after implementing solve"]
    fn handles_standard_no_match_empty_target_and_repeated_needs() {
        assert_eq!(solve("ADOBECODEBANC", "ABC"), "BANC");
        assert_eq!(solve("a", "aa"), "");
        assert_eq!(solve("abc", ""), "");
        assert_eq!(solve("AAABBC", "AABC"), "AABBC");
    }

    #[test]
    #[ignore = "Stretch exercise: enable after implementing solve"]
    fn handles_empty_text_exact_matches_edge_windows_and_duplicate_requirements() {
        assert_eq!(solve("", "a"), "");
        assert_eq!(solve("abc", "abc"), "abc");
        assert_eq!(solve("abc", "cba"), "abc");
        assert_eq!(solve("abc", "ac"), "abc");
        assert_eq!(solve("bba", "ab"), "ba");
        assert_eq!(solve("aa", "aa"), "aa");
        assert_eq!(solve("aaflslflsldkalskaaa", "aaa"), "aaa");
        assert_eq!(solve("abdcab", "ab"), "ab");
        assert_eq!(solve("abc", "abcd"), "");
    }
}
