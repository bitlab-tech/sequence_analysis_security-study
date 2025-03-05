# Binary Encoding for Genomic Sequences in Rust

## Overview

This project demonstrates how to binary encode genomic sequences using Rust. Genomic sequences consist of four nucleotide bases: `A`, `C`, `G`, and `T`. Representing these sequences in binary format can make computation, comparison, and storage more efficient, especially for large datasets used in bioinformatics.

### Binary Encoding Concept

We assign a 2-bit binary representation to each nucleotide as follows:
- `A` → `0b00`
- `C` → `0b01`
- `G` → `0b10`
- `T` → `0b11`

For example, the k-mer (short DNA sequence) `ACGT` is encoded as:

```
A → 0b00
C → 0b01
G → 0b10
T → 0b11
Combined: 0b00011011
```


### Why Binary Encoding?

1. **Compact Representation**:
   - Each nucleotide requires only 2 bits, reducing memory usage compared to standard character storage (which uses 8 bits per nucleotide).
   - Example: A 16-nucleotide k-mer can fit into a 32-bit integer (`u32`).

2. **Efficient Computation**:
   - Operations like comparison, searching, and manipulation can be performed directly on binary values, which are computationally faster than handling strings or characters.

3. **Scalability**:
   - Large genomic datasets (e.g., whole-genome sequencing) benefit from reduced memory usage and faster processing.

4. **Compatibility**:
   - Binary-encoded data can be easily used in algorithms requiring numerical inputs or optimizations.

## Implementation

### Dynamic Bit-Width Support

The program dynamically selects the numeric type (`u8`, `u16`, `u32`, `u64`, or `u128`) based on the k-mer length to minimize memory usage.

### Bitwise Operations

Binary operations such as shifting (`<<`) and bitwise OR (`|`) construct the binary representation of the sequence.

### Error Handling

The program validates input sequences to ensure they contain only valid nucleotide characters (`A`, `C`, `G`, `T`).


## Key Functions

### `init_kmer_type`

```rust
fn init_kmer_type(len: usize, val: u8) -> KmerType
```
Initializes a KmerType instance with the appropriate numeric type based on the k-mer length.

### `binary_encode`

```rust
fn binary_encode(kmer: &str) -> KmerType
```

Encodes a genomic sequence into its binary representation:

- Each nucleotide is converted to its 2-bit value.
- The bits are shifted and combined into a single value.

## Example Input and Output

For the k-mer `ACGTACGTACGTACGT` (length 16):

- **Binary encoded value (in binary)**: `0b00011011000110110001101100011011`
- **Binary encoded value (in decimal)**: `732013653`

## Applications

1. **Genome Analysis**:
   - Efficiently compare genomic sequences or motifs.

2. **Data Compression**:
   - Store large genomic datasets in binary format to save space.

3. **Pattern Matching**:
   - Search for specific sequences in binary-encoded data using bitwise operations.

4. **High-Performance Computing**:
   - Optimize sequence alignment, SNP detection, or other bioinformatics algorithms.

## Future Work

- **Extend to RNA Sequences**: Add support for RNA bases (`A`, `C`, `G`, `U`).
- **Parallel Processing**: Enable multi-threaded encoding for large datasets.
- **Integration with Databases**: Store binary-encoded sequences for fast retrieval.

## Acknowledgments

Special thanks to the Rust community for providing excellent resources and support. This project was inspired by the need for efficient genomic data handling in bioinformatics.
