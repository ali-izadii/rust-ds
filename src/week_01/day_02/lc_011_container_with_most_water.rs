//! [LeetCode 11 — Container With Most Water](https://leetcode.com/problems/container-with-most-water/)

#![allow(unused_variables)]
use std::cmp;

pub fn solve(height: Vec<i32>) -> i32 {
    if height.len() < 2 {
        return 0;
    }

    let mut start_index = 0;
    let mut end_index = height.len() - 1;
    let mut capacity = cmp::min(height[start_index], height[end_index]) * (end_index - start_index) as i32;

    while end_index != start_index {

        let cap_move_start = cmp::min(height[start_index + 1], height[end_index])
            * (end_index - (start_index + 1)) as i32;
        let cap_move_end = cmp::min(height[start_index], height[end_index - 1])
            * ((end_index - 1) - start_index) as i32;

        let tmp_cap = cmp::max(cap_move_start, cap_move_end);
        if tmp_cap == cap_move_start && cap_move_start >= capacity {
            start_index = start_index + 1;
            capacity = cap_move_start;
            continue;
        }
        if tmp_cap == cap_move_end && cap_move_end >= capacity {
            end_index = end_index - 1;
            capacity = cap_move_end;
            continue;
        }
        if height[start_index] > height[end_index] {
            end_index = end_index -1;
        } else {
            start_index = start_index + 1;
        }
    }

    capacity
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn covers_standard_edge_and_equal_height_cases() {
        assert_eq!(solve(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(solve(vec![1, 1]), 1);
        assert_eq!(solve(vec![4, 4, 4, 4]), 12);
        assert_eq!(solve(vec![]), 0);
        assert_eq!(solve(vec![9]), 0);
        assert_eq!(solve(vec![1,3,2,5,25,24,5]), 24);
    }

    #[test]
    fn handles_zero_heights_monotonic_inputs_and_interior_optimum() {
        assert_eq!(solve(vec![0, 0]), 0);
        assert_eq!(solve(vec![0, 5, 0]), 0);
        assert_eq!(solve(vec![1, 2, 3, 4, 5]), 6);
        assert_eq!(solve(vec![5, 4, 3, 2, 1]), 6);
        assert_eq!(solve(vec![1, 10, 1, 10, 1]), 20);
        assert_eq!(solve(vec![2, 3, 10, 5, 7, 8, 9]), 36);
    }
}
