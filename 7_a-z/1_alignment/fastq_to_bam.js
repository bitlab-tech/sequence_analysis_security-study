const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

/**
 * Executes a command and streams its output in real-time
 * @param {string} command - The command to execute
 * @param {string[]} args - Command arguments
 * @returns {Promise<void>}
 */
function executeCommand(command, args) {
  return new Promise((resolve, reject) => {
    console.log(`Executing: ${command} ${args.join(' ')}`);
    
    const process = spawn(command, args);
    
    process.stdout.on('data', (data) => {
      console.log(`${command} stdout:`, data.toString());
    });
    
    process.stderr.on('data', (data) => {
      console.log(`${command} stderr:`, data.toString());
    });
    
    process.on('close', (code) => {
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`Command failed with code ${code}`));
      }
    });
    
    process.on('error', (err) => {
      reject(err);
    });
  });
}

/**
 * Converts FASTQ files to BAM using BWA alignment
 * @param {Object} options - Configuration options
 * @param {string} options.fastq1 - Path to first FASTQ file (R1)
 * @param {string} options.fastq2 - Path to second FASTQ file (R2) for paired-end reads
 * @param {string} options.reference - Path to reference genome file
 * @param {string} options.output - Path to output BAM file
 * @param {string} options.threads - Number of threads to use (default: 4)
 * @returns {Promise<string>} Path to the output BAM file
 */
async function fastqToBam({
  fastq1,
  fastq2,
  reference,
  output,
  threads = 5
}) {
  try {
    // Validate input files
    if (!fs.existsSync(fastq1)) {
      throw new Error(`FASTQ file 1 not found: ${fastq1}`);
    }
    if (fastq2 && !fs.existsSync(fastq2)) {
      throw new Error(`FASTQ file 2 not found: ${fastq2}`);
    }
    if (!fs.existsSync(reference)) {
      throw new Error(`Reference genome file not found: ${reference}`);
    }

    // Create output directory if it doesn't exist
    const outputDir = path.dirname(output);
    if (!fs.existsSync(outputDir)) {
      fs.mkdirSync(outputDir, { recursive: true });
    }

    // Index reference genome if not already indexed
    const refIndex = `${reference}.bwt`;
    if (!fs.existsSync(refIndex)) {
      console.log('Indexing reference genome...');
      try {
        await executeCommand('bwa', ['index', reference]);
      } catch (error) {
        console.error('Error during BWA indexing:', error.message);
        throw error;
      }
    }

    // Run BWA alignment
    console.log('Running BWA alignment...');
    const samFile = output.replace('.bam', '.sam');
      
    try {
      const bwaArgs = ['mem', '-t', threads.toString(), reference, fastq1];
      if (fastq2) {
        bwaArgs.push(fastq2);
      }
      
      // Create a write stream for the SAM file
      const samStream = fs.createWriteStream(samFile);
      
      const bwaProcess = spawn('bwa', bwaArgs);
      
      bwaProcess.stdout.on('data', (data) => {
        console.log('BWA alignment stdout:', data.toString());
        samStream.write(data);
      });
      
      bwaProcess.stderr.on('data', (data) => {
        console.log('BWA alignment stderr:', data.toString());
      });
      
      await new Promise((resolve, reject) => {
        bwaProcess.on('close', (code) => {
          samStream.end();
          if (code === 0) {
              resolve();
          } else {
              reject(new Error(`BWA alignment failed with code ${code}`));
          }
        });
        
        bwaProcess.on('error', (err) => {
          samStream.end();
          reject(err);
        });
      });
    } catch (error) {
      console.error('Error during BWA alignment:', error.message);
      throw error;
    }

    // Convert SAM to BAM
    console.log('Converting SAM to BAM...');
    const outputUnsorted = output.replace('.bam', '-unsorted.bam');
    try {
      await executeCommand('samtools', [
        'view',
        '-@', threads.toString(),
        '-bS',
        '-o', outputUnsorted,
        samFile
      ]);
    } catch (error) {
      console.error('Error during SAM to BAM conversion:', error.message);
      throw error;
    }

    // Sort BAM file
    console.log('Sorting BAM file...');
    try {
      await executeCommand('samtools', [
        'sort',
        '-@', threads.toString(),
        '-o', output, 
        outputUnsorted
      ]);
    } catch (error) {
      console.error('Error during BAM sorting:', error.message);
      throw error;
    }

    // Index BAM file
    console.log('Indexing BAM file...');
    try {
      await executeCommand('samtools', ['index', output]);
    } catch (error) {
      console.error('Error during BAM indexing:', error.message);
      throw error;
    }

    // Clean up temporary SAM file
    fs.unlinkSync(samFile);

    console.log('Conversion completed successfully!');
    return output;
  } catch (error) {
    console.error('Error during FASTQ to BAM conversion:', error);
    throw error;
  }
}

// Example usage
async function main() {
  try {
    const result = await fastqToBam({
      fastq1: 'path/to/sample_R1.fastq.gz',
      fastq2: 'path/to/sample_R2.fastq.gz',
      reference: 'path/to/reference.fasta',
      output: 'path/to/output.bam',
      threads: 5
    });
    console.log('Output BAM file:', result);
  } catch (error) {
    console.error('Error:', error);
  }
}

main();

module.exports = fastqToBam; 