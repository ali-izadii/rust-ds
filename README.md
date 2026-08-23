# Rust Data Structures & Algorithms Roadmap

A 60-day, Rust-first study plan for building practical data-structure and algorithm skills. The roadmap combines concept study, from-scratch implementations, LeetCode practice, review prompts, and timed assessments.

It is a curriculum repository, not a Rust crate: there is no `Cargo.toml` or application to build. Create your own practice project or template library as you work through the exercises.

## Goals

By the end of the plan, you should be able to:

- Recognize common algorithmic patterns and choose an appropriate approach.
- State invariants and time/space complexity before implementation.
- Implement core data structures and algorithms in idiomatic Rust.
- Handle Rust ownership and collection choices in linked-list, graph, and tree problems.
- Solve mixed problems under timed, contest-like conditions.

## Curriculum

| Week | Days | Topics | Guide |
| --- | --- | --- | --- |
| 1 | 1–7 | Arrays, hashing, two pointers, prefix sums, sliding window | [Week 1](program/week_01_days_01_07.md) |
| 2 | 8–14 | Binary search, linked lists, stacks, queues | [Week 2](program/week_02_days_08_14.md) |
| 3 | 15–21 | Intervals, trees, binary search trees, heaps | [Week 3](program/week_03_days_15_21.md) |
| 4 | 22–28 | Backtracking, graphs, BFS/DFS, topological sort | [Week 4](program/week_04_days_22_28.md) |
| 5 | 29–35 | Disjoint-set union, Dijkstra, MST, greedy algorithms | [Week 5](program/week_05_days_29_35.md) |
| 6 | 36–42 | Dynamic programming, knapsack, LIS, grid/string/bitmask DP | [Week 6](program/week_06_days_36_42.md) |
| 7 | 43–49 | Tries, string algorithms, bits, number theory | [Week 7](program/week_07_days_43_49.md) |
| 8 | 50–55 | Modular arithmetic, segment trees, Fenwick trees, integration | [Week 8](program/week_08_days_50_55.md) |
| 9 | 56–60 | Mock contests, targeted repair, and final assessment | [Week 9](program/week_09_days_56_60.md) |

## How to use it

1. Work through one day at a time, in order. Each day lists what to study, small Rust functions or data structures to build, practice problems, and a review question.
2. Implement the listed “Build” exercises before consulting a solution. Keep them in a separate Rust project so that they compile and can be tested.
3. For every solution, record the invariant, complexity, and any Rust-specific ownership or borrowing issue you encountered.
4. Use each weekly review/buffer day to rebuild the named templates from memory and repair gaps rather than starting new material.
5. Reserve the final week for timed work. Treat the mock-contest rules as part of the exercise: no notes, no external solutions, and deliberate post-contest diagnosis.

## Suggested practice setup

Install the stable Rust toolchain with [rustup](https://rustup.rs/), then make a companion project for your implementations:

```bash
cargo new rust-algorithm-practice
cd rust-algorithm-practice
cargo test
```

As the roadmap reaches Week 8, organize only the algorithms you can recreate confidently into modules such as `binary_search`, `dsu`, `graph`, `dijkstra`, `fenwick`, `segment_tree`, `trie`, `kmp`, and `math`.

## What each day contains

- **Study** explains the core model, invariant, or proof idea.
- **Build** names a small implementation to write in Rust.
- **Problems** supplies focused LeetCode exercises.
- **Review** asks a question intended to test understanding rather than recall.
- **Weekly review/buffer** days provide rebuild, timed-practice, and stretch exercises.

## Progress tracking

Use the checklist below in your own notes or copy it into an issue:

```text
[ ] Completed the day’s study and build exercise
[ ] Solved the assigned problems without a copied solution
[ ] Wrote down the invariant and complexity
[ ] Recorded Rust-specific mistakes or lessons
[ ] Rebuilt the weekly templates from memory
```

## License

No license file is currently included. Add one before reusing or distributing the material under specific terms.
