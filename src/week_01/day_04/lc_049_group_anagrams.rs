//! [LeetCode 49 — Group Anagrams](https://leetcode.com/problems/group-anagrams/)

#![allow(unused_variables)]

pub fn solve(words: &[&str]) -> Vec<Vec<String>> {
    todo!("group words by a canonical key")
}

#[cfg(test)]
mod tests {
    use super::solve;

    fn normalized(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut groups {
            group.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn is_order_independent_and_handles_empty_words() {
        assert_eq!(
            normalized(solve(&["eat", "tea", "tan", "ate", "nat", "bat"])),
            normalized(vec![
                vec!["eat".into(), "tea".into(), "ate".into()],
                vec!["tan".into(), "nat".into()],
                vec!["bat".into()],
            ])
        );
        assert_eq!(
            normalized(solve(&["", ""])),
            vec![vec![String::new(), String::new()]]
        );
    }

    #[test]
    #[ignore = "Exercise scaffold: enable after implementing solve"]
    fn handles_empty_input_singletons_duplicates_and_multiple_group_sizes() {
        assert_eq!(normalized(solve(&[])), Vec::<Vec<String>>::new());
        assert_eq!(normalized(solve(&["a"])), vec![vec![String::from("a")]]);
        assert_eq!(
            normalized(solve(&["abc", "bca", "cab", "foo", "oof", "abc", "z"])),
            normalized(vec![
                vec!["abc".into(), "bca".into(), "cab".into(), "abc".into()],
                vec!["foo".into(), "oof".into()],
                vec!["z".into()],
            ])
        );
        assert_eq!(
            normalized(solve(&["ab", "ba", "cd", "dc"])),
            normalized(vec![
                vec!["ab".into(), "ba".into()],
                vec!["cd".into(), "dc".into()],
            ])
        );
    }
}
