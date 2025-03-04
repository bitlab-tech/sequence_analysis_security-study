# Calculate polygenic Risk Score homomorphically with CKKS 

This Go program implements a [**Polygenic Risk Score (PRS)**](https://en.wikipedia.org/wiki/Polygenic_score) calculator using the [**CKKS homomorphic encryption scheme**](https://eprint.iacr.org/2016/421.pdf) from the [Lattigo library](https://github.com/tuneinsight/lattigo). By leveraging CKKS, the program performs secure and privacy-preserving computations on encrypted genomic data, ensuring that sensitive information remains protected throughout the process. Notably, the implementation has a **fast runtime**, making it efficient even when handling large datasets.

## Why Use CKKS for PRS?

- **Privacy**: Genomic data is highly sensitive, and CKKS ensures it remains encrypted during computation, protecting against unauthorized access.

- **Efficiency**: This implementation leverages CKKS's fast arithmetic and optimized matrix operations, resulting in rapid PRS computation even for thousands of SNPs.

- **Flexibility**: CKKS supports real-number computations, aligning perfectly with the continuous nature of PRS inputs and outputs.

## Features

- **Secure PRS Calculation**: Computes PRS using CKKS homomorphic encryption, enabling computations on encrypted data without decryption.

- **Fast Runtime**: Designed for efficiency, with quick execution times even for large genomic datasets (e.g., 10k SNPs and 50 individuals).

- **Scalable Security Levels**: Supports multiple security parameters (e.g., $2^{13}$, $2^{14}$, $2^{15}$, $2^{16}$) via CKKS parameter literals. Increasing the value of this parameter enhances security but reduces performance.

## How It Works

1. **Data Encryption:**
    - Genotype and phenotype data are encoded and encrypted using CKKS parameters.
    - Keys (public, secret, relinearization, and rotation) are generated for secure computation.

2. **Homomorphic PRS Calculation:**
    - The program computes the inner product of encrypted genotype and coefficient matrices using CKKS homomorphic operations.
    - Results remain encrypted during computation, preserving privacy.

3. **Decryption:**
    - The encrypted PRS results are decrypted and transposed to match the expected output format (individuals × phenotypes).

4. **Performance:**
    - The runtime completes encryption, computation, and decryption in a matter of seconds for typical datasets (e.g., 10k SNPs, 50 individuals).
    - Memory usage is monitored and printed at the end of execution.


## Prerequisites

To run this program, ensure you have the following installed:

- [Go](https://golang.org/dl/) (version 1.16 or higher recommended)
- [Lattigo v3](https://github.com/tuneinsight/lattigo) library for CKKS homomorphic encryption

## Usage
1. **Run the Program:**

    Execute the program with default settings:
    ```bash
    go run main.go
    ```
    Example output:

    ```bash
    Input Encrypt success
    Model Encrypt success
    Run model success
    Decrypt success
    Alloc = 34 MiB  TotalAlloc = 1105 MiB   HeapSys = 463 MiB       NumGC = 19
    2025/03/04 21:10:00 The program PN13QP218pq took 2.464347625s
    ```

2. **Customize Parameters:**

    Modify the `main()` function to adjust:
    - `N_PHENO`: Number of phenotypes (default: 1).
    - `n_sample`: Number of individuals (default: 50).
    - `pheno_name`: Output file name prefix (default: "phenotype0").
    - Security level: Uncomment the desired CKKS parameter (e.g., `PN14QP411pq` for 2^14).