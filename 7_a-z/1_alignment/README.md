# FASTQ to BAM Conversion

This Node.js module provides functionality to convert FASTQ files to BAM format using BWA alignment with real-time progress monitoring.

## Prerequisites

Before using this module, ensure you have the following installed:

1. BWA (Burrows-Wheeler Aligner)
   ```bash
   # On macOS
   brew install bwa
   
   # On Ubuntu/Debian
   sudo apt-get install bwa
   ```

2. SAMtools
   ```bash
   # On macOS
   brew install samtools
   
   # On Ubuntu/Debian
   sudo apt-get install samtools
   ```

3. Node.js (v12 or higher)

## Installation

```bash
npm install
```

## Usage

```javascript
const fastqToBam = require('./fastq_to_bam');

async function main() {
    try {
        const result = await fastqToBam({
            fastq1: 'path/to/sample_R1.fastq.gz',    // Required: First FASTQ file
            fastq2: 'path/to/sample_R2.fastq.gz',    // Optional: Second FASTQ file for paired-end reads
            reference: 'path/to/reference.fasta',    // Required: Reference genome file
            output: 'path/to/output.bam',            // Required: Output BAM file path
            threads: 5                               // Optional: Number of threads (default: 4)
        });
        console.log('Output BAM file:', result);
    } catch (error) {
        console.error('Error:', error);
    }
}

main();
```

## Parameters

- `fastq1` (required): Path to the first FASTQ file
- `fastq2` (optional): Path to the second FASTQ file for paired-end reads
- `reference` (required): Path to the reference genome file
- `output` (required): Path where the output BAM file should be saved
- `threads` (optional): Number of threads to use for alignment (default: 4)

## Features

- Real-time progress monitoring with streaming stdout/stderr
- Automatic reference genome indexing
- Support for both single-end and paired-end reads
- Multi-threading support
- Automatic BAM indexing
- Input file validation
- Comprehensive error handling
- Progress logging for each step

## Process Steps

The conversion process includes the following steps:

1. Input validation
2. Reference genome indexing (if not already indexed)
3. BWA alignment with real-time progress monitoring
4. SAM to BAM conversion
5. BAM indexing
6. Cleanup of temporary files

## Error Handling

The function includes comprehensive error handling for:
- Missing input files
- Invalid file paths
- BWA alignment errors
- SAM to BAM conversion errors
- File system errors
- Process execution errors

Each step provides detailed error messages and logs both stdout and stderr output.

## Real-time Monitoring

The module provides real-time monitoring of:
- Command execution status
- BWA alignment progress
- SAMtools operations
- Error messages and warnings

## Notes

- The reference genome will be automatically indexed if not already indexed
- Temporary SAM files are automatically cleaned up after conversion
- The output BAM file is automatically indexed
- The function creates output directories if they don't exist 