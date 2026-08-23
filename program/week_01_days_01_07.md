# Week 1 — Arrays, Hashing & Sliding Window
**Days 1–7 | Target: build linear-scan pattern recognition**

## Weekly goals

By the end of this week, you should be able to:

- Explain why two-pointer algorithms can be `O(n)`.
- Use `HashMap`, `HashSet`, and fixed-size frequency arrays idiomatically in Rust.
- Derive prefix-sum + hashmap solutions instead of memorizing them.
- Distinguish fixed-size and variable-size sliding windows.
- Explain why a shrinking sliding window with nested loops can still be `O(n)`.

## Rust focus

```rust
use std::collections::{HashMap, HashSet};
```

Practice:

- `Vec<T>` and slices `&[T]`
- `iter()`, `enumerate()`
- manual indexing with `usize`
- `HashMap::entry`
- fixed-size arrays such as `[usize; 26]`
- `String::as_bytes()`

---

## Day 1 — Arrays & Two Pointers

### Study
Learn:

- array invariants
- slow/fast pointers
- opposite-end pointers
- why monotonic pointer movement prevents quadratic work

Recommended:
- USACO Guide — Two Pointers
- Competitive Programmer's Handbook — Sorting and Two Pointers

### Build in Rust

Create:

```rust
fn remove_duplicates(nums: &mut Vec<i32>) -> usize
fn move_zeroes(nums: &mut Vec<i32>)
fn two_sum_sorted(nums: &[i32], target: i32) -> Option<(usize, usize)>
```

### Problems

1. LC 26 — Remove Duplicates from Sorted Array
2. LC 283 — Move Zeroes
3. LC 977 — Squares of a Sorted Array

### Review prompt

> Why can each pointer move at most `n` times?

---

## Day 2 — Two-Pointer Elimination

### Study

Internalize this idea:

> Moving a pointer must eliminate a set of impossible answers.

Learn:

- sorted pair search
- 3Sum transformation
- greedy elimination in Container With Most Water

### Build

Implement `three_sum` without looking at an existing solution.

### Problems

1. LC 167 — Two Sum II
2. LC 11 — Container With Most Water
3. LC 15 — 3Sum

### Review

> For Container With Most Water, prove why moving the taller wall cannot improve the current limiting height.

---

## Day 3 — Hashing Fundamentals

### Study

Learn when lookup replaces repeated search.

Key transformation:

```text
Instead of:
"search previous elements"

Ask:
"what information from previous elements should I remember?"
```

### Build

```rust
fn frequencies(nums: &[i32]) -> HashMap<i32, usize>
fn contains_duplicate(nums: &[i32]) -> bool
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>
```

Use:

```rust
*freq.entry(value).or_insert(0) += 1;
```

### Problems

1. LC 217 — Contains Duplicate
2. LC 1 — Two Sum
3. LC 242 — Valid Anagram

### Review

> When is a fixed `[usize; 26]` array better than `HashMap<char, usize>`?

---

## Day 4 — Prefix State + HashMap

### Study

Understand:

```text
prefix[j] - prefix[i] = k
prefix[i] = prefix[j] - k
```

This turns a subarray search into a lookup problem.

### Build

```rust
fn prefix_sum(nums: &[i32]) -> Vec<i64>
fn subarray_sum(nums: &[i32], k: i32) -> i32
```

### Problems

1. LC 49 — Group Anagrams
2. LC 128 — Longest Consecutive Sequence
3. LC 560 — Subarray Sum Equals K

### Review

> Derive LC 560 algebraically without looking at code.

---

## Day 5 — Sliding Window Fundamentals

### Study

Distinguish:

- fixed-size windows
- variable-size windows
- window state
- invalid-window repair

### Build

Write a reusable ASCII character-frequency window.

```rust
let bytes = s.as_bytes();
let mut freq = [0usize; 128];
```

### Problems

1. LC 643 — Maximum Average Subarray I
2. LC 3 — Longest Substring Without Repeating Characters
3. LC 209 — Minimum Size Subarray Sum

### Review

> What changes when the right pointer moves? What changes when the left pointer moves?

---

## Day 6 — Advanced Sliding Window

### Study

Learn:

- at-most-K constraints
- frequency-based validity
- exact-K transformations
- anagram windows

### Build

Create:

```rust
fn longest_at_most_k_distinct(...)
```

Then derive:

```text
exactly(k) = at_most(k) - at_most(k - 1)
```

### Problems

1. LC 424 — Longest Repeating Character Replacement
2. LC 904 — Fruit Into Baskets
3. LC 438 — Find All Anagrams in a String

### Review

> Why can the left pointer move at most `n` times across the entire algorithm?

---

## Day 7 — Weekly Review / Buffer

**Lighter day unless catching up.**

### Rebuild from memory

- two-pointer template
- HashMap frequency counter
- prefix-sum hashmap
- variable sliding window

### Timed re-solves

1. LC 15 — 3Sum
2. LC 560 — Subarray Sum Equals K
3. LC 3 — Longest Substring Without Repeating Characters

### Weekly self-test

You should be able to answer:

- Why is two-pointer often `O(n)`?
- Why is LC 560 not a sliding-window problem for arbitrary integers?
- How do you recognize a monotonic window condition?
- When do you prefer a frequency array over a `HashMap`?

## Stretch bank

- LC 76 — Minimum Window Substring
- LC 239 — Sliding Window Maximum
- LC 992 — Subarrays with K Different Integers
