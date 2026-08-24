//! [LeetCode 242 — Valid Anagram](https://leetcode.com/problems/valid-anagram/)

#![allow(unused_variables)]

pub fn solve(left: &str, right: &str) -> bool {
    todo!("compare character frequencies")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn rejects_different_lengths_counts_and_case() {
        assert!(solve("anagram", "nagaram"));
        assert!(solve("aabbcc", "ccbbaa"));
        assert!(!solve("rat", "car"));
        assert!(!solve("a", "aa"));
        assert!(!solve("Ab", "ab"));
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_single_repeated_and_reordered_words() {
        assert!(solve("", ""));
        assert!(solve("a", "a"));
        assert!(solve("listen", "silent"));
        assert!(solve("aaabbbbcc", "cbababcab"));
        assert!(!solve("aacc", "ccac"));
        assert!(!solve("abc", "abd"));
        assert!(!solve("", "a"));
    }
}
