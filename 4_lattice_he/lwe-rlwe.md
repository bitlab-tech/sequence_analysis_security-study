# On LWE and RLWE

## Overview

This document provides a structured exploration of the [Learning With Errors (LWE)](https://en.wikipedia.org/wiki/Learning_with_errors) and [Ring Learning With Errors (RLWE)](https://en.wikipedia.org/wiki/Ring_learning_with_errors) problems, foundational concepts in [lattice-based cryptography](https://en.wikipedia.org/wiki/Lattice-based_cryptography). 

**LWE** and **RLWE** are the building blocks of advanced cryptographic systems like [Fully Homomorphic Encryption (FHE)](https://en.wikipedia.org/wiki/Homomorphic_encryption#Fully_homomorphic_encryption), enabling secure computation on encrypted data. This document connects theory to practice with references to seminal works and implementations.

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
The noise $\vec{e}$ makes solving **LWE** much harder than a simple linear system, turning it into a [lattice-based decoding problem](https://en.wikipedia.org/wiki/Lattice_problem). This hardness forms the basis of many cryptographic schemes.


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

    $C' = \text{GLWE}_{\mathbf{S}, \sigma}(\Delta M') \subseteq R_q^{k+1}$
    
    $C' = (A_0', \dots, A_{k-1}', B')$

2. Perform addition:

    $C^{(+)} = C + C'$

    $C^{(+)} = (A_0 + A_0', \dots, A_{k-1} + A_{k-1}', B + B')$

    Equivalently:

    $C^{(+)} = \text{GLWE}_{\mathbf{S}, \sigma'}(\Delta(M + M')) \subseteq R_q^{k+1}$

3. The error standard deviation grows during addition as follows:

    $\sigma' = \sqrt{\sigma^2 + \sigma^2} = \sqrt{2\sigma^2} = \sqrt{2} \cdot \sigma$

---

### Constant Multiplication

1. Let $\Lambda$ be a small constant polynomial or a scalar in $\mathbb{Z}$:
   - $\Lambda = \sum_{i=0}^{N-1} \Lambda_i X^i \in R$

     or:
   - $\Lambda \in \mathbb{Z}$

2. Perform multiplication:

   $C^{(\cdot)} = \Lambda \cdot C$

   $C^{(\cdot)} = (\Lambda \cdot A_0, \dots, \Lambda \cdot A_{k-1}, \Lambda \cdot B)$

   Equivalently:

   $C^{(\cdot)} = \text{GLWE}_{\mathbf{S}, \sigma''}(\Delta (\Lambda \cdot M)) \subseteq \mathcal{R}_q^{k+1}$

3. Error growth:

   $\sigma'' = |\Lambda| \cdot \sigma$


## References

- [Fully Homomorphic Encryption](https://cseweb.ucsd.edu/classes/fa17/cse206A-a/LecFHE.pdf)
- [TFHE Deep Dive - Part I - Ciphertext types](https://www.zama.ai/post/tfhe-deep-dive-part-1)
- [TFHE Deep Dive - Part II - Encodings and linear leveled operations](https://www.zama.ai/post/tfhe-deep-dive-part-2)
- [003 TFHE Deep Dive w/ Ilaria Chillotti](https://www.youtube.com/watch?v=npoHSR6-oRw)


## Example

- Set up:
  - $q=\color{red}{64}$, $p=\color{red}{4}$
  - $\Delta = q/p = \color{red}{16}$
  - $N = \color{red}{4}$
  - $k = \color{red}{2}$

  - $\vec{S}$ in uniform binary distribution $= (S_{0}, S{1})$

    $= \color{red}{(0 + 0X + 1X^{2} + 1X^{3}, 1 + 0X^{2} + 1X^{3}) \in R^{2}}$

  - $\vec{A} = (A_{0}, A_{1})$ with coefficients in $\lbrace -32, -31, ..., 0, ..., 30, 31 \rbrace \in R_{q}^{k}$

    $= \color{red}{(17 + 5X - 30X^{2} + 7X^{3}, 23 + 7X + 27X^{2} - 4X^{3})}$

  - $E= \color{red}{1 + 0X + 1X^{3}  \in R_{q}}$

  **Message $M$ is:**

  - $M \in R_{p} = \color{red}{-2 + 0 \cdot X + 1 \cdot X^{2} - 1 \cdot X^{3}}$

  **Message $M'$ (for homomorphic addition) is:**

  - $M' \in R_{p} = \color{red}{0 + 0 \cdot X + 1 \cdot X^{2} - 2 \cdot X^{3}}$

  ---
- Encryption:
  - $\color{red}{C = (A_{0}, A_{1}, B) \in R_{q}}$
  - $\color{red}{B = AS + \Delta M + E} = A_{0}S_{0} + A_{1}S_{1} + \Delta M + E$

      - $\color{blue}{A_{0}S_{0}}$ $= (17 + 5X - 30X^{2} + 7X^{3})(X^{2} + X^{3})$

        $= 17X^{2} + 17X^{3} + 5X^{3} + 5X^{4} - 30X^{4} - 30X^{5} + 7X^{5} + 7X^{6}$

        $= 17X^{2} + X^{3}(17 + 5) + X^{4}(5 -30) + X^{5}(-30 + 7) + 7X^{6}$

        $= 17X^{2} + 22X^{3} - 25X^{4} - 23X^{5} + 7X^{6}$

        Using $X^{N} = X^{4} \equiv -1 \mod (X^{4} + 1)$,

        Replace $X^{4}$ with $-1$:

        $= 17X^{2} + 22X^{3} - (25 \cdot -1)  - 23X(-1) + 7X^{2}(-1)$

        $= 17X^{2} + 22X^{3} + 25 + 23X - 7X^{2}$

        $= 25 + 23X + 10X^{2} + 22X^{3} \in R_{q}$

      - $\color{blue}{A_{1}S_{1}}$ $= (23 + 7X + 27X^{2} - 4X^{3})(1 + X^{3})$

        $= 23 + 23X^3 + 7X + 7X^{4} + 27X^{2} + 27 X^{5} - 4X^{3} - 4X^{6}$

        $= 23 + X^{3}(23 - 4) + 7X + 7X^{4} + 27X^{2} + 27X^{5} - 4X^{6}$

        $= 23 + 19X^{3} + 7X + 7X^{4} + 27X^{2} + 27X^{5} - 4X^{6}$

        Replace $X^{4}$ with $-1$:

        $= 23 + 19X^{3} + 7X + 7(-1) + 27X^{2} + 27X(-1) - 4X^2(-1)$

        $= 23 + 19X^{3} + 7X - 7 + 27X^{2} - 27X + 4X^{2}$

        $= 16 - 20X + 31X^{2} + 19X^{3}$

      - $\color{blue}{\Delta M}$ $= 16M = 16(-2 + X^{2} - X^{3}) = -32 + 16X^{2} - 16X^{3}$

    So:

    $\color{red}{B = A_{0}S_{0} + A_{1}S_{1} + \Delta M + E}$

    $= (25 + 23X + 10X^{2} + 22X^{3}) + (16 - 20X + 31X^{2} + 19X^{3}) + (-32 + 16X^{2} - 16X^{3}) + (1 + X^{3})$

    $= \color{green}{10 + 3X + 57X^{2} + 26 X^{3}}$

  - **Cipher text** $\color{red}{(A_{0}, A_{1}, B)}$ is:

    $\color{green}{\boxed{(17 + 5X - 30X^{2} + 7X^{3}, 23 + 7X + 27X^{2} - 4X^{3}, 10 + 3X + 57X^{2} + 26 X^{3})}}$

  ---

- Decryption:
  - $\color{red}{M = \lfloor (\Delta M + E)/\Delta \rceil}$

    - $\color{blue}{\Delta M + E = B - \sum_{i = 0}^{k - 1} A_{i}S{i}}$

      $=(10 + 3X + 57X^{2} + 26X^{3}) - ((25 + 23X + 10X^{2} + 22X^{3}) + (16 - 20X + 31X^{2} + 19X^{3}))$

      $= (10 + 3X + 57X^{2} + 26X^{3}) - (41 + 3X + 41X^{2} + 41X^{3})$

      $= 10 + 3X + 57X^{2} + 26X^{3} - 41 - 3X - 41X^{2} - 41X^{3}$

      $= \color{green}{-31 + 16X^{2} - 15X^{3}}$

  - $\color{red}{\lfloor (\Delta M + E) / \Delta \rceil = \lfloor \frac{-31 + 16X^{2} - 15X^{3}}{16} \rceil}$

      $= \lfloor -\frac{31}{16} + \frac{16}{16}X^{2} - \frac{15}{16}X^{3} \rceil$

      $= \color{green}{\boxed{-2 + X^2 - X^3}}$ $= \color{green}{\boxed{M}}$

  ---

- Ciphertext Addition:
  - Recall that $\color{red}{M' = X^2 - 2X^3 \in R_q}$
  - $\color{red}{M^{(+)} = M + M'}$

    $= -2 + X^2 - X^3 + X^2 - 2X^3$

    $= \color{green}{-2 + 2X^2 -3X^3}$

  - Let's choose:

    $\vec{A'} = (A_0',A_1')= \color{red}{(9 + 20X + X^2 - X^3, -6 - 4X + 13X^2 -3X^3) \in R_q^2}$

    $E' = \color{red}{5 + X + 2X^2}$

  ---

  - Encrypt $M'$:
    - $\color{red}{C' = (A_0', A_1', B') \in R_q}$
    - $\color{blue}{B' = A_0'S_0 + A_1'S_1 + \Delta M' + E'}$
      - $\color{blue}{A_0'S_0}$ $= (9 + 20X + X^2 - X^3)(X^2 + X^3)$

        $= 9X^2 + 9X^3 + 20X^3 + 20X^4 + X^4 + X^5 - X^5 - X^6$

        $= 9X^2 + (9 + 20)X^3 + (20 + 1)X^4 + (1 - 1)X^5 - X^6$

        $= 9X^2 + 29X^3 + 21X^4 - X^6$

        Replace $X^{4}$ with $-1$:

        $= 9X^2 + 29X^3 + 21(-1) - X^2(-1)$

        $= 9X^2 + 29X^3 - 21 + X^2$

        $= -21 + 10X^2 + 29X^3$

      - $\color{blue}{A_1'S_1}$ $= (-6 -4X + 13X^2 - 3X^3)(1 + X^3)$

        $= -6 -6X^3 -4X - 4X^4 + 13X^2 + 13X^5 - 3X^3 - 3X^6$

        $= -6 -4X +13X^2 -9X^3 -4X^4 +13X^5 -3X^6$

        Replace $X^{4}$ with $-1$:

        $= -6 -4X + 13X^2 -9X^3 -4(-1) + 13X(-1) -3X^2(-1)$

        $= -6 -4X + 13X^2 -9X^3 +4 -13X +3X^2$

        $= -2 -17X +16X^2 -9X^3$

      - $\color{blue}{\Delta M'}$ $= 16(X^2 - 2X^3)= 16X^2 - 32X^3$

      So:

      $\color{blue}{B' = A_0'S_0 + A_1'S_1 + \Delta M' + E'}$

      $= (-21 + 10X^2 + 29X^3) + (-2 -17X +16X^2 -9X^3) + (16X^2 - 32X^3) + (5 + X + 2X^2)$

      $= -21 + 10X^2 + 29X^3 -2 -17X +16X^2 -9X^3 + 16X^2 - 32X^3 + 5 + X + 2X^2$

      $= (-21 - 2 + 5) + (-17 + 1)X + (10 + 16 + 16 + 2)X^2 + (29 - 9 + 32)X^3$

      $= -18 - 16X + 44X^2 - 12X^3$

    - **Ciphertext** $\color{red}{(A_0', A_1', B')}$ is:

      $\color{green}{\boxed{(9 + 20X + X^2 - X^3, -6 -4X + 13X^2 - 3X^3, -18 - 16X + 44X^2 - 12X^3)}}$

  ---

  - Perform addition:

    $\color{red}{C^{(+)} = C + C' = (A_0 + A_0', A_1 + A_1', B + B')}$

      - $\color{blue}{A_0 + A_0'}$ $= 17 + 5X - 30X^{2} + 7X^{3} + 9 + 20X + X^2 - X^3$

        $= 26 + 25X - 29X^2 + 6X^3$

      - $\color{blue}{A_1 + A_1'}$ $= 23 + 7X + 27X^{2} - 4X^{3} + (-6 - 4X + 13X^2 -3X^3)$

        $= 17 + 3X + 40X^2 - 7X^3$

      - $\color{blue}{B + B'}$ $= 10 + 3X + 57X^{2} + 26 X^{3} + (-18 - 16X + 44X^2 - 12X^3)$

        $= -8 - 13X + 101X^2 + 14X^3$

    $\color{red}{C^{(+)} = (A_0^{(+)}, A_1^{(+)}, B^{(+)})}$

    $=\color{green}{\boxed{(26 + 25X -29X^2 + 6X^3, 17 + 3X + 40X^2 - 7X^3, -8 - 13X + 101X^2 + 14X^3)}}$

  ---

  - Decrypt the ciphertext addition result:

    $\color{red}{M^{(+)} = \lfloor (\Delta M^{(+)} + E^{(+)})/\Delta \rceil}$

      - $\color{blue}{\Delta M^{(+)} + E^{(+)} = B^{(+)} - \sum_{i=0}^{k-1}A_{i}^{(+)} \cdot S_i}$

        $= B^{(+)} - (A_0^{(+)}S_0 + A_1^{(+)}S_1)$

        - $\color{blue}{A_0^{(+)}S_0}$ $= (26 + 25X -29X^2 + 6X^3)(X^2 + X^3)$

          $=26X^2 + 26X^3 + 25X^3 + 25X^4 - 29X^4 - 29X^5 + 6X^5 + 6X^6$

          $= 26X^2 + 51X^3 - 4X^4 - 23X^5 + 6X^6$

          Replace $X^4$ with $-1$:

          $= 26X^2 +  51X^3 - 4(-1) - 23X(-1) + 6X^2(-1)$

          $= 26X^2 + 51X^3 + 4 + 23X -6X^2$

          $= 4 + 23X + 20X^2 + 51X^3$

        - $\color{blue}{A_1^{(+)}S_1}$ $= (17 + 3X + 40X^2 - 7X^3)(1 + X^3)$

          $=17 + 17X^3 + 3X + 3X^4 + 40X^2 + 40X^5 -7X^3 - 7X^6$

          $= 17 + 3X + 40X^2 + 10X^3 + 3X^4 + 40X^5 - 7X^6$

          Replace $X^4$ with $-1$:

          $= 17 + 3X + 40X^2 + 10X^3 + 3(-1) + 40X(-1) - 7X^2(-1)$

          $= 17 + 3X + 40X^2 + 10X^3 - 3 - 40X + 7X^2$

          $= 14 - 37X + 47X^2 + 10X^3$

        - $\color{blue}{\Delta M^{(+)} + E^{(+)} = B^{(+)} - (A_0^{(+)}S_0 + A_1^{(+)}S_1)}$

          $= -8 - 13X + 101X^2 + 14X^3 - (4 + 23X + 20X^2 + 51X^3 + 14 - 37X + 47X^2 + 10X^3)$

          $= -8 - 13X + 101X^2 + 14X^3 - 4 - 23X - 20X^2 - 51X^3 - 14 + 37X - 47X^2 - 10X^3$

          $= (-8 - 4 - 14) + (-13 - 23 + 37)X + (101 - 20 - 47)X^2 + (14 - 51 - 10)X^3$

          $= -26 + X + 34X^2 - 47X^3$

      - $\color{red}{M^{(+)} = \lfloor (\Delta M^{(+)} + E^{(+)})/\Delta \rceil}$

          $= \lfloor \frac{-26 + X + 34X^2 - 47X^3}{16} \rceil$

          $= \lfloor -\frac{26}{16} + \frac{X}{16} + \frac{34X^2}{16} - \frac{47X^3}{16} \rceil$

          $= \color{green}{\boxed{-2 + 2X^2 - 3X^3}}$ $= \color{green}{\boxed{M + M' = M^{(+)}}}$
  
          Decryption worked fine because the **error coefficients** were all smaller (in absolute value) than $\Delta/2 = 8$.

          The new error was equal to:

          $E^{(+)} = E + E' = 1 + 1X^3 + 5 + X + 2X^2$

          $= 6 + X+ 2X^2 + X^3 $
  
  ---
- Constant multiplication:

  - Recall:

    $\color{red}{M = -2 + 1X^{2} - 1X^{3}}$

  - Choose $\Lambda$ as a small constant polynomial:
  
    $\color{red}{\Lambda = 2 + X^2 - 2X^3} \in R$

  - The multiplication is equal to:

    $\color{red}{M^{(\cdot)} \in R_p = \Lambda M}$
    
    $= (-2 + 1X^{2} - 1X^{3})(2 + X^2 - 2X^3)$

    $= -4 + 2X^3 + X^4 - 3X^5 + 2X^6$

    Replace $X^4$ with $-1$:

    $= -4 + 2X^3 + (-1) - 3X(-1) + 2X^2(-1)$

    $= -4 + 2X^3 - 1 + 3X - 2X^2$

    $= -5 + 3X - 2X^2 +2X^3$

  - Recall ciphertext $C$:

    $\color{red}{C = (A_0, A_1, B)}$
    
    $= \color{green}{\boxed{(17 + 5X - 30X^{2} + 7X^{3}, 23 + 7X + 27X^{2} - 4X^{3}, 10 + 3X + 57X^{2} + 26 X^{3})}}$

  ---

  - $\Lambda$ multiplication on ciphertext:

    $\color{red}{C^{(\cdot)} = (A_0^{(\cdot)}, A_1^{(\cdot)}, B^{(\cdot)})}$

      - $\color{blue}{A_0^{(\cdot)} = \Lambda \cdot A_0}$ 
      
        $= (17 + 5X - 30X^{2} + 7X^{3})(2 + X^2 - 2X^3)$
        
        $= 34 + 17X^2 - 34X^3 + 10X + 5X^3 - 10X^4 - 60X^2 - 30X^4 + 60X^5 + 14X^3 + 7X^5 - 14X^6$

        $= 34 + 10X - 43X^2 - 15X^3 - 40X^4 + 67X^5 - 14X^6$

        Replace $X^4$ with $-1$:

        $= 34 + 10X - 43X^2 - 15X^3 - 40(-1) + 67X(-1) - 14X^2(-1)$

        $= 34 + 10X - 43X^2 - 15X^3 + 40 - 67X + 14X^2$

        $= 74 - 57X - 29X^2 - 15X^3$
      
      - $\color{blue}{A_1^{(\cdot)} = \Lambda \cdot A_1}$

        $= (23 + 7X + 27X^{2} - 4X^{3})(2 + X^2 - 2X^3)$

        $= 46 + 23X^2 - 46X^3 + 14X + 7X^3 - 14X^4 + 54X^2 + 27X^4 - 54X^5 - 8X^3 - 4X^5 + 8X^6$

        $= 46 + 14X + 77X^2 - 47X^3 + 13X^4 - 58X^5 + 8X^6$

        Replace $X^4$ with $-1$:

        $= 46 + 14X + 77X^2 - 47X^3 + 13(-1) - 58X(-1) + 8X^2(-1)$

        $= 46 + 14X + 77X^2 - 47X^3 -13 + 58X - 8X^2$

        $= 33 + 72X + 69X^2 - 47X^3$

      - $\color{blue}{B^{(\cdot)} = \Lambda \cdot B}$

        $= (10 + 3X + 57X^{2} + 26 X^{3})(2 + X^2 - 2X^3)$

        $= 20 + 10X^2 - 20X^3 +6X + 3X^3 - 6X^4 + 114X^2 + 57X^4 - 114X^5 + 52X^3 + 26X^5 - 52X^6$

        $= 20 + 6X + 124X^2 + 35X^3 + 51X^4 - 88X^5 - 52X^6$

        Replace $X^4$ with $-1$:

        $= 20 + 6X + 124X^2 + 35X^3 + 51(-1) - 88X(-1) - 52X^2(-1)$

        $= 20 + 6X + 124X^2 + 35X^3 - 51 + 88X + 52X^2$

        $= -31 + 94X + 176X^2 + 35X^3$

    $\color{red}{C^{(\cdot)} = (A_0^{(\cdot)}, A_1^{(\cdot)}, B^{(\cdot)})}$
    
    $= \color{green}{\boxed{(74 - 57X - 29X^2 - 15X^3, 33 + 72X + 69X^2 - 47X^3, -31 + 94X + 176X^2 + 35X^3)}}$

  ---

  - Decrypt the constant multiplication result:

    $\color{red}{M^{(\cdot)} = \lfloor (M^{(\cdot)} + E^{(\cdot)}) / \Delta \rceil}$

    $\color{red}{M^{(\cdot)} + E^{(\cdot)} = B^{(\cdot)} - \sum_{i=0}^{k-1}A_i^{(\cdot)}S_i} \in R_p $

    $\color{red}{= B^{(\cdot)} - (A_0^{(\cdot)}S_0 + A_1^{(\cdot)}S_1)}$

      - $\color{blue}{A_0^{(\cdot)}S_0}$
        $= (74 - 57X - 29X^2 - 15X^3)(X^2 + X^3)$

        $= 74X^2 + 74X^3 - 57X^3 - 57X^4 - 29X^4 - 29X^5 - 15X^5 - 15X^6$

        $= 74X^2 + 17X^3 - 86X^4 - 44X^5 - 15X^6$

        Replace $X^4$ with $-1$:

        $= 74X^2 + 17X^3 - 86(-1) - 44X(-1) - 15X^2(-1)$

        $= 74X^2 + 17X^3 + 86 + 44X + 15X^2$

        $= 86 + 44X + 89X^2 + 17X^3$

      - $\color{blue}{A_1^{(\cdot)}S_1}$
        $= (33 + 72X + 69X^2 - 47X^3)(1 + X^3)$

        $= 33 + 33X^3 + 72X + 72X^4 + 69X^2 + 69X^5 - 47X^3 - 47X^6$

        $= 33 + 72X + 69X^2 - 14X^3 + 72X^4 + 69X^5 - 47X^6$

        Replace $X^4$ with $-1$:

        $= 33 + 72X + 69X^2 - 14X^3 + 72(-1) + 69X(-1) - 47X^2(-1)$

        $= 33 + 72X + 69X^2 - 14X^3 - 72 - 69X + 47X^2$

        $= -39 + 3X + 116X^2 - 14X^3$

      - $\color{blue}{\Delta M^{(\cdot)} + E^{(\cdot)}}$
        $= \color{blue}{B^{(\cdot)} - (A_0^{(\cdot)}S_0 + A_1^{(\cdot)}S_1)}$

        $= -31 + 94X + 176X^2 + 35X^3 - (86 + 44X + 89X^2 + 17X^3 + (-39 + 3X + 116X^2 - 14X^3))$

        $= -31 + 94X + 176X^2 + 35X^3 - (86 + 44X + 89X^2 + 17X^3 - 39 + 3X + 116X^2 - 14X^3)$

        $= -31 + 94X + 176X^2 + 35X^3 - (47 + 47X + 205X^2 + 3X^3)$

        $= -31 + 94X + 176X^2 + 35X^3 - 47 - 47X - 205X^2 - 3X^3$

        $= -78 + 47X - 29X^2 + 32X^3$

    $\color{red}{M^{(\cdot)} = \lfloor (M^{(\cdot)} + E^{(\cdot)}) / \Delta \rceil}$

      $= \lfloor \frac{-78 + 47X - 29X^2 + 32X^3}{16} \rceil$

      $= \lfloor \frac{-78}{16} + \frac{47X}{16} - \frac{29X^2}{16} + \frac{32X^3}{16} \rceil$

      $= \color{green}{\boxed{-5 + 3X - 2X^2 + 2X^3}}$
      $= \color{green}{\boxed{M^{(\cdot)} = \Lambda M}}$