# Week 2 — Binary Search, Linked Lists, Stacks & Queues
**Days 8–14**

## Weekly goals

- Treat binary search as boundary finding over a monotone predicate.
- Implement lower/upper bound from scratch.
- Become comfortable with `Option<Box<Node>>`.
- Understand why `.take()` is useful in Rust linked-list mutation.
- Recognize monotonic-stack problems.
- Use `VecDeque` correctly for queues.

---

## Day 8 — Sorting & Binary Search

### Study

Learn the invariant:

```text
one side is known false
the other side is known true
binary search shrinks the unknown boundary
```

### Build

```rust
fn lower_bound<T: Ord>(a: &[T], x: &T) -> usize
fn upper_bound<T: Ord>(a: &[T], x: &T) -> usize
```

Also implement merge sort.

### Problems

1. LC 704 — Binary Search
2. LC 35 — Search Insert Position
3. LC 34 — Find First and Last Position

### Review

> State your binary-search invariant before writing code.

---

## Day 9 — Binary Search on Answer

### Study

Recognition pattern:

```text
minimize/maximize answer
        ↓
candidate answer x
        ↓
can(x)?
        ↓
monotone true/false
        ↓
binary search
```

### Build

Create a generic integer boundary-search function.

### Problems

1. LC 875 — Koko Eating Bananas
2. LC 1011 — Capacity To Ship Packages Within D Days
3. LC 1482 — Minimum Number of Days to Make m Bouquets

### Review

> Why must the feasibility predicate be monotone?

---

## Day 10 — Linked Lists & Rust Ownership

### Study

Focus on:

```rust
Option<Box<ListNode>>
```

Learn:

- moving ownership
- `Option::take`
- `as_ref`
- `as_mut`

### Build

Implement your own:

```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
```

Operations:

- push front
- pop front
- reverse

### Problems

1. LC 206 — Reverse Linked List
2. LC 21 — Merge Two Sorted Lists
3. LC 141 — Linked List Cycle

### Review

> What value owns each node at each stage of reversal?

---

## Day 11 — Linked-List Pointer Surgery

### Study

Learn:

- fast/slow pointers
- dummy nodes
- reversing subranges
- splitting and reconnecting lists

### Build

Implement:

```rust
fn reverse_list(...)
fn reverse_between(...)
```

### Problems

1. LC 19 — Remove Nth Node From End
2. LC 24 — Swap Nodes in Pairs
3. LC 143 — Reorder List

### Review

> Before modifying `next`, where is the remainder of the list stored?

---

## Day 12 — Stack & Queue Fundamentals

### Study

Use:

```rust
Vec<T>       // stack
VecDeque<T>  // queue
```

Learn:

- LIFO
- FIFO
- expression evaluation
- auxiliary stack state

### Build

Implement:

- stack using queue
- queue using stacks
- min-stack

### Problems

1. LC 20 — Valid Parentheses
2. LC 155 — Min Stack
3. LC 150 — Evaluate Reverse Polish Notation

### Review

> What ordering property makes a stack useful for nested structures?

---

## Day 13 — Monotonic Stack

### Study

Core idea:

> Keep only unresolved candidates. Pop anything that can no longer matter.

### Build

Implement a generic "next greater index" routine.

### Problems

1. LC 496 — Next Greater Element I
2. LC 739 — Daily Temperatures
3. LC 901 — Online Stock Span

### Review

> Prove that every index is pushed and popped at most once.

---

## Day 14 — Weekly Review / Buffer

### Rebuild blind

- lower bound
- binary search on predicate
- linked-list reversal
- monotonic stack

### Timed problems

1. LC 11 — Container With Most Water
2. LC 875 — Koko Eating Bananas
3. LC 739 — Daily Temperatures

### Rust self-test

Explain:

- why `String` cannot be indexed
- why `.take()` moves out of an `Option`
- when `VecDeque` is preferable to `Vec`

## Stretch

- LC 84 — Largest Rectangle in Histogram
- LC 25 — Reverse Nodes in k-Group
- LC 410 — Split Array Largest Sum
