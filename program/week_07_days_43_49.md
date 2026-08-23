# Week 7 — Tries, String Algorithms, Bits & Number Theory
**Days 43–49**

## Weekly goals

- Implement a Trie using indexed nodes.
- Implement KMP prefix function from memory.
- Understand rolling hash and collisions.
- Become fluent with XOR and bitmask identities.
- Implement GCD, sieve and prime factorization.

---

## Day 43 — Trie

### Study

Understand shared prefixes.

Preferred Rust representation:

```rust
struct Node {
    next: [Option<usize>; 26],
    terminal: bool,
}

struct Trie {
    nodes: Vec<Node>,
}
```

### Build

Implement:

- insert
- contains
- starts_with

### Problems

1. LC 208 — Implement Trie
2. LC 211 — Add and Search Word
3. LC 648 — Replace Words

### Review

> Why are integer node indices often easier than references in Rust?

---

## Day 44 — KMP

### Study

Understand:

```text
pi[i] = length of longest proper prefix
        that is also suffix for s[0..=i]
```

### Build

```rust
fn prefix_function(s: &[u8]) -> Vec<usize>
fn kmp_find(text: &[u8], pattern: &[u8]) -> Option<usize>
```

### Problems

1. LC 28 — Find First Occurrence
2. LC 459 — Repeated Substring Pattern
3. LC 1392 — Longest Happy Prefix

### Review

> When a mismatch occurs, why is restarting at pattern index zero unnecessary?

---

## Day 45 — Rabin-Karp / Rolling Hash

### Study

Learn:

- polynomial rolling hash
- rolling substring update
- collision risk
- double hashing concept

### Build

Rabin-Karp substring search.

### Problems

1. LC 686 — Repeated String Match
2. LC 214 — Shortest Palindrome

### Review

> Why does equal hash not mathematically guarantee equal strings?

---

## Day 46 — Bit Manipulation Fundamentals

### Study

Memorize by derivation, not rote:

```text
x ^ x = 0
x ^ 0 = x
x & (x - 1) removes lowest set bit
```

### Build

Bit utility module:

```rust
fn is_power_of_two(...)
fn popcount_manual(...)
fn lowbit(...)
```

### Problems

1. LC 191 — Number of 1 Bits
2. LC 136 — Single Number
3. LC 338 — Counting Bits
4. LC 268 — Missing Number

### Review

> Derive `x & (x - 1)` using a binary example.

---

## Day 47 — Advanced Bits

### Study

Learn:

- bit partitions
- XOR groups
- subset masks
- range bit patterns

### Build

Subset-mask iterator.

### Problems

1. LC 260 — Single Number III
2. LC 421 — Maximum XOR
3. LC 201 — Bitwise AND of Numbers Range

### Review

> How does the lowest set bit separate two XOR groups?

---

## Day 48 — Number Theory

### Study

Learn:

- Euclid's algorithm
- LCM
- sieve
- trial division
- prime factorization

### Build

```rust
fn gcd(a: u64, b: u64) -> u64
fn sieve(n: usize) -> Vec<bool>
fn factorize(n: u64) -> Vec<(u64, usize)>
```

### Problems

1. LC 1071 — Greatest Common Divisor of Strings
2. LC 204 — Count Primes
3. LC 1492 — Kth Factor of n

### Review

> Prove `gcd(a, b) = gcd(b, a % b)`.

---

## Day 49 — Weekly Review / Buffer

### Rebuild blind

- Trie
- KMP prefix function
- low-bit functions
- GCD
- sieve

### Timed problems

1. LC 208
2. LC 421
3. LC 204

### Weekly self-test

- Trie vs HashMap of complete words?
- Trie vs KMP?
- KMP vs rolling hash?
- XOR vs addition?
- sieve vs per-number primality checks?

## Stretch

- LC 212 — Word Search II
- LC 336 — Palindrome Pairs
- LC 1611 — Minimum One Bit Operations
