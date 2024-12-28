# Improvement approaches on Homomorphic Equality Test (HET) of K-mers

## Directions of improvement

- **Accuracy Improvement**

  Improving accuracy in HET focuses on reducing false positives and false negatives in genome comparisons. Accurate results ensure meaningful biological insights, even in the presence of sequencing errors or natural genomic variations.

- **Efficiency Improvement**

  Efficiency improvements aim to optimize space and time complexity, enabling scalable comparisons for large genomic datasets while maintaining accuracy.

## Naive approach

- **Accuracy Constraints**

  While conceptually straightforward, the naive approach has inherent accuracy limitations:

  - **False Positives**: Occur when unrelated k-mers from the two sets match due to randomness or insufficient specificity of the k-mer length. This is particularly problematic in repetitive or low-complexity regions of the genome.

  - **False Negatives**: Arise when biologically relevant matches are missed due to minor variations, such as single nucleotide polymorphisms (SNPs), sequencing errors, or insertions/deletions. The exact matching criterion of the naive approach fails to account for these small but meaningful differences.

- **Efficiency Constraints**

  The naive approach for HET involves comparing two native vectors of k-mers directly. This method has a time complexity of $O(n \cdot m)$, where $n$ and $m$ are the lengths of the vectors. Although simple, this approach becomes computationally prohibitive for large-scale genomic datasets.



## Accuracy improvement

- Increase k-mer length to ensure uniqueness (typically between 30 and 35 in real world usage).
- ...

## Efficiency improvement

- Binary encoding
- Sorted comparison
