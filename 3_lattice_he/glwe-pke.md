# Public-Key Encryption for LWE/RLWE

## Overview
LWE (Learning With Errors) and its ring variant RLWE (Ring Learning With Errors) form the foundation for many post-quantum cryptographic schemes, including public-key encryption. The security of these schemes is based on the hardness of solving certain lattice-based problems with noise.

This document describes a **public-key encryption** scheme based on GLWE (Generalized Learning With Errors). The encryption scheme ensures confidentiality by leveraging a public key composed of encryptions of zero. A message is encrypted by adding a random combination of these encryptions of zero and the scaled message. Decryption relies on removing the structured noise and rounding to recover the original plaintext.

## Details
In practice, a public key would be a list of encryptions of zero (i.e., $M = 0$). To encrypt a message, it is sufficient to take a random combination of these encryptions of zero and add the desired message.

### Public Key Definition
- Public key is defined as:

    $PubKey = \text{GLWE}_{\mathbf{S}, \sigma}(0) \subseteq R_q^{k+1}$

    $PubKey =  (A_0, \dots, A_{k-1}, B)$

    $B = \displaystyle\sum_{i = 0}^{k - 1} A_i \cdot S_i + \Delta 0 + E = \displaystyle\sum_{i = 0}^{k - 1} A_i \cdot S_i + E$

### Encryption

1. Define a trivial cipher text we want to add:

    $C_M = (0, ..., 0, \Delta M) \in R_q^{k+1}$

2. Choose a small random mask vector $R = (r_0, \dots, r_{k-1})$ with small coefficients in $R_q$.

3. Compute the ciphertext as:

    $C =  \displaystyle\sum_{i=0}^{k-1} r_i \cdot PubKey + C_M$

    Expanding this,

    $C = \displaystyle\sum_{i=0}^{k-1} r_i \cdot (A_i, B) + (0, \dots, 0, \Delta M)$

    Which simplifies to:

    $C = \left( \displaystyle\sum_{i=0}^{k-1} r_i A_i, \displaystyle\sum_{i=0}^{k-1} r_iB + \Delta M \right)$

    Since $B = \sum_{i=0}^{k-1}A_iS_i + E$, we get:

    $C = \left( \displaystyle\sum_{i=0}^{k-1} r_i A_i, \displaystyle\sum_{i=0}^{k-1} r_i(A_iS_i + E) + \Delta M \right)$

    $C = \left( \displaystyle\sum_{i=0}^{k-1} r_i A_i, \displaystyle\sum_{i=0}^{k-1} r_i A_i S_i + \displaystyle\sum_{i=0}^{k-1} r_i E + \Delta M \right)$

### Decryption

1. Compute:

    $B_M - \displaystyle\sum_{i=0}^{k-1} A_{Mi}S_{i}$

    where $(A_{Mi}, B_M)$ are the components of $C$.

    Plugging in the values:

    $\displaystyle\sum_{i=0}^{k-1} r_i A_i  S_i + \displaystyle\sum_{i=0}^{k-1} r_iE + \Delta M - \displaystyle\sum_{i=0}^{k-1} A_{Mi}S_i$

    Since $A_{Mi} = r_iA_i$, this simplifies to:

    $\displaystyle\sum_{i=0}^{k-1} r_i A_i S_i - \displaystyle\sum_{i=0}^{k-1} r_i A_i S_i + \displaystyle\sum_{i=0}^{k-1} r_i E + \Delta M$

    $= \displaystyle\sum_{i=0}^{k-1} r_i E + \Delta M$

    Since $\sum_{i=0}^{k-1} r_i E$ is still a **small error term**, decryption proceeds by rounding:

    $M = \left\lfloor (\Delta M + \displaystyle\sum_{i=0}^{k-1} r_i E ) / \Delta \right\rceil$

    As long as $\sum_{i=0}^{k-1} r_i E$ is small enough, rounding correctly recovers $M$.

## Examples

### Example 1: Encrypting a Message
Assume we have:
- Secret key $S = (s_0, s_1, ..., s_{k-1})$
- Public key $PubKey = (A_0, ..., A_{k-1}, B)$ where $B = \sum_{i=0}^{k-1} A_i S_i + E$
- Message $M = 1$ (represented in the encrypted domain as $\Delta M$)
- Random mask vector $R = (r_0, r_1, ..., r_{k-1})$

Then the ciphertext is computed as:

$C = \sum_{i=0}^{k-1} r_i (A_i, B_i) + (0, ..., 0, \Delta M)$

### Example 2: Decrypting the Ciphertext

Given the ciphertext $C = (A_{M0}, ..., A_{Mk-1}, B_M)$, the decryptor computes:

$B_M - \sum_{i=0}^{k-1} A_{Mi} S_i$

Which simplifies to:

$\Delta M + \sum_{i=0}^{k-1} r_i E_i$

Since the error term is small, rounding yields the correct message:

$M = \lfloor (\Delta M + \sum_{i=0}^{k-1} r_i E_i) / \Delta \rceil$

Thus, the message $M$ is successfully recovered.

