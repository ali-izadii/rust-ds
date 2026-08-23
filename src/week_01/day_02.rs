//! Test-first scaffolds for LC 15 and LC 11.

#![allow(unused_variables)]
#![allow(unused_variables)]

pub fn three_sum(nums: &[i32]) -> Vec<Vec<i32>> {
    todo!("sort then use two pointers")
}

pub fn max_area(heights: &[i32]) -> i32 {
    todo!("implement opposite-end two pointers")
}

#[cfg(test)]
mod tests {
    use super::{max_area, three_sum};

    fn normalized(mut triplets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for triplet in &mut triplets {
            triplet.sort_unstable();
        }
        triplets.sort_unstable();
        triplets
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing three_sum"]
    fn three_sum_finds_unique_triplets_and_ignores_duplicate_answers() {
        assert_eq!(
            normalized(three_sum(&[-1, 0, 1, 2, -1, -4])),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing three_sum"]
    fn three_sum_handles_short_no_solution_and_all_zero_inputs() {
        assert_eq!(normalized(three_sum(&[])), Vec::<Vec<i32>>::new());
        assert_eq!(normalized(three_sum(&[1, 2])), Vec::<Vec<i32>>::new());
        assert_eq!(
            normalized(three_sum(&[1, 2, -2, -1])),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(normalized(three_sum(&[0, 0, 0, 0])), vec![vec![0, 0, 0]]);
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing max_area"]
    fn max_area_covers_standard_edge_and_equal_height_cases() {
        assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(&[1, 1]), 1);
        assert_eq!(max_area(&[4, 4, 4, 4]), 12);
        assert_eq!(max_area(&[]), 0);
        assert_eq!(max_area(&[9]), 0);
    }
}
