# K-mer Concept and Proof of Concept in Rust

## Overview
This document describes the concept of [K-mers](https://en.wikipedia.org/wiki/K-mer) in bioinformatics, a powerful way to represent sequences of nucleotides in DNA or RNA. It also provides an implementation in Rust to generate k-mers both eagerly (storing all k-mers at once) and lazily (calculating each k-mer only when needed). The lazy evaluation technique is particularly useful for handling large DNA sequences efficiently.

## What is a K-mer?
A **k-mer** is a substring of length `k` within a DNA sequence. K-mers are commonly used in bioinformatics for sequence analysis, alignment, genome assembly, and more. Given a DNA sequence, extracting all possible k-mers allows us to analyze the sequence in smaller, fixed-length segments. This approach is beneficial for finding patterns, identifying repeats, or matching sequences across larger datasets.

### Example
For a sequence `ACGTACGT` and `k = 3`, the k-mers are:
* `ACG`
* `CGT`
* `GTA`
* `TAC`
* `ACG`
* `CGT`

This can be thought of as a sliding window of length `k` that moves across the sequence, producing overlapping segments.

## Proof of Concept (PoC) implementation in Rust
The Rust code below provides two implementations to generate k-mers from a sequence:
1. A **naive implementation** that eagerly returns all k-mers at once in a `Vec<&str>`.
2. A **lazy implementation** that returns an iterator, generating each k-mer only when it's needed, which is useful for handling large sequences without significant memory usage.

## Build and run the code

```bash
cargo run
```

## Output

```bash
K-mers: ["ACGTA", "CGTAC", "GTACG", "TACGT", "ACGTA", "CGTAC", "GTACG"]
K-mers lazy: ["ACGTA", "CGTAC", "GTACG", "TACGT", "ACGTA", "CGTAC", "GTACG"]
K-mers lazy evaluation: ACGTA
K-mers lazy evaluation: CGTAC
K-mers lazy evaluation: GTACG
K-mers lazy evaluation: TACGT
K-mers lazy evaluation: ACGTA
K-mers lazy evaluation: CGTAC
K-mers lazy evaluation: GTACG
```