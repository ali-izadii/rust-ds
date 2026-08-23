# Week 5 — DSU, Dijkstra, MST & Greedy
**Days 29–35**

## Weekly goals

- Implement DSU with path compression and union-by-size.
- Implement Dijkstra with Rust `BinaryHeap`.
- Understand Prim vs Kruskal.
- Prove greedy algorithms using exchange arguments.
- Recognize greedy + heap combinations.

---

## Day 29 — Disjoint Set Union

### Study

Understand parent forests.

Core operations:

```text
find(x)
union(a, b)
same(a, b)
```

Optimizations:

- path compression
- union by size/rank

### Build

```rust
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}
```

### Problems

1. LC 684 — Redundant Connection
2. LC 721 — Accounts Merge
3. LC 1319 — Number of Operations to Make Network Connected

### Review

> Why does path compression preserve component identity?

---

## Day 30 — Dijkstra

### Study

Invariant:

> When the smallest tentative-distance node is removed from the priority queue, its distance is final if all edge weights are non-negative.

### Build

```rust
BinaryHeap<(Reverse<i64>, usize)>
```

Implement Dijkstra from scratch.

### Problems

1. LC 743 — Network Delay Time
2. LC 1514 — Path with Maximum Probability
3. LC 1631 — Path With Minimum Effort

### Review

> Construct an example showing why Dijkstra fails with a negative edge.

---

## Day 31 — Minimum Spanning Tree

### Study

Learn:

- MST definition
- cut property
- Kruskal
- Prim
- differences from shortest-path trees

### Build

Implement both:

```text
Kruskal = sort edges + DSU
Prim    = frontier + heap
```

### Problems

1. LC 1584 — Min Cost to Connect All Points
2. LC 1697 — Checking Existence of Edge Length Limited Paths
3. LC 1489 — Critical and Pseudo-Critical Edges in MST

### Review

> Why is an MST not necessarily a shortest-path tree from any root?

---

## Day 32 — Greedy Fundamentals

### Study

Learn exchange arguments.

Template:

```text
1. Assume optimal solution O.
2. Compare its first differing choice with greedy G.
3. Exchange O's choice for G.
4. Prove solution is not worse.
5. Repeat.
```

### Build

Interval scheduling by end time.

### Problems

1. LC 455 — Assign Cookies
2. LC 55 — Jump Game
3. LC 881 — Boats to Save People

### Review

> Write the exchange argument, not just the code.

---

## Day 33 — Greedy Scan Patterns

### Study

Learn:

- reachability frontier
- local replacement
- partition boundaries
- resource sufficiency

### Build

Reusable greedy scans.

### Problems

1. LC 45 — Jump Game II
2. LC 134 — Gas Station
3. LC 763 — Partition Labels

### Review

> What exact invariant makes your greedy choice irreversible?

---

## Day 34 — Greedy + Heap

### Study

Pattern:

```text
sort by when a candidate becomes available
maintain eligible candidates in heap
take best candidate greedily
```

### Build

Generic event scheduler with `BinaryHeap`.

### Problems

1. LC 1353 — Maximum Number of Events That Can Be Attended
2. LC 630 — Course Schedule III
3. LC 502 — IPO

### Review

> Why should the heap contain only currently eligible candidates?

---

## Day 35 — Weekly Review / Buffer

### Rebuild blind

- DSU
- Dijkstra
- Kruskal
- greedy interval scheduling

### Timed problems

1. LC 743
2. LC 1584
3. LC 55

### Weekly self-test

- BFS vs Dijkstra?
- Dijkstra vs MST?
- Kruskal vs Prim?
- Greedy vs DP?
- Why is DSU nearly constant amortized time?

## Stretch

- LC 1579
- LC 952
- LC 871
