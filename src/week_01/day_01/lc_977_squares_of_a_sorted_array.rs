//! [LeetCode 977 — Squares of a Sorted Array](https://leetcode.com/problems/squares-of-a-sorted-array/)

#![allow(unused_variables)]

pub fn solve(nums: Vec<i32>) -> Vec<i32> {

    if nums.is_empty() {
        return vec![];
    }

    let mut new_nums = vec![0; nums.len()];
    let mut position = nums.len() - 1;

    let mut start_index = 0;
    let mut end_index = nums.len() - 1;

    while start_index < end_index {
        let start_pow = nums[start_index].pow(2);
        let end_pow = nums[end_index].pow(2);

        if end_pow >= start_pow {
            new_nums[position] = end_pow;
            end_index -= 1;
        } else {
            new_nums[position] = start_pow;
            start_index += 1;
        }

        position -= 1;
    }

    new_nums[position] = nums[start_index].pow(2);

    new_nums
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn handles_empty_negative_positive_and_mixed_inputs() {
        //assert_eq!(solve(vec![]), Vec::<i32>::new());
        assert_eq!(solve(vec![-4, -2]), vec![4, 16]);
        //assert_eq!(solve(vec![1, 3]), vec![1, 9]);
        //assert_eq!(solve(vec![-7, -3, 0, 2, 5]), vec![0, 4, 9, 25, 49]);
    }

    #[test]
    fn handles_zeroes_duplicates_and_equal_absolute_values() {
        assert_eq!(solve(vec![0]), vec![0]);
        assert_eq!(solve(vec![0, 0, 0]), vec![0, 0, 0]);
        assert_eq!(solve(vec![-2, -2, 2, 2]), vec![4, 4, 4, 4]);
        assert_eq!(solve(vec![-5, -1, 1, 5]), vec![1, 1, 25, 25]);
        assert_eq!(solve(vec![-10, -3, 1, 4, 8]), vec![1, 9, 16, 64, 100]);
    }
}
