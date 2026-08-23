# Week 8 — Modular Arithmetic, Segment Trees, Fenwick Trees & Integration
**Days 50–55**

## Weekly goals

- Implement fast exponentiation.
- Implement Fenwick and segment trees from scratch.
- Understand coordinate compression.
- Start solving cross-category problems deliberately.
- Build your personal Rust algorithm-template library.

---

## Day 50 — Modular Arithmetic

### Study

Learn exponentiation by squaring:

```text
a^13
= a^(8 + 4 + 1)
```

Each exponent bit determines whether a power contributes.

### Build

```rust
fn pow(mut base: i64, mut exp: u64) -> i64
fn mod_pow(mut base: u64, mut exp: u64, modulo: u64) -> u64
```

Use `u128` where multiplication may overflow.

### Problems

1. LC 50 — Pow(x, n)
2. LC 372 — Super Pow
3. LC 878 — Nth Magical Number

### Review

> Why does repeated squaring reduce exponentiation to `O(log n)` multiplications?

---

## Day 51 — Segment Tree

### Study

Understand:

- interval decomposition
- associative combine operation
- tree nodes representing ranges
- point updates
- range queries

### Build

Implement a sum segment tree:

```rust
struct SegTree {
    n: usize,
    tree: Vec<i64>,
}
```

Support:

```text
build
update(index, value)
query(left, right)
```

### Problems

1. LC 303 — Range Sum Query Immutable
2. LC 307 — Range Sum Query Mutable
3. LC 699 — Falling Squares

### Review

> Why is a query representable using only `O(log n)` relevant tree nodes in the standard traversal?

---

## Day 52 — Fenwick Tree

### Study

Understand `lowbit`.

Concept:

```text
bit[i] summarizes a block whose size is lowbit(i)
```

### Build

```rust
struct Fenwick {
    bit: Vec<i64>,
}
```

Support:

```text
add(index, delta)
prefix_sum(index)
range_sum(left, right)
```

### Problems

1. LC 1409 — Queries on a Permutation With Key
2. LC 315 — Count of Smaller Numbers After Self
3. LC 1649 — Create Sorted Array through Instructions

### Review

> Explain exactly what range is represented by `bit[i]`.

---

## Day 53 — Coordinate Compression & Advanced Range Queries

### Study

Use compression when:

- values are huge
- only relative order matters
- Fenwick/segment tree requires dense indices

### Build

```rust
fn compress(values: &[i64]) -> Vec<usize>
```

Then combine compression + Fenwick.

### Problems

1. LC 327 — Count of Range Sum
2. LC 493 — Reverse Pairs

### Review

> What properties does coordinate compression preserve, and what does it destroy?

---

## Day 54 — Mixed: Sliding Window + Hashing

### Study

Do not label the whole problem with one category.

Instead split:

```text
window mechanics
+
window validity state
```

### Problems

1. LC 76 — Minimum Window Substring
2. LC 992 — Subarrays with K Different Integers
3. LC 30 — Substring with Concatenation of All Words

### Build

No new DS. Rewrite a generic frequency-window helper.

### Review

> Which part of each solution is sliding window, and which part is hashing?

---

## Day 55 — Weekly Review / Buffer

### Rebuild blind

- `mod_pow`
- Fenwick tree
- segment tree
- coordinate compression

### Timed problems

1. LC 307
2. LC 315
3. LC 76

### Personal Rust template library

Create:

```text
src/
  binary_search.rs
  dsu.rs
  graph.rs
  dijkstra.rs
  heap.rs
  fenwick.rs
  segment_tree.rs
  trie.rs
  kmp.rs
  math.rs
```

Only add algorithms that you can currently implement from memory.

## Stretch

- LC 715 — Range Module
- LC 732 — My Calendar III
- LC 2179 — Count Good Triplets in an Array
