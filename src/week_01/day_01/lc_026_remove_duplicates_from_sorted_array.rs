//! [LeetCode 26 — Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/)

#![allow(unused_variables)]

/// Retains one copy of every value in a sorted slice and returns the new length.
pub fn solve(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn handles_empty_and_single_element_inputs() {
        let mut empty = vec![];
        assert_eq!(solve(&mut empty), 0);

        let mut one = vec![7];
        assert_eq!(solve(&mut one), 1);
        assert_eq!(&one[..1], &[7]);
    }

    #[test]
    fn compacts_runs_without_changing_unique_order() {
        let mut nums = vec![-3, -3, -1, -1, -1, 0, 4, 4];
        let length = solve(&mut nums);

        assert_eq!(length, 4);
        assert_eq!(&nums[..length as usize], &[-3, -1, 0, 4]);
    }

    #[test]
    fn handles_all_equal_and_already_unique_inputs() {
        let mut equal = vec![2, 2, 2, 2];
        assert_eq!(solve(&mut equal), 1);
        assert_eq!(&equal[..1], &[2]);

        let mut unique = vec![-2, 0, 5];
        assert_eq!(solve(&mut unique), 3);
        assert_eq!(&unique[..3], &[-2, 0, 5]);
    }

    #[test]
    fn handles_duplicate_runs_at_the_start_middle_and_end() {
        let mut at_start = vec![1, 1, 2, 3];
        let length = solve(&mut at_start);
        assert_eq!(&at_start[..length as usize], &[1, 2, 3]);

        let mut in_middle = vec![-3, -1, -1, -1, 2];
        let length = solve(&mut in_middle);
        assert_eq!(&in_middle[..length as usize], &[-3, -1, 2]);

        let mut at_end = vec![0, 1, 2, 2, 2];
        let length = solve(&mut at_end);
        assert_eq!(&at_end[..length as usize], &[0, 1, 2]);
    }
}

//  Notes
//  You cannot delete from a Vec while iterating with nums.iter(), because the loop borrows the vector
//      iter() creates an iterator that holds an immutable reference to nums for the duration of the loop

// for sorted list
// fn manual_dedup(v: &mut Vec<i32>) {
//     if v.len() <= 1 {
//         return;
//     }
//
//     let mut write_index = 1;
//
//     for read_index in 1..v.len() {
//         if v[write_index] == v[read_index - 1] {
//             continue
//         }
//         v[write_index] = v[read_index - 1];
//         write_index += 1;
//     }
//     v.truncate(write_index);
// }

