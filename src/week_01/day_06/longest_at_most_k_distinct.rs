//! [Week 1 Day 6 — At-most-K build exercise](../../../program/week_01_days_01_07.md#build-2)

#![allow(unused_variables)]

pub fn solve(s: &str, k: usize) -> usize {
    todo!("track counts in a variable-size window")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_zero_k_and_shrinking_windows() {
        assert_eq!(solve("", 2), 0);
        assert_eq!(solve("abc", 0), 0);
        assert_eq!(solve("eceba", 2), 3);
        assert_eq!(solve("aa", 1), 2);
        assert_eq!(solve("abc", 5), 3);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_single_kind_repeated_shrinks_and_full_string_windows() {
        assert_eq!(solve("aaaaa", 1), 5);
        assert_eq!(solve("aabbcc", 1), 2);
        assert_eq!(solve("aabbcc", 2), 4);
        assert_eq!(solve("aabbcc", 3), 6);
        assert_eq!(solve("abaccc", 2), 4);
        assert_eq!(solve("ccaabbb", 2), 5);
        assert_eq!(solve("abcadcacacaca", 3), 11);
    }
}
