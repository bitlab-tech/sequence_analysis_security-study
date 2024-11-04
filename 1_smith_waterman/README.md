# Smith-Waterman Local Sequence Alignment in Rust

This implements a simple POC of the [Smith-Waterman algorithm](https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm) in Rust for local sequence alignment of two DNA sequences. The algorithm finds the most similar subsequences between two input sequences, which is useful in bioinformatics for identifying regions of similarity that may indicate functional, structural, or evolutionary relationships.

## Overview

The **Smith-Waterman algorithm** is a dynamic programming approach that performs local sequence alignment, finding the optimal matching region between two sequences. It uses scoring parameters for matches, mismatches, and gaps, making it sensitive to insertions, deletions, and substitutions often found in biological sequences.

### Key Features

- **Local Alignment**: Finds the best-matching local regions, allowing unrelated parts of the sequences to remain unaligned.
- **Adjustable Scoring**: Configurable parameters for match score, mismatch penalty, and gap penalty.
- **Efficient Implementation**: Written in Rust for high performance and memory efficiency.

## Algorithm Steps

1. **Initialize the Scoring Matrix**: Create a matrix to store alignment scores for subsequences.
2. **Matrix Filling**: Populate the matrix based on:
   - **Match/Mismatch**: Adds the match score if characters are the same, or mismatch penalty if they differ.
   - **Gap**: Applies a gap penalty for insertions or deletions.
3. **Traceback**: Starting from the highest score in the matrix, trace back through the matrix to construct the best alignment. Stop when reaching a cell with a score of zero, indicating the boundary of the local alignment.

## Code Usage

### Function Signature

```rust
fn smith_waterman(seq1: &str, seq2: &str, match_score: i32, mismatch_penalty: i32, gap_penalty: i32) -> (String, String, i32)
```

### Parameters
- `seq1`: String representing the query sequence to be aligned
- `seq2`: String representing the target sequence to be aligned.
- `match_score`: An integer score for matching characters.
- `mismatch_penalty`: An integer penalty for mismatched characters.
- `gap_penalty`: An integer penalty for introducing a gap.

### Return Value
Returns a tuple with:
- `align1`: The aligned version of seq1.
- `align2`: The aligned version of seq2.
- `max_score`: The score of the optimal local alignment.

### Example Usage
```rust
fn main() {
    let seq1 = "AGACTAGTTAC";
    let seq2 = "CGTGAATTCAT";
    let (align1, align2, score) = smith_waterman(&seq1, &seq2, 2, -1, -1);

    println!("Aligned Sequences:");
    println!("{}", align1);
    println!("{}", align2);
    println!("Alignment Score: {}", score);
}
```

### Expected Output
```
Aligned Sequences:
C-T-AGTT-A
CGTGAATTCA
Alignment Score: 8
```

In this example, the algorithm finds the best alignment between `seq1` and `seq2` using a match score of `+2`, mismatch penalty of `-1`, and gap penalty of `-1`. The output provides the aligned subsequences and the maximum alignment score.

## Algorithm Complexity
- Time Complexity: $O(m \times n)$, where `m` and `n` are the lengths of `seq1` and `seq2`.
- Space Complexity: $O(m \times n)$ due to the scoring matrix storage.

## Further Improvements
- Parallelization: For longer sequences, matrix filling can be parallelized for improved performance.
- Memory Optimization: Use a linear space optimization technique for reduced memory consumption.