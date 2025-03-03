use std::{error::Error};
use std::fs::File;
use csv::StringRecord;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "prs")]
#[command(version = "0.1")]
#[command(about = "Calculate Polygenic Risk Scores", long_about = None)]
struct Args {
    #[arg(short, long)]
    genotype: String,

    #[arg(short, long)]
    phenotype: String,

    #[arg(short, long)]
    output: String,
}

fn read_file(file_path: String) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    // Open the file from the path
    let file = File::open(file_path)?;

    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    // Collect all the records into a vector.
    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result?;
        records.push(record);
    }
    
    Ok(records)
}

fn prs(genos: Vec<StringRecord>, phenos: Vec<StringRecord>) -> Vec<f32> {
    let mut results = Vec::new();
    let pheno = &phenos[0];

    genos.iter().for_each(|geno| {
        let mut prs: f32 = 0.0;

        for i in 1..geno.len() {
            let g_i =  geno[i].parse::<f32>().unwrap();
            let w_i =  pheno[i].parse::<f32>().unwrap();
            let score_i = g_i * w_i;
            prs += score_i;
        }
        results.push(prs);
    });
    results
}

fn write_file(data: Vec<f32>, file_path: String) -> Result<(), Box<dyn Error>> {
    // Open the file from the path
    let file = File::create(file_path)?;

    // Build the CSV reader and iterate over each record.
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    // Write each f32 as its own record (one per row)
    for value in data {
        wtr.write_record(&[value.to_string()])?;
    }

    // Flush the writer to ensure all data has been written
    wtr.flush()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    let genotypes = read_file(args.genotype).unwrap();
    let phenotypes = read_file(args.phenotype).unwrap();

    let results = prs(genotypes, phenotypes);

    let _ = write_file(results, args.output);
}
