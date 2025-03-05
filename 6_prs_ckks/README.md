# Calculate polygenic Risk Score homomorphically with CKKS 

This Go program implements a [**Polygenic Risk Score (PRS)**](https://en.wikipedia.org/wiki/Polygenic_score) calculator using the [**CKKS homomorphic encryption scheme**](https://eprint.iacr.org/2016/421.pdf) from the [Lattigo library](https://github.com/tuneinsight/lattigo). This is based on [Homomorphic Encryption: An Application to Polygenic Risk Scores - Knight et al. (2024)](https://www.researchgate.net/publication/380961036_Homomorphic_Encryption_An_Application_to_Polygenic_Risk_Scores). By leveraging CKKS, the program performs secure and privacy-preserving computations on encrypted genomic data, ensuring that sensitive information remains protected throughout the process. Notably, the implementation has a **fast runtime**, making it efficient even when handling large datasets.

## Why Use CKKS for PRS?

- **Privacy**: Genomic data is highly sensitive, and CKKS ensures it remains encrypted during computation, protecting against unauthorized access.

- **Efficiency**: This implementation leverages CKKS's fast arithmetic and optimized matrix operations, resulting in rapid PRS computation even for thousands of SNPs.

- **Flexibility**: CKKS supports real-number computations, aligning perfectly with the continuous nature of PRS inputs and outputs.

## Formula

$$
PRS_j = \sum_{i=0}^{i=M} X_{i,j} \beta_{i}
$$

- $\beta_{i}$ : effect size for variant i
- $X_{i,j}$ : the effect allele count for sample $j$ at variant $i$
- $M$ : the number of variants


## How It Works
1. **Encoding the Data**

    - **Input Data**:
        - $X_{j}$ : A vector of genotypes (0, 1 or 2) for individual $j$ across $M$ SNPs.        
        - $\beta$ : A vector of model coefficients (effect sizes) for the same SNPs.

    - **Encoding Process**:
        - Both $X_{j}$ and $\beta$ are encoded into CKKS plaintexts, transforming them from simple vectors into polynomials in the ring $Z_{Q}[x]/(x^{N} + 1)$.
        - $N$ is typically a power of 2 (e.g. 8192, 16384), determining the maximum vector lenfth that can be encoded (up to $N/2$ real numbers dua to complex packing in CKKS).
        - The genotype values (integers: 0, 1, 2) and $\beta$ values (typically floats) are scaled and embedded into polynomial coefficients in $Z_{Q}$
        - Operations (multiplication, addition) are performed modulo $Z^{N} + 1$, ensuring the polynomial structure is preserved.

    - **Purpose**:
        - Encoding allows these vectors to be encrypted and processed homomorphically while packing multiple values into a single polynomial for efficiency (via SIMD-like parallelism).

2. **Encryption:**
    - The encoded plaintexts $X_{j}$ and $\beta$ are encrypted into ciphertexts:
        - $X_{e,j}$ : Encrypted genotype matrix for individual $j$.
        - $\beta_{e}$ : Encrypted model coefficient matrix.

    - **Dimensions**:
        - Both matrices are described as $K$ x $N/2$:
            - $N/2$ represents the number of slots (values) packed into each polynomial, leveraging CKKS's packing capability.
            - $K$ is the number of polynomials needed to encode the full vector if the SNP count $M > N/2$. For example, if $M=10,000$ and $N=8192$, $K=\lceil{10,000/4096}\rceil=3$.
        - This structure aligns with the encryption parameters (e.g., modulus $Q$, polynomial degree $N$).

3. **Computing the Inner Product**:

    1. **Element-Wise Multiplication**:
        - For each row $h$ (where $h = 1, ..., K$):
        - Take the encrypted genotype vector $X_{e,j}[h]$ and model vector $\beta_e[h]$.
        - Perform element-wise multiplication using `MulRelinNew()`:
            - $v_h = X_{e,i}[h] * \beta_e[h]$.
        - `MulRelinNew()` multiplies the ciphertexts and applies relinearization to manage ciphertext growth (a technical requirement in HE to keep ciphertexts manageable).
        - $v_h$ is a new encrypted vector representing the weighted contributions ($X_{ij} \cdot \beta_i$) for that segment of SNPs.

    2. **Accumulation**:
        - The resulting vectors $v_h$ are aggregated across all $K$ rows:
        - Initial result $r = v_1$.
        - For $h = 2, ..., K$, add $v_h$ to $r$ using homomorphic addition: $r = r + v_h$.
        - This combines the partial products into a single encrypted result vector.
    3. **Summation with `InnerSumLog()`**:
        - The `InnerSumLog()` function sums the coefficients within the polynomial $r$:
        - In CKKS, a polynomial encodes up to $N/2$ values across its coefficients.
        - `InnerSumLog()` efficiently computes the sum of these values (e.g., $\sum X_{ij} \cdot \beta_i$) by rotating and adding the polynomial terms, leveraging the ring structure.
        - The output is a single encrypted PRS value for individual $j$.

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