# Homomorphic PRS Calculator in Rust with TFHE

This Rust program implements a **[Polygenic Risk Scores (PRS)](https://en.wikipedia.org/wiki/Polygenic_score)** calculator using the [**TFHE (Fully Homomorphic Encryption)**](https://github.com/zama-ai/tfhe-rs) library. It performs privacy-preserving computations on encrypted genomic data, ensuring that sensitive information remains secure throughout the process. However, due to the computational complexity of TFHE, the execution speed is notably **slow**, particularly for large datasets.

## Features

- **Secure PRS Calculation**: Computes PRS using TFHE homomorphic encryption, allowing operations on encrypted data without decryption.
- **Parallel Processing**: Utilizes the [rayon](https://github.com/rayon-rs/rayon) crate for parallel computation across individuals.
- **CSV Input**: Reads genotype and phenotype data from CSV files.
- **Integer Scaling**: Handles floating-point phenotype coefficients by scaling them to integers for encryption.
- **Slow Speed**: Emphasized due to the heavy computational overhead of TFHE, making it less suitable for time-sensitive applications.

## Prerequisites

To run this program, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust's package manager, included with Rust)
- Dependencies listed in `Cargo.toml` (installed automatically via `cargo build`)

## Usage

- Run the program:
    ```bash
    cargo run -r
    ```
- Output:

    ```bash
    Server execution time: 45.32s
    Results: [1.234, 2.567, ...]
    ```

## License
Unlicensed, provided as-is for educational use.