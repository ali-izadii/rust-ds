# Week 4 — Backtracking & Graph Foundations
**Days 22–28**

## Weekly goals

- Model search as a decision tree.
- Separate traversal DFS from backtracking DFS.
- Implement graph adjacency lists, BFS and DFS.
- Prove BFS shortest-path correctness for unweighted graphs.
- Implement topological sorting using Kahn's algorithm.

---

## Day 22 — Backtracking Fundamentals

### Study

Every backtracking problem should identify:

```text
state
choices
constraints
base case
undo
```

### Build

```rust
fn dfs(
    start: usize,
    path: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
)
```

### Problems

1. LC 78 — Subsets
2. LC 46 — Permutations
3. LC 77 — Combinations

### Review

> What is the difference between "state" and "choice"?

---

## Day 23 — Backtracking Pruning

### Study

Learn:

- sort to group duplicates
- skip equivalent branches
- prune impossible partial states

### Build

Duplicate-safe permutation/subset generators.

### Problems

1. LC 39 — Combination Sum
2. LC 40 — Combination Sum II
3. LC 22 — Generate Parentheses

### Review

> Why is a branch safe to prune?

---

## Day 24 — Grid Backtracking

### Study

Learn:

- visited state
- choose → mark → recurse → unmark
- global vs path-local visitation

### Build

Generic 4-direction grid DFS.

### Problems

1. LC 79 — Word Search
2. LC 131 — Palindrome Partitioning
3. LC 51 — N-Queens

### Review

> Why must Word Search unmark a cell after returning from recursion?

---

## Day 25 — Graph Representation & Traversal

### Study

Adjacency list:

```rust
let graph = vec![Vec::<usize>::new(); n];
```

Learn:

- vertices vs edges
- visited array
- connected components

### Build

- iterative DFS
- BFS
- component counter

### Problems

1. LC 1971 — Find if Path Exists in Graph
2. LC 200 — Number of Islands
3. LC 695 — Max Area of Island

### Review

> Explain why adjacency-list traversal is `O(V + E)`.

---

## Day 26 — BFS Shortest Paths

### Study

BFS explores by distance layers.

Learn:

- single-source BFS
- multi-source BFS
- shortest path in grids

### Build

```rust
VecDeque<(usize, usize)>
```

multi-source BFS template.

### Problems

1. LC 994 — Rotting Oranges
2. LC 542 — 01 Matrix
3. LC 127 — Word Ladder

### Review

> Why is the first time BFS reaches a node its shortest distance?

---

## Day 27 — DAGs & Topological Sort

### Study

Learn:

- indegree
- Kahn's algorithm
- DFS cycle detection
- dependency scheduling

### Build

Both:

```text
toposort using indegree
toposort using DFS postorder
```

### Problems

1. LC 207 — Course Schedule
2. LC 210 — Course Schedule II
3. LC 785 — Is Graph Bipartite?

### Review

> Why does a cycle prevent a complete topological ordering?

---

## Day 28 — Weekly Review / Buffer

### Rebuild blind

- backtracking skeleton
- BFS
- iterative DFS
- Kahn topological sort

### Timed problems

1. LC 79
2. LC 200
3. LC 207

### Weekly self-test

Explain:

- backtracking visited state vs graph visited state
- BFS vs DFS
- DAG vs general directed graph
- why recursion depth may be dangerous in Rust

## Stretch

- LC 37 — Sudoku Solver
- LC 417 — Pacific Atlantic Water Flow
- LC 802 — Find Eventual Safe States
