# Week 3 — Intervals, Trees & Heaps
**Days 15–21**

## Weekly goals

- Recognize interval sorting patterns.
- Define recursive tree state before coding.
- Understand BST invariants.
- Implement a binary heap from scratch.
- Use `BinaryHeap<Reverse<T>>` as a min-heap.

---

## Day 15 — Intervals

### Study

Learn:

- sort-by-start → merging
- sort-by-end → scheduling
- sweep-line events
- inclusive vs half-open intervals

### Build

```rust
fn merge_intervals(...)
fn interval_intersection(...)
```

### Problems

1. LC 56 — Merge Intervals
2. LC 57 — Insert Interval
3. LC 435 — Non-overlapping Intervals

### Review

> Why does sorting turn global overlap relationships into local comparisons?

---

## Day 16 — Tree Traversal

### Study

Understand:

- preorder
- inorder
- postorder
- subtree recursion

### Build

Create your own generic binary tree and implement all three traversals recursively.

### Problems

1. LC 104 — Maximum Depth
2. LC 226 — Invert Binary Tree
3. LC 543 — Diameter of Binary Tree

### Review

> What information does each recursive call return?

---

## Day 17 — BST & Iterative Traversal

### Study

BST invariant:

```text
every value in left subtree < node
every value in right subtree > node
```

Learn iterative DFS with an explicit stack.

### Build

- iterative inorder
- BST search
- BST insert

### Problems

1. LC 98 — Validate Binary Search Tree
2. LC 230 — Kth Smallest Element in BST
3. LC 102 — Binary Tree Level Order Traversal

### Review

> Why is comparing a node only with its parent insufficient for validating a BST?

---

## Day 18 — Tree Composition

### Study

Learn "return useful information upward":

```text
left state
right state
current combination
```

### Build

Reusable DFS templates for:

- height
- subtree size
- path aggregate
- ancestor search

### Problems

1. LC 236 — Lowest Common Ancestor
2. LC 105 — Construct Tree from Preorder and Inorder
3. LC 124 — Binary Tree Maximum Path Sum

### Review

> Define the return value of your recursion before writing the recursive body.

---

## Day 19 — Heap From Scratch

### Study

Learn:

- complete binary tree
- array representation
- parent/child index formulas
- sift-up
- sift-down
- heapify

### Build

Implement:

```rust
struct BinaryHeapMin<T: Ord> {
    data: Vec<T>,
}
```

with:

- push
- peek
- pop
- heapify

### Problems

1. LC 1046 — Last Stone Weight
2. LC 215 — Kth Largest Element
3. LC 973 — K Closest Points to Origin

### Review

> Why can heap insertion repair the invariant along only one root-to-leaf path?

---

## Day 20 — Heap Applications

### Study

Patterns:

- top K
- multiway merge
- frontier search
- two heaps

### Build

Use:

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
```

Build a streaming median structure.

### Problems

1. LC 347 — Top K Frequent Elements
2. LC 373 — Find K Pairs with Smallest Sums
3. LC 295 — Find Median from Data Stream

### Review

> When does a heap beat sorting the entire dataset?

---

## Day 21 — Weekly Review / Buffer

### Rebuild blind

- interval merge
- recursive tree height
- iterative inorder
- binary heap push/pop

### Timed problems

1. LC 98
2. LC 56
3. LC 215

### Weekly self-test

- Why is a heap not a sorted array?
- How does tree recursion differ from graph DFS?
- When should intervals be sorted by end instead of start?

## Stretch

- LC 84 — Largest Rectangle in Histogram
- LC 297 — Serialize and Deserialize Binary Tree
- LC 857 — Minimum Cost to Hire K Workers
