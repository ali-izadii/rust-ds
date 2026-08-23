# Week 9 — Mock Contests & Final Assessment
**Days 56–60**

This is the certification phase.

The goal is no longer acquiring algorithms. It is:

- fast recognition
- correct category composition
- implementation from memory
- proof/invariant quality
- Rust fluency under time pressure

---

# Day 56 — Mock Contest #1

**Heavy day — approximately 3 hours.**

## Rules

- No notes.
- No external solutions.
- No personal templates.
- Use Rust.
- Before submitting, write complexity mentally.
- If stuck for 25–30 minutes, move on.

## Problems

1. LC 410 — Split Array Largest Sum
   - binary search + greedy feasibility
2. LC 212 — Word Search II
   - trie + DFS
3. LC 1631 — Path With Minimum Effort
   - graph + minimax Dijkstra
4. LC 84 — Largest Rectangle in Histogram
   - monotonic stack

## Post-contest diagnosis

Classify every failure:

```text
R = recognition failure
P = proof/invariant failure
A = algorithm derivation failure
I = implementation failure
RS = Rust-specific failure
T = time-management failure
```

Record the classification.

---

# Day 57 — Mock #1 Repair

## Study

Read only material corresponding to failures from Day 56.

Do not broadly review everything.

## Build

Rewrite failed algorithm templates from memory.

Examples:

- binary search on predicate
- trie
- Dijkstra
- monotonic stack

## Practice

- Re-solve two problems failed on Day 56.
- LC 1976 — Number of Ways to Arrive at Destination

## Review

For each previous mistake answer:

> What signal did I miss?

> What invariant should I have written?

> Was the issue algorithmic or Rust-specific?

---

# Day 58 — Mock Contest #2

**Heavy day — approximately 3 hours.**

## Rules

Same as Day 56.

## Problems

1. LC 239 — Sliding Window Maximum
   - sliding window + monotonic deque
2. LC 1697 — Checking Existence of Edge Length Limited Paths
   - sorting + DSU
3. LC 502 — IPO
   - greedy + heap
4. LC 847 — Shortest Path Visiting All Nodes
   - BFS + bitmask state

## Post-contest evaluation

Compare against Mock #1:

- recognition time
- first-attempt correctness
- Rust compile errors
- number of algorithm changes
- time spent debugging

---

# Day 59 — Final Rehearsal

Do not learn new algorithms.

## Study

Read only your own notes:

```text
pattern signal
invariant
complexity
Rust gotcha
```

## Blind implementation

Without reference implement:

1. binary search
2. binary search on answer
3. DSU
4. BFS
5. Dijkstra
6. Trie
7. KMP
8. Fenwick tree

## Practice

1. LC 1851 — Minimum Interval to Include Each Query
2. LC 329 — Longest Increasing Path in a Matrix
3. LC 1552 — Magnetic Force Between Two Balls

## Oral recognition drill

Given each phrase, name the likely family immediately:

```text
"shortest path, all edges weight 1"
"minimum possible maximum"
"k distinct elements in subarray"
"dependencies"
"connect all nodes minimum cost"
"dynamic prefix sums with updates"
"many prefix queries over words"
"n <= 20 and state includes visited nodes"
"nearest greater to the right"
```

Expected answers:

```text
BFS
binary search on answer
sliding window + hashing
topological sort
MST
Fenwick/segment tree
Trie
bitmask DP
monotonic stack
```

---

# Day 60 — Final Assessment

This is intentionally heavier than a normal study day.

# Phase A — Blind Implementation Test

**45 minutes.**

Implement without reference:

```text
binary search on predicate
DSU
BFS
Dijkstra
binary heap
Trie
KMP prefix function
Fenwick tree
segment tree
0/1 knapsack
```

Try not to compile until each function is substantially complete.

The goal is to detect whether you understand the structure or depend on compiler-driven trial and error.

---

# Phase B — Mixed Contest

**4 hours.**

## Problems

1. LC 15 — 3Sum
   - sorting + two pointers

2. LC 76 — Minimum Window Substring
   - sliding window + hashing

3. LC 98 — Validate Binary Search Tree
   - trees + recursive bounds

4. LC 207 — Course Schedule
   - graph + topological sort

5. LC 743 — Network Delay Time
   - Dijkstra

6. LC 416 — Partition Equal Subset Sum
   - 0/1 knapsack

7. LC 1697 — Checking Existence of Edge Length Limited Paths
   - offline sorting + DSU

8. LC 315 — Count of Smaller Numbers After Self
   - coordinate compression + Fenwick/segment tree

---

# Certification Levels

## Level 1 — Working Knowledge

```text
4/8 accepted
most categories recognized correctly
can use references for advanced DS
```

You understand the curriculum but still need repetition.

## Level 2 — Interview Ready

Target:

```text
6/8 accepted
no external references
all chosen patterns correctly identified
no major complexity mistakes
DSU/Dijkstra/Trie/Fenwick from memory
can explain correctness for at least five solutions
```

## Level 3 — Strong Algorithmic Fluency

```text
7/8 accepted
clean Rust
within the time limit
few debugging iterations
correct complexity analysis
clear invariants/proofs
```

## Level 4 — Mastery Track

You should additionally be able to:

- solve unfamiliar variations rather than only known problems
- derive templates instead of recalling exact code
- identify cross-category compositions quickly
- explain why rejected approaches fail
- implement common structures generically in Rust
- perform well in repeated timed contests

---

# Final Category Checklist

Before declaring the 60-day program complete, verify:

- [ ] Arrays / Two Pointers
- [ ] Hashing
- [ ] Sliding Window
- [ ] Sorting
- [ ] Binary Search
- [ ] Linked Lists
- [ ] Stacks / Queues
- [ ] Monotonic Stack / Queue
- [ ] Intervals
- [ ] Trees / BST
- [ ] Heaps
- [ ] Backtracking
- [ ] BFS / DFS
- [ ] Topological Sort
- [ ] DSU
- [ ] Dijkstra
- [ ] MST
- [ ] Greedy
- [ ] Dynamic Programming
- [ ] Trie
- [ ] KMP / Rolling Hash
- [ ] Bit Manipulation
- [ ] Number Theory
- [ ] Fenwick Tree
- [ ] Segment Tree
- [ ] Mixed pattern recognition

---

# What to Do After Day 60

Do not immediately start another giant curriculum.

Instead:

1. Enter 1–2 timed contests each week.
2. Maintain a mistake log.
3. Re-solve failed problems after:
   - 1 day
   - 7 days
   - 30 days
4. Replace memorized templates with derivable invariants.
5. Start Codeforces / AtCoder problems where categories are not labeled.
6. Keep all implementations in Rust.

Your next objective is not "learn more algorithms."

It is:

> Reduce the time between reading an unfamiliar problem and identifying the correct structural model.
