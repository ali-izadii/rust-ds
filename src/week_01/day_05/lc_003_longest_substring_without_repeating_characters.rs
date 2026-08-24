//! [LeetCode 3 — Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

#![allow(unused_variables)]

pub fn solve(s: &str) -> usize {
    todo!("implement a variable-size sliding window")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_repetition_and_window_restarts() {
        assert_eq!(solve(""), 0);
        assert_eq!(solve("bbbb"), 1);
        assert_eq!(solve("abcabcbb"), 3);
        assert_eq!(solve("pwwkew"), 3);
        assert_eq!(solve("abba"), 2);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_singletons_spaces_symbols_and_late_left_pointer_updates() {
        assert_eq!(solve("a"), 1);
        assert_eq!(solve(" "), 1);
        assert_eq!(solve("a b!c"), 5);
        assert_eq!(solve("dvdf"), 3);
        assert_eq!(solve("tmmzuxt"), 5);
        assert_eq!(solve("anviaj"), 5);
        assert_eq!(solve("aab"), 2);
        assert_eq!(solve("abcdef"), 6);
    }
}
