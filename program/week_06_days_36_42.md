# Week 6 — Dynamic Programming
**Days 36–42**

## Weekly goals

- Design states before recurrences.
- Convert recursion → memoization → tabulation.
- Understand 0/1 and unbounded knapsack.
- Implement O(n²) and O(n log n) LIS.
- Derive grid and string DP.
- Introduce bitmask DP.

---

## Day 36 — DP Mental Model

### Study

For every DP problem write:

```text
State:
Transition:
Base case:
Evaluation order:
Answer:
```

### Build

Implement:

- Fibonacci recursion
- memoized Fibonacci
- bottom-up Fibonacci
- House Robber

### Problems

1. LC 70 — Climbing Stairs
2. LC 746 — Min Cost Climbing Stairs
3. LC 198 — House Robber

### Review

> What exactly does `dp[i]` mean?

---

## Day 37 — Knapsack & Coin Change

### Study

Understand:

```text
0/1 knapsack:
each item used at most once

unbounded knapsack:
item may be reused
```

Pay close attention to loop direction.

### Build

- 0/1 knapsack
- unbounded knapsack
- minimum coin count
- number of coin combinations

### Problems

1. LC 322 — Coin Change
2. LC 518 — Coin Change II
3. LC 416 — Partition Equal Subset Sum

### Review

> Why does iterating capacity backward prevent reusing an item in 0/1 knapsack?

---

## Day 38 — LIS & Sequence DP

### Study

Learn:

```text
dp[i] = best solution ending at i
```

Then learn patience-sorting / binary-search optimization.

### Build

Both LIS versions:

```text
O(n²)
O(n log n)
```

### Problems

1. LC 300 — Longest Increasing Subsequence
2. LC 673 — Number of LIS
3. LC 354 — Russian Doll Envelopes

### Review

> What does the `tails` array mean? It is not itself necessarily an LIS.

---

## Day 39 — Grid DP

### Study

Learn dependencies such as:

```text
dp[r][c] ← dp[r-1][c], dp[r][c-1]
```

### Build

Implement both 2D and flattened representations.

### Problems

1. LC 62 — Unique Paths
2. LC 64 — Minimum Path Sum
3. LC 221 — Maximal Square

### Review

> Draw dependency arrows before choosing iteration order.

---

## Day 40 — String DP

### Study

For two strings, think in prefixes:

```text
dp[i][j] = answer involving first i chars of A
           and first j chars of B
```

### Build

- LCS
- edit distance

### Problems

1. LC 1143 — Longest Common Subsequence
2. LC 583 — Delete Operation for Two Strings
3. LC 72 — Edit Distance

### Review

> Explain every transition in Edit Distance in plain language.

---

## Day 41 — Bitmask DP

### Study

A subset of `n` elements can be represented by an `n`-bit integer.

Use:

```rust
mask & (1usize << i)
mask | (1usize << i)
```

### Build

- subset enumeration
- state over `(node, mask)`
- memo table indexed by mask

### Problems

1. LC 698 — Partition to K Equal Sum Subsets
2. LC 847 — Shortest Path Visiting All Nodes

### Review

> What does each bit represent?

---

## Day 42 — Weekly Review / Buffer

### Rebuild blind

- 0/1 knapsack
- coin change
- LIS O(n log n)
- LCS

### Timed problems

1. LC 322
2. LC 300
3. LC 1143

### Weekly self-test

For a new DP problem, can you state the state and recurrence before coding?

## Stretch

- LC 115 — Distinct Subsequences
- LC 10 — Regular Expression Matching
- LC 44 — Wildcard Matching
