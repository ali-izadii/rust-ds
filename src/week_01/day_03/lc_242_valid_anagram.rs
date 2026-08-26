//! [LeetCode 242 — Valid Anagram](https://leetcode.com/problems/valid-anagram/)

#![allow(unused_variables)]

use std::collections::HashMap;

pub fn solve(s: String, t: String) -> bool {
    if s.is_empty() && t.is_empty() {
        return true;
    }
    if s.len() != t.len()
        || s.is_empty()
        || t.is_empty()
    {
        return false;
    }

    let m1 = s.chars().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0) += 1;
        m
    });

    let m2 = t.chars().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0) += 1;
        m
    });

    for (char, count) in &m1 {
        if m2.get(char) != Some(count) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn rejects_different_lengths_counts_and_case() {
        assert!(solve(String::from("anagram"), String::from("nagaram")));
        assert!(solve(String::from("aabbcc"), String::from("ccbbaa")));
        assert!(!solve(String::from("rat"), String::from("car")));
        assert!(!solve(String::from("a"), String::from("aa")));
        assert!(!solve(String::from("Ab"), String::from("ab")));
    }

    #[test]
    fn handles_empty_single_repeated_and_reordered_words() {
        assert!(solve(String::from(""), String::from("")));
        assert!(solve(String::from("a"), String::from("a")));
        assert!(solve(String::from("listen"), String::from("silent")));
        assert!(solve(String::from("aaabbbbcc"), String::from("cbababcab")));
        assert!(!solve(String::from("aacc"), String::from("ccac")));
        assert!(!solve(String::from("abc"), String::from("abd")));
        assert!(!solve(String::from(""), String::from("a")));
    }
}
