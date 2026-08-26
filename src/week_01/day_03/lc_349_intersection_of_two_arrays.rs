//! [LeetCode 349 — Intersection of Two Arrays](https://leetcode.com/problems/intersection-of-two-arrays/)

#![allow(unused_variables)]

use std::collections::HashSet;

pub fn solve(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1 = HashSet::<i32>::from_iter(nums1);
    let set2 = HashSet::<i32>::from_iter(nums2);
    set1.intersection(&set2).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::solve;

    fn sorted(mut values: Vec<i32>) -> Vec<i32> {
        values.sort_unstable();
        values
    }

    #[test]
    fn finds_unique_common_values_regardless_of_output_order() {
        assert_eq!(sorted(solve(vec![1, 2, 2, 1], vec![2, 2])), vec![2]);
        assert_eq!(
            sorted(solve(vec![4, 9, 5], vec![9, 4, 9, 8, 4])),
            vec![4, 9]
        );
        assert_eq!(sorted(solve(vec![3, 1, 2], vec![1, 1, 2, 2])), vec![1, 2]);
    }

    #[test]
    fn handles_empty_disjoint_and_single_value_inputs() {
        assert_eq!(solve(vec![], vec![1, 2]), Vec::<i32>::new());
        assert_eq!(solve(vec![1, 2], vec![]), Vec::<i32>::new());
        assert_eq!(solve(vec![1, 2], vec![3, 4]), Vec::<i32>::new());
        assert_eq!(solve(vec![7], vec![7]), vec![7]);
        assert_eq!(solve(vec![7], vec![8]), Vec::<i32>::new());
    }

    #[test]
    fn handles_negatives_zero_duplicates_and_integer_boundaries() {
        assert_eq!(
            sorted(solve(
                vec![i32::MIN, -1, 0, 0, i32::MAX],
                vec![i32::MAX, 0, -1, -1, i32::MIN],
            )),
            vec![i32::MIN, -1, 0, i32::MAX]
        );
        assert_eq!(solve(vec![5, 5, 5], vec![5, 5]), vec![5]);
    }
}
