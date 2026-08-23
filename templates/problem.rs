//! # Problem title
//!
//! Source: <https://example.com/problem>
//!
//! ## Task
//! Briefly restate the problem in your own words. Include only the examples
//! and constraints you need; link to the original source instead of copying it.
//!
//! ## Idea and invariant
//! Explain the approach and the fact that remains true while the algorithm
//! runs. This is more valuable than a line-by-line explanation.
//!
//! - Time: `O(?)`
//! - Extra space: `O(?)`

/// Describe the return value and any input mutation here.
pub fn solve(input: &[i32]) -> usize {
    todo!("implement the solution")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example() {
        let input = [];

        // Replace both expected values when implementing the problem.
        assert_eq!(solve(&input), 0);
    }

    #[test]
    fn edge_case() {
        // Add the smallest, largest, or otherwise tricky valid case.
    }
}
