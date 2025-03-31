# A survey on the proof of the Worst-Case Hardness of GLWE

## Overview

The Generalized Learning With Errors (GLWE) problem is a foundational framework in lattice-based cryptography, unifying several important problems such as Learning With Errors (LWE), Ring Learning With Errors (RLWE), and Module Learning With Errors (MLWE). GLWE is parameterized by a ring $R$, a modulus $q$, and a rank $d$, allowing it to capture a wide range of cryptographic constructions. Its versatility and efficiency make it a cornerstone of many post-quantum cryptographic schemes, such as those standardized by NIST (e.g., CRYSTALS-Kyber and CRYSTALS-Dilithium). Establishing the worst-case hardness of GLWE is thus critical for ensuring the security of these schemes against both classical and quantum adversaries.

This survey proves that solving GLWE is at least as hard as solving worst-case lattice problems, specifically the Shortest Vector Problem (SVP), the Shortest Independent Vectors Problem (SIVP), and Ideal-SIVP. We achieve this through a chain of reductions, starting from well-known hard lattice problems and culminating in GLWE. The structure of the survey is as follows:

- **Section 1** establishes the hardness of SVP and SIVP, showing that approximating these problems within certain factors is NP-hard.
- **Section 2** reduces SIVP to the Bounded Distance Decoding (BDD) problem, a decoding problem on lattices that serves as an intermediate step.
- **Section 3** reduces BDD to LWE, which corresponds to GLWE with $R = \mathbb{Z}$ and $d = n$.
- **Section 4** reduces BDD to RLWE, which corresponds to GLWE with $d = 1$.
- **Section 5** reduces BDD to MLWE, which corresponds to GLWE with arbitrary $d$, typically $1 \leq d \leq n$.
- The **Conclusion** defines GLWE, notes that it subsumes LWE, RLWE, and MLWE, and concludes that BDD reduces to GLWE in all cases, implying that GLWE inherits the hardness of worst-case lattice problems.

The result confirms that GLWE-based cryptographic schemes are secure under standard assumptions about the hardness of lattice problems, providing a strong theoretical foundation for their use in post-quantum cryptography.

