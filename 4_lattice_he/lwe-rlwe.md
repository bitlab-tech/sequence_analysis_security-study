# On LWE and RLWE

## Overview

This document provides a structured exploration of the [Learning With Errors (LWE)](https://en.wikipedia.org/wiki/Learning_with_errors) and [Ring Learning With Errors (RLWE)](https://en.wikipedia.org/wiki/Ring_learning_with_errors) problems, foundational concepts in [lattice-based cryptography](https://en.wikipedia.org/wiki/Lattice-based_cryptography). 

**LWE** and **RLWE** are the building blocks of advanced cryptographic systems like [Fully Homomorphic Encryption (FHE)](https://en.wikipedia.org/wiki/Homomorphic_encryption), enabling secure computation on encrypted data. This document connects theory to practice with references to seminal works and implementations.

Explore these concepts to understand the interplay between mathematical structures and cryptographic operations essential for [post-quantum security](https://en.wikipedia.org/wiki/Post-quantum_cryptography).

## Table of Contents

- [Security Assumption](#security-assumption)
  * [The LWE Problem](#the-lwe-problem)
  * [Why is LWE Hard?](#why-is-lwe-hard)
- [LWE and RLWE (GLWE)](#lwe-and-rlwe-glwe)
  * [Parameters](#parameters)
  * [Encryption](#encryption)
  * [Decryption](#decryption)
  * [Ciphertext Addition](#ciphertext-addition)
  * [Constant Multiplication](#constant-multiplication)
- [References](#references)
- [Example](#example)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>


## Security Assumption

- Given:
  1. A random matrix $A \in \mathbb{Z}_q^{n \times m}$.  
  2. A secret vector $\vec{s} \in \mathbb{Z}_q^n$, chosen uniformly at random.  
  3. An error vector $\vec{e} \in \mathbb{Z}_q^m$, where entries are small ("short").  

- Computation:

  - Define the vector:  
  
    $b = A^T \vec{s} + \vec{e}$

- Combine $A$ and $b^T$ into an augmented matrix:  

  $\bar{A} = (A, b^T) \in \mathbb{Z}_q^{(n+1) \times m}$


### The LWE Problem

Recovering the secret vector $\vec{s}$ from the augmented matrix $\bar{A}$ is computationally hard.

This problem is called the **Learning With Errors (LWE)** problem.  

It is closely related to finding the [closest vector](https://en.wikipedia.org/wiki/Lattice_problem#Closest_vector_problem_(CVP)) in a random $q$-ary [lattice](https://en.wikipedia.org/wiki/Lattice_(group)):

$\Lambda_q(A) = \lbrace A^T x \ | \ x \in \mathbb{Z}_q^n \rbrace + q \mathbb{Z}^m$

given a noisy target vector $b$.


### Why is LWE Hard?
The noise $\vec{e}$ makes solving **LWE** much harder than a simple linear system, turning it into a [lattice-based decoding](https://en.wikipedia.org/wiki/Lattice-based_cryptography) problem. This hardness forms the basis of many cryptographic schemes.


## LWE and RLWE (GLWE)

### Parameters

Given:

- $R = \mathbb{Z}[X]/(X^N + 1)$, a [polynomial ring](https://en.wikipedia.org/wiki/Polynomial_ring) modulo $X^N + 1$

- $R_q = (\mathbb{Z}/q\mathbb{Z})[X]/(X^N + 1)$, the same ring over the modular integers

- Modular reductions are centered around zero. As an example, when reducing modulo $64$, we use the [congruence](https://en.wikipedia.org/wiki/Congruence_relation) classes $\lbrace-32, -31,..., 0,..., 30, 31\rbrace$

- $\chi_\sigma$: a [Gaussian probability distribution](https://en.wikipedia.org/wiki/Normal_distribution) with mean $\mu = 0$ and standard deviation $\sigma$

- Capital letters ($A, B, S, \dots$) denote polynomials

- Lowercase letters ($a, b, s, \dots$) denote modular integers

And:

- $\vec{S} \in R^k$

- $\vec{A} \in R_q^k$

- $E \in R_q$, with coefficients sampled from $\chi_\sigma$

- Plaintext modulus $p$ and ciphertext modulus $q$ ($p, q \in \mathbb{Z}^+$), where $p \leq q$, both being powers of 2

- $\Delta = q / p$, the scaling factor.

**Explicitly:**

- For **LWE**:
  - $k = n \in \mathbb{Z}$
  - $N = 1$

- For **RLWE**:
  - $k = 1$
  - $N$ is a power of 2.

**Notes:**

- $GLWE$ generalizes both LWE and RLWE.
- The noise $E$ must remain small enough to ensure decryption correctness, typically constrained by $\Delta / 2$.

---

### Encryption

1. Compute:

    $B = \sum_{i = 0}^{k - 1} A_i \cdot S_i + \Delta M + E$

2. Define:

    $GLWE_{\vec{S}, \sigma}(\Delta M) = (A, B) \subseteq \mathcal{R}_q^{k+1}$

3. The ciphertext is:
    
    $C = (A_0, \ldots, A_{k-1}, B)$

---

### Decryption

1. Compute the linear combination:

    $B - \sum_{i = 0}^{k - 1} A_{i}S_{i}$

    This simplifies to:
  
    $\Delta M + E \in R_q$

2. Recover the plaintext $M$ by rounding:

    $M = \lfloor (\Delta M + E) / \Delta \rceil$

---

### Ciphertext Addition

1. Consider:

    $C' = GLWE_{\vec S, \sigma}(\Delta M')$
    $= (A_0^{'}, ..., A'_{k-1}, B') \subseteq \mathcal{R}_{q}^{k+1}$

2. Perform addition:

    $C^{(+)} = C + C'$

    $= (A_{0} + A'_{0}, ...,A_{k-1} + A'_{k-1}, B + B')$

    $= GLWE_{\vec S, \sigma'}(\Delta (M + M')) \subseteq \mathcal{R}_{q}^{k+1}$

3. New error standard deviation $\sigma'$ (error growth):

    $\sigma' = \sqrt{\sigma^2 + \sigma^2} = \sqrt{2\sigma^2}$

    $= \sqrt{2} \cdot \sigma$

---

### Constant Multiplication

1. Consider $\Lambda$ a small constant polynomial or a scalar in $\mathbb{Z}$:

    - $\Lambda = \sum_{i=0}^{N-1} \Lambda_{i}X^{i} \in R$ 
    
      or:

    - $\Lambda \in \mathbb{Z}$

2. Perform multiplication:

    $C^{(\cdot)} = \Lambda \cdot C$

    $= (\Lambda \cdot A_0, ..., \Lambda \cdot A_{k-1}, \Lambda \cdot B)$

      $= GLWE_{\vec S, \sigma''}(\Delta (\Lambda \cdot M)) \subseteq \mathcal{R}_{q}^{k+1}$

3. Error growth:

    $\sigma'' = |\Lambda| \cdot \sigma$

## References

- [Fully Homomorphic Encryption](https://cseweb.ucsd.edu/classes/fa17/cse206A-a/LecFHE.pdf)
- [TFHE Deep Dive - Part I - Ciphertext types](https://www.zama.ai/post/tfhe-deep-dive-part-1)
- [TFHE Deep Dive - Part II - Encodings and linear leveled operations](https://www.zama.ai/post/tfhe-deep-dive-part-2)
- [003 TFHE Deep Dive w/ Ilaria Chillotti](https://www.youtube.com/watch?v=npoHSR6-oRw)


## Example

- Set up:
  - $q=64$, $p=4$
  - $\Delta = q/p = 16$
  - $N = 4$
  - $k = 2$

  - $\vec{S}$ in uniform binary distribution $= (S_{0}, S{1})$
    
    $= (0 + 0X + 1X^{2} + 1X^{3}, 1 + 0X^{2} + 1X^{3}) \in R^{2}$

  - $\vec{A} = (A_{0}, A_{1})$ with coefficients in $\lbrace -32, -31, ..., 0, ..., 30, 31 \rbrace \in R_{q}^{k}$
  
    $= (17 + 5X - 30X^{2} + 7X^{3}, 23 + 7X + 27X^{2} - 4X^{3})$
  
  - $E= 1 + 0X + 1X^{3}  \in R_{q} $

  **Message $M$ is:**

  - $M \in R_{p} = -2 + 0 \cdot X + 1 \cdot X^{2} - 1 \cdot X^{3}$


- Encryption:
  - $C = (A_{0}, A_{1}, B) \in R_{q}$
  - $B = AS + \Delta M + E$
    
    $= A_{0}S_{0} + A_{1}S_{1} + \Delta M + E$
    
      - $A_{0}S_{0} = (17 + 5X - 30X^{2} + 7X^{3})(X^{2} + X^{3})$

        $= 17X^{2} + 17X^{3} + 5X^{3} + 5X^{4} - 30X^{4} - 30X^{5} + 7X^{5} + 7X^{6}$
        
        $= 17X^{2} + X^{3}(17 + 5) + X^{4}(5 -30) + X^{5}(-30 + 7) + 7X^{6}$

        $= 17X^{2} + 22X^{3} - 25X^{4} - 23X^{5} + 7X^{6}$

        Using $X^{N} = X^{4} \equiv -1 \mod (X^{4} + 1)$,
        
        Replace $X^{4}$ with $-1$:

        $= 17X^{2} + 22X^{3} - (25 \cdot -1)  - 23X(-1) + 7X^{2}(-1)$

        $= 17X^{2} + 22X^{3} + 25 + 23X - 7X^{2}$

        $= 25 + 23X + 10X^{2} + 22X^{3} \in R_{q}$

      - $A_{1}S_{1} = (23 + 7X + 27X^{2} - 4X^{3})(1 + X^{3})$

        $= 23 + 23X^3 + 7X + 7X^{4} + 27X^{2} + 27 X^{5} - 4X^{3} - 4X^{6}$

        $= 23 + X^{3}(23 - 4) + 7X + 7X^{4} + 27X^{2} + 27X^{5} - 4X^{6}$

        $= 23 + 19X^{3} + 7X + 7X^{4} + 27X^{2} + 27X^{5} - 4X^{6}$
        
        Replace $X^{4}$ with $-1$:

        $= 23 + 19X^{3} + 7X + 7(-1) + 27X^{2} + 27X(-1) - 4X^2(-1)$

        $= 23 + 19X^{3} + 7X - 7 + 27X^{2} - 27X + 4X^{2}$

        $= 16 - 20X + 31X^{2} + 19X^{3}$
      
      - $\Delta M = 16M = 16(-2 + X^{2} - X^{3}) = -32 + 16X^{2} - 16X^{3}$

    So:

    $B = A_{0}S_{0} + A_{1}S_{1} + \Delta M + E$

    $= (25 + 23X + 10X^{2} + 22X^{3}) + (16 - 20X + 31X^{2} + 19X^{3}) + (-32 + 16X^{2} - 16X^{3}) + (1 + X^{3})$

    $= 10 + 3X + 57X^{2} + 26 X^{3}$

  - **Cipher text** $(A_{0}, A_{1}, B)$ is:
  
    $(17 + 5X - 30X^{2} + 7X^{3}, 23 + 7X + 27X^{2} - 4X^{3}, 10 + 3X + 57X^{2} + 26 X^{3})$

- Decryption:
  - $M = \lfloor (\Delta M + E)/\Delta \rceil$
  
    - $\Delta M + E = B - \sum_{i = 0}^{k - 1} A_{i}S{i}$
    
      $=(10 + 3X + 57X^{2} + 26X^{3}) - ((25 + 23X + 10X^{2} + 22X^{3}) + (16 - 20X + 31X^{2} + 19X^{3}))$

      $= (10 + 3X + 57X^{2} + 26X^{3}) - (41 + 3X + 41X^{2} + 41X^{3})$

      $= 10 + 3X + 57X^{2} + 26X^{3} - 41 - 3X - 41X^{2} - 41X^{3}$

      $= -31 + 16X^{2} - 15X^{3}$

  - $\lfloor (\Delta M + E) / \Delta \rceil = \lfloor \frac{-31 + 16X^{2} - 15X^{3}}{16} \rceil$

      $= \lfloor -\frac{31}{16} + \frac{16}{16}X^{2} - \frac{15}{16}X^{3} \rceil$

      $= -2 + X^2 - X^3 = M$