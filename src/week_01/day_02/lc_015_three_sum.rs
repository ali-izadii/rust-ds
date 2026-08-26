//! [LeetCode 15 — 3Sum](https://leetcode.com/problems/3sum/)

#![allow(unused_variables)]

pub fn solve(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    todo!("sort then use two pointers")
}

#[cfg(test)]
mod tests {
    use super::solve;

    fn normalized(mut triplets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triplet in &mut triplets {
            triplet.sort_unstable();
        }
        triplets.sort_unstable();
        triplets
    }

    #[test]
    fn finds_unique_triplets_and_ignores_duplicate_answers() {
        assert_eq!(
            normalized(solve(vec![-1, 0, 1, 2, -1, -4])),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn handles_short_no_solution_and_all_zero_inputs() {
        assert_eq!(normalized(solve(vec![])), Vec::<Vec<i32>>::new());
        assert_eq!(normalized(solve(vec![1, 2])), Vec::<Vec<i32>>::new());
        assert_eq!(
            normalized(solve(vec![1, 2, -2, -1])),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(normalized(solve(vec![0, 0, 0, 0])), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn handles_duplicate_values_multiple_answers_and_extreme_shapes() {
        assert_eq!(
            normalized(solve(vec![-2, 0, 0, 2, 2])),
            vec![vec![-2, 0, 2]]
        );
        assert_eq!(
            normalized(solve(vec![-4, -2, -2, -2, 0, 1, 2, 2, 3, 3, 4, 4, 6, 6])),
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2],
            ]
        );
        assert_eq!(normalized(solve(vec![-1, 0, 1])), vec![vec![-1, 0, 1]]);
        assert_eq!(normalized(solve(vec![1, 1, 1])), Vec::<Vec<i32>>::new());
    }
}
