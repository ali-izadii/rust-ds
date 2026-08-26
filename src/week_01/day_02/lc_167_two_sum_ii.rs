//! [LeetCode 167 — Two Sum II](https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/)

#![allow(unused_variables)]

pub fn solve(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    if numbers.len() < 2 {
        return vec![];
    }

    let mut start_index = 0;
    let mut end_index = numbers.len() - 1;

    while start_index != end_index {
        let sum = numbers[start_index] + numbers[end_index];
        if sum == target {
            return vec![start_index as i32, end_index as i32];
        }
        if sum > target {
            end_index -= 1;
        } else if sum < target {
            start_index += 1;
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn finds_pairs_at_edges_and_with_duplicate_values() {
        assert_eq!(solve(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(solve(vec![-5, -1, 0, 4, 10], 5), vec![0, 4]);
        assert_eq!(solve(vec![1, 1, 3], 2), vec![0, 1]);
    }

    #[test]
    fn reports_no_pair_for_short_or_impossible_inputs() {
        assert_eq!(solve(vec![], 0), vec![]);
        assert_eq!(solve(vec![4], 8), vec![]);
        assert_eq!(solve(vec![1, 3, 5], 7), vec![]);
    }

    #[test]
    fn handles_negative_targets_zero_and_pairs_away_from_the_edges() {
        assert_eq!(solve(vec![-8, -5, -2, 1, 4], -10), vec![0, 2]);
        assert_eq!(solve(vec![-4, -1, 0, 3, 8], 4), vec![0, 4]);
        assert_eq!(solve(vec![1, 2, 4, 6, 10], 8), vec![1, 3]);
        assert_eq!(solve(vec![0, 0], 0), vec![0, 1]);
    }
}