## Table of Contents
* [1. Hardness of SVP and SIVP](#1-hardness-of-svp-and-sivp)
  + [Definition 1: Lattice](#definition-1-lattice)
  + [Definition 2: Shortest Vector Problem (SVP)](#definition-2-shortest-vector-problem-svp)
  + [Definition 3: Shortest Independent Vectors Problem (SIVP)](#definition-3-shortest-independent-vectors-problem-sivp)
  + [Theorem 1: Hardness of SVP and SIVP](#theorem-1-hardness-of-svp-and-sivp)
  + [Lemma 1.1: Reduction from SVP to SIVP](#lemma-11-reduction-from-svp-to-sivp)
* [2. Reduction from SIVP to BDD](#2-reduction-from-sivp-to-bdd)
  + [Definition 4: Bounded Distance Decoding (BDD)](#definition-4-bounded-distance-decoding-bdd)
  + [Theorem 2: Hardness of BDD](#theorem-2-hardness-of-bdd)
  + [Lemma 2.1: SIVP Reduces to BDD](#lemma-21-sivp-reduces-to-bdd)
* [3. Reduction from BDD to LWE](#3-reduction-from-bdd-to-lwe)
  + [Definition 5: Learning with Errors (LWE)](#definition-5-learning-with-errors-lwe)
  + [Theorem 3: Hardness of LWE](#theorem-3-hardness-of-lwe)
  + [Lemma 3.1: Reduction from BDD to LWE](#lemma-31-reduction-from-bdd-to-lwe)
* [4. Reduction from BDD to RLWE](#4-reduction-from-bdd-to-rlwe)
  + [Definition 6: Ring Learning with Errors (RLWE)](#definition-6-ring-learning-with-errors-rlwe)
  + [Theorem 4: Hardness of RLWE](#theorem-4-hardness-of-rlwe)
  + [Lemma 4.1: Reduction from BDD to RLWE](#lemma-41-reduction-from-bdd-to-rlwe)
* [5. Reduction from BDD to MLWE](#5-reduction-from-bdd-to-mlwe)
  + [Definition 7: Module Learning with Errors (MLWE)](#definition-7-module-learning-with-errors-mlwe)
  + [Theorem 5: Hardness of MLWE](#theorem-5-hardness-of-mlwe)
  + [Lemma 5.1: Reduction from BDD to MLWE](#lemma-51-reduction-from-bdd-to-mlwe)
* [Conclusion](#conclusion)
  + [Definition 8: Generalized Learning with Errors (GLWE)](#definition-8-generalized-learning-with-errors-glwe)
  + [Final Theorem: Hardness of GLWE](#final-theorem-hardness-of-glwe)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>


## 1. Hardness of SVP and SIVP

### Definition 1: Lattice
A full-rank lattice $\Lambda \subset \mathbb{R}^n$ is a discrete subgroup of $\mathbb{R}^n$.

### Definition 2: Shortest Vector Problem (SVP)
Find the shortest nonzero vector:

$$
\lambda_1(\Lambda) = \min_{v \in \Lambda \setminus \{0\}} \| v \|.
$$

### Definition 3: Shortest Independent Vectors Problem (SIVP)
Find $n$ linearly independent vectors $v_1, \dots, v_n$ such that:

$$
\max_{i} \| v_i \| \leq \gamma \lambda_n(\Lambda),
$$

where $\lambda_n(\Lambda)$ is the $n$-th successive minimum, and $\gamma \geq 1$ is an approximation factor.

### Theorem 1: Hardness of SVP and SIVP

**Citations:**
- [Ajtai, M. (1998). "The Shortest Vector Problem in L2 is NP-hard for Randomized Reductions." *Proceedings of the 30th Annual ACM Symposium on Theory of Computing (STOC), 10-19.*](https://dl.acm.org/doi/pdf/10.1145/276698.276705)
- [Haviv, I., & Regev, O. (2007). "Tensor-based Hardness of the Shortest Vector Problem to within Almost Polynomial Factors." *Proceedings of the 39th Annual ACM Symposium on Theory of Computing (STOC), 469-477.*](https://dl.acm.org/doi/pdf/10.1145/1250790.1250859)
- [Micciancio, D. (2008). "Inapproximability of the Shortest Vector Problem: Toward a Deterministic Reduction." *Theory of Computing, 4(1), 149-161.*](https://theoryofcomputing.org/articles/v004a008/v004a008.pdf)

Approximating SVP within a factor of $2^{(\log n)^{1 - \epsilon}}$ (i.e., $O(2^{\text{polylog}(n)})$ ) is **NP-hard** under randomized reductions. Approximating SIVP within a factor of $n^{1/\log \log n}$ is also **NP-hard**. These hardness results establish the foundation for reductions to other problems such as BDD, LWE, RLWE, MLWE, and GLWE.

### Lemma 1.1: Reduction from SVP to SIVP
If SIVP is efficiently solvable within a factor $\gamma$, then SVP can also be efficiently solved within a factor $\gamma \cdot \text{poly}(n)$.

**References:**
- [Goldreich, O., Micciancio, D., Safra, S., & Seifert, J.-P. (1999). "On the Limits of Non-Approximability of Lattice Problems." *Journal of Computer and System Sciences, 58(1), 191-203.*](https://dl.acm.org/doi/pdf/10.1145/276698.276704)
- [Peikert, C. (2016). "A Decade of Lattice Cryptography." *Foundations and Trends in Theoretical Computer Science, 10(4), 283-424.*](https://eprint.iacr.org/2015/939.pdf)

---

## 2. Reduction from SIVP to BDD

### Definition 4: Bounded Distance Decoding (BDD)
Given a lattice $\Lambda$ and a target $v$ satisfying:

$$
\text{dist}(v, \Lambda) \leq d < \frac{\lambda_1(\Lambda)}{2},
$$

find the closest lattice point $w \in \Lambda$.

### Theorem 2: Hardness of BDD

**Citation:** [Lyubashevsky, V., & Micciancio, D. (2009). "On Bounded Distance Decoding and Related Problems." *CRYPTO 2009, LNCS 5677, 577-594.*](https://www.iacr.org/archive/crypto2009/56770568/56770568.pdf)

BDD is at least as hard as **SIVP**.

### Lemma 2.1: SIVP Reduces to BDD
If an efficient BDD solver exists, then an efficient SIVP solver exists.

**Reference:**
- [Micciancio, D., & Regev, O. (2004). "Worst-case to average-case reductions based on Gaussian measures." *SIAM Journal on Computing, 37(1), 267-302.*](https://cseweb.ucsd.edu/~daniele/papers/Gaussian.pdf)

---

## 3. Reduction from BDD to LWE

### Definition 5: Learning with Errors (LWE)
Given samples $(a_i, b_i)$ where:

$$
b_i = \langle a_i, s \rangle + e_i \mod q,
$$

distinguish from uniform $(a_i, u_i)$, where:

- $a_i \sim U(\mathbb{Z}_q^n)$,
- $s \in \mathbb{Z}_q^n$,
- $e_i \sim \chi$(small noise).

### Theorem 3: Hardness of LWE

**Citation:** [Regev, O. (2005). "On lattices, learning with errors, random linear codes, and cryptography." *Journal of the ACM (JACM), 54(6), Article 30.*](https://arxiv.org/pdf/2401.03703)

BDD reduces to LWE, meaning LWE is at least as hard as BDD.

### Lemma 3.1: Reduction from BDD to LWE
If an efficient LWE oracle exists, it can be used to solve BDD.

**References:**
- [Regev, O. (2005). "On lattices, learning with errors, random linear codes, and cryptography." *Journal of the ACM (JACM), 54(6), Article 30.*](https://arxiv.org/pdf/2401.03703)
- [Brakerski, Z., Langlois, A., Peikert, C., Regev, O., & Stehlé, D. (2013). "Classical Hardness of Learning with Errors." *Proceedings of the 45th Annual ACM Symposium on Theory of Computing (STOC), 575-584.*](https://arxiv.org/pdf/1306.0281)

---

## 4. Reduction from BDD to RLWE

### Definition 6: Ring Learning with Errors (RLWE)
Given samples $(a_i, b_i)$ where:

$$
b_i = a_i \cdot s + e_i \mod q,
$$

distinguish from uniform $(a_i, u_i)$, where:
- $a_i \sim U(R_q)$,
- $s \in R_q$,
- $e_i \sim \chi$(small noise),
- $R$ is a polynomial ring (e.g., $\mathbb{Z}[X]/(X^n + 1)$ ).

### Theorem 4: Hardness of RLWE

**Citation:** [Lyubashevsky, V., Peikert, C., & Regev, O. (2010). "On Ideal Lattices and Learning with Errors Over Rings." *Advances in Cryptology – EUROCRYPT 2010, LNCS 6110, 1-23.*](https://eprint.iacr.org/2012/230.pdf)

BDD reduces to RLWE, meaning RLWE is at least as hard as BDD.

### Lemma 4.1: Reduction from BDD to RLWE
If an RLWE oracle exists, it can be used to solve BDD.

**Reference:**
- [Lyubashevsky, V., Peikert, C., & Regev, O. (2010). "On Ideal Lattices and Learning with Errors Over Rings." *Advances in Cryptology – EUROCRYPT 2010, LNCS 6110, 1-23.*](https://eprint.iacr.org/2012/230.pdf)

---

## 5. Reduction from BDD to MLWE

### Definition 7: Module Learning with Errors (MLWE)
Given samples $(\mathbf{a}_i, b_i)$ where:

$$
b_i = \langle \mathbf{a}_i, \mathbf{s} \rangle + e_i \mod q,
$$

distinguish from uniform $(\mathbf{a}_i, u_i)$, where:
- $\mathbf{a}_i \in R_q^d$,
- $\mathbf{s} \in R_q^d$,
- $e_i \in R$,
- $R$ is a polynomial ring (e.g., $\mathbb{Z}[X]/(X^n + 1)$),
- $d$ is the rank of the module (typically $1 \leq d \leq n$).

### Theorem 5: Hardness of MLWE

**Citation:** [Jeudy, B., Boudgoust, K., Roux-Langlois, A., & Wen, W. (2023). "On the Hardness of Module Learning With Errors with Short Distributions." *ePrint Archive, 2022/472.*](https://eprint.iacr.org/2022/472.pdf)

BDD reduces to MLWE, meaning MLWE is at least as hard as BDD.

### Lemma 5.1: Reduction from BDD to MLWE
If an MLWE oracle exists, it can be used to solve BDD.

**References:**
- [Langlois, A., & Stehlé, D. (2015). "Worst-Case to Average-Case Reductions for Module Lattices." *Designs, Codes and Cryptography, 75(1), 53-77.*](https://eprint.iacr.org/2012/090.pdf)
- [Jeudy, B., Boudgoust, K., Roux-Langlois, A., & Wen, W. (2023). "On the Hardness of Module Learning With Errors with Short Distributions." *ePrint Archive, 2022/472.*](https://eprint.iacr.org/2022/472.pdf)

---

## Conclusion

### Definition 8: Generalized Learning with Errors (GLWE)
GLWE is a unifying framework that generalizes LWE, RLWE, and MLWE. Given samples $(\mathbf{a}_i, b_i)$ where:

$$
b_i = \langle \mathbf{a}_i, \mathbf{s} \rangle + e_i \mod q,
$$

distinguish from uniform$(\mathbf{a}_i, u_i)$, where:
- $\mathbf{a}_i \in R_q^d$,
- $\mathbf{s} \in R_q^d$,
- $e_i \in R$,
- $R$ is a ring (e.g., $\mathbb{Z}$ or $\mathbb{Z}[X]/(X^n + 1)$),
- $d$ is the rank of the module.

GLWE subsumes:
- **LWE**: When $R = \mathbb{Z}$, $d = n$.
- **RLWE**: When $d = 1$.
- **MLWE**: When $d$ is arbitrary (typically $1 \leq d \leq n$).

**Reference:** [Peikert, C. (2016). "A Decade of Lattice Cryptography." *Foundations and Trends in Theoretical Computer Science, 10(4), 283-424.*](https://eprint.iacr.org/2015/939.pdf)

### Final Theorem: Hardness of GLWE
Since LWE, RLWE, and MLWE are special cases of GLWE, and BDD reduces to LWE (Theorem 3), RLWE (Theorem 4), and MLWE (Theorem 5), it follows that BDD reduces to GLWE in all cases. Therefore, for appropriate parameters $q$, noise distribution $\chi$, and ring structure $R$:

$$
\text{Solving GLWE is at least as hard as worst-case (Gap)SVP, SIVP, and Ideal-SIVP.}
$$

**References:**
- [Regev, O. (2005). "On lattices, learning with errors, random linear codes, and cryptography." *Journal of the ACM (JACM), 54(6), Article 30.*](https://arxiv.org/pdf/2401.03703)
- [Lyubashevsky, V., Peikert, C., & Regev, O. (2010). "On Ideal Lattices and Learning with Errors Over Rings." *Advances in Cryptology – EUROCRYPT 2010, LNCS 6110, 1-23.*](https://eprint.iacr.org/2012/230.pdf)
- [Jeudy, B., Boudgoust, K., Roux-Langlois, A., & Wen, W. (2023). "On the Hardness of Module Learning With Errors with Short Distributions." *ePrint Archive, 2022/472.*](https://eprint.iacr.org/2022/472.pdf)
- [Peikert, C. (2016). "A Decade of Lattice Cryptography." *Foundations and Trends in Theoretical Computer Science, 10(4), 283-424.*](https://eprint.iacr.org/2015/939.pdf)