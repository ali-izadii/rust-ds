//! [LeetCode 424 — Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)

#![allow(unused_variables)]

pub fn solve(s: &str, k: usize) -> usize {
    todo!("track the highest character frequency in the window")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn covers_no_replacement_exact_replacement_and_repair() {
        assert_eq!(solve("A", 0), 1);
        assert_eq!(solve("ABAB", 2), 4);
        assert_eq!(solve("AABABBA", 1), 4);
        assert_eq!(solve("ABCDE", 0), 1);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_uniform_oversized_budget_and_stale_max_frequency() {
        assert_eq!(solve("", 0), 0);
        assert_eq!(solve("AAAA", 0), 4);
        assert_eq!(solve("ABCDE", 4), 5);
        assert_eq!(solve("BAAAB", 2), 5);
        assert_eq!(solve("ABBB", 2), 4);
        assert_eq!(solve("ABAA", 0), 2);
        assert_eq!(solve("BAAABCC", 1), 4);
    }
}
