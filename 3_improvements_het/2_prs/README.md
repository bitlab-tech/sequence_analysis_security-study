# Polygenic Risk Score (PRS) Calculator

This Rust program calculates **[Polygenic Risk Scores (PRS)](https://en.wikipedia.org/wiki/Polygenic_score)**, a powerful tool in genetic research and personalized medicine. PRS quantifies an individual's genetic predisposition to a specific trait or disease by aggregating the effects of multiple genetic variants. This implementation reads genotype and phenotype data from CSV files, computes PRS for each individual, and outputs the results to a file.

## What is a Polygenic Risk Score (PRS)?
A **[Polygenic Risk Score (PRS)](https://en.wikipedia.org/wiki/Locus_(genetics))** is a numerical estimate of an individual's genetic risk for a particular trait or disease, based on the combined impact of many genetic variants (typically [single nucleotide polymorphisms, or SNPs](https://en.wikipedia.org/wiki/Single-nucleotide_polymorphism)). Unlike single-gene disorders, most complex traits (e.g., diabetes, heart disease, height) are influenced by hundreds or thousands of [genetic loci](https://en.wikipedia.org/wiki/Locus_(genetics)), each contributing a small effect. PRS sums these effects into a single score, providing a personalized risk assessment.

### How PRS is Calculated
PRS is calculated as a weighted sum of an individual's genetic variants:
- **Genotypes $(g_i)$**: Represent the number of risk alleles (e.g., 0, 1, or 2) an individual carries at each genetic locus.
- **Weights $(w_i)$**: Reflect the effect size of each variant on the trait, typically derived from genome-wide association studies (GWAS).
- **Formula**:  

  $PRS = \displaystyle\sum_{i=0}^{i=M}{g_i . w_i}$

  where:
  - $g_i$ is the genotype value.
  - $w_i$ is the corresponding weight for each variant $i$.
  - $M$ is the number of variants.

In this program:
- Genotype data provides $g_i$ values (numeric, e.g., 0.5, 1.2) for each SNP.
- Phenotype data provides $w_i$ weights (numeric, e.g., 0.1, 0.4) for each SNP.
- The PRS is computed for each individual (row) in the genotype file.

### Significance of PRS
PRS is significant because:
- **Risk Prediction**: It helps identify individuals at higher or lower genetic risk for diseases, enabling early interventions or personalized treatments.
- **Research Tool**: It advances our understanding of the genetic basis of complex traits by quantifying polygenic contributions.
- **Clinical Potential**: PRS is increasingly used in precision medicine, such as stratifying patients for screening or therapy (e.g., breast cancer risk assessment).
This program provides a simplified PRS calculation, ideal for educational purposes or small-scale research.

## Features
- Command-line interface using `clap` for argument parsing.
- Reads genotype and phenotype data from CSV files.
- Computes PRS as $\displaystyle\sum_{i=0}^{i=M}{g_i . w_i}$ for each genotype record.
- Outputs PRS results to a file, one score per line.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (install via `rustup`).
- A working Rust environment with `cargo`.

## Installation
1. Clone or download this repository:
```bash
git clone <repository-url>
cd <repository-directory>
```

2. Build the project:
```bash
cargo build --release
```

The binary will be in `target/release/prs`.

## Usage
Run the program with:

```bash
prs -g <genotype_file> -p <phenotype_file> -o <output_file>
```

Example:

```bash
prs -g data/genotype_10kSNP_50individual.csv -p data/beta_10kSNP_phenotype0.csv -o prs_10kSNP_50individual.csv
```

## Limitations
- Assumes one phenotype row.
- Requires matching column counts (excluding the first) in input files.
- Uses unwrap in prs, risking panics on invalid numbers.
- No header support.

## Future Improvements
- Robust error handling for numeric parsing.
- Header support and configurable column mapping.
- Multi-row phenotype support for complex traits.
- Input validation for consistency.

## License
Unlicensed, provided as-is for educational use.
