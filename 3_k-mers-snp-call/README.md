# Homomorphic SNP Comparison Proof of Concept

This project is a **proof of concept (POC)** implementation based on research paper: ["A secure SNP panel scheme using homomorphically encrypted K-mers without SNP calling on the user side (2019)"](https://pubmed.ncbi.nlm.nih.gov/30967116/) by **Sungjoon Park, Minsu Kim, Seokjun Seo, Seungwan Hong, Kyoohyung Han, Keewoo Lee, Jung Hee Cheon and Sun Kim**.

It describes a privacy-preserving method for comparing patient genomic sequences to SNP panels held by a hospital using [homomorphic encryption](https://en.wikipedia.org/wiki/Homomorphic_encryption). The goal is to ensure that the patient's genomic data remains encrypted and confidential throughout the entire comparison process while allowing the hospital to conduct the necessary operations securely.

## Project Overview

This POC demonstrates a method where:
1. The **patient splits their genomic sequence** into k-mers (subsequences of length `k`).
2. The **k-mers are hashed and encrypted** on the client side using homomorphic encryption.
3. The **encrypted k-mers are sent to a server**, which holds the SNP panel data.
4. The **server performs homomorphic comparisons** to check for matches between the patient's k-mers and the SNP panel k-mers.
5. The **encrypted results** of the comparison are returned to the patient, who then decrypts and analyzes them.

## Key Components

- **Homomorphic Encryption Library**: This implementation uses the [tfhe-rs](https://github.com/zama-ai/tfhe-rs) library for homomorphic encryption to securely process encrypted data.
- **[MurmurHash](https://en.wikipedia.org/wiki/MurmurHash) Function**: Used to hash k-mers for uniform representation before encryption.
- **K-mer Generation**: A function to split a sequence into k-mers for comparison.
- **Homomorphic Equality Test**: A method to perform equality testing between encrypted k-mers and SNP sequences.

## How It Works

### Client-Side Workflow
1. **Split the Query Sequence**:
   - The query sequence is split into k-mers using a sliding window approach.
2. **Generate and Encrypt k-mer Hashes**:
   - Each k-mer is hashed using a modified MurmurHash function and then encrypted using homomorphic encryption.
3. **Send Encrypted Data**:
   - The encrypted k-mers and the encryption keys (server key) are sent to the server for comparison.

### Server-Side Workflow
4. **Set Server Key**:
   - The server sets the provided server key to enable homomorphic operations.
5. **Homomorphic Comparison**:
   - The server compares each encrypted query k-mer with k-mers generated from the SNP panel sequences and returns a vector of encrypted results.

### Client-Side Result Decryption
6. **Decrypt Results**:
   - The client receives the encrypted results from the server and decrypts them to analyze the outcome of the SNP comparison.

## Dependencies
- Rust version **1.81** or later.
- `tfhe-rs`: A Rust crate for fully homomorphic encryption.

## Research Context

This POC is inspired by the methods described in a research paper that aims to establish a secure communication protocol between a patient and a hospital. The protocol enables the patient to encrypt their genomic data and send it for analysis without exposing the raw sequence, while the hospital preserves the confidentiality of its SNP panel data.

## Limitations and Future Work

- **Performance**: Homomorphic encryption introduces computational overhead. Optimizations can be made to improve processing time.
- **Scalability**: This POC handles a limited number of SNPs and k-mers; further work is needed to scale up for real-world genome analysis.
- **Security**: The cryptographic methods are for demonstration and should be thoroughly reviewed for production use.