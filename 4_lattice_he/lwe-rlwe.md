# On LWE and RLWE

## Security Assumption

Given:

- A randomly chosen matrix $A \in \mathbb{Z}_q^{n \times m}$
- A vector $\vec{s} \in \mathbb{Z}_q^n$, chosen uniformly at random
- A vector $\vec{e} \in \mathbb{Z}_q^m$, chosen at random from a set of "short" vectors.

Let:

$b = A^{T} \vec{s} + \vec{e}$

The problem of recovering $\vec{s}$ from:

$\bar{A} = (A, b^{T}) \in \mathbb{Z}_q^{(n + 1) \times m}$

is known as the ["Learning With Errors" (LWE)](https://en.wikipedia.org/wiki/Learning_with_errors) problem. This is an average-case variant of the [bounded distance decoding (BDD)](https://en.wikipedia.org/wiki/Lattice_problem#Bounded_distance_decoding) problem for a random $q$-ary lattice:

$\Lambda_q(A) = \lbrace A^T x \ | \ x \in \mathbb{Z}_q^n \rbrace + q \mathbb{Z}^m,$

with target vector $b$.

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

#### Explicitly:

- For **LWE**:
  - $k = n \in \mathbb{Z}$
  - $N = 1$

- For **RLWE**:
  - $k = 1$
  - $N$ is a power of 2.

---

### Notes

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

### References

- [Fully Homomorphic Encryption](https://cseweb.ucsd.edu/classes/fa17/cse206A-a/LecFHE.pdf)
- [TFHE Deep Dive - Part I - Ciphertext types](https://www.zama.ai/post/tfhe-deep-dive-part-1)
- [003 TFHE Deep Dive w/ Ilaria Chillotti](https://www.youtube.com/watch?v=npoHSR6-oRw)

---

### Example

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