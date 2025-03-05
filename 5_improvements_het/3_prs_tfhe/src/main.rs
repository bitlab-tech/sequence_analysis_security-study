use std::error::Error;
use std::fs::File;
use csv::StringRecord;
use tfhe::prelude::*;
use tfhe::{
    Config,
    ConfigBuilder,
    generate_keys,
    set_server_key,
    ClientKey,
    FheInt32,
};
use rayon::prelude::*;
use std::time::Instant;

fn read_file(file_path: &str) -> Result<Vec<StringRecord>, Box<dyn Error>> {
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

fn convert_genotypes_to_i32(vec: Vec<StringRecord>) -> Vec<Vec<i32>> {
    vec.iter().map(|genotype| {
        genotype.iter().map(|x|
            x.parse::<i32>().unwrap()
        ).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>()
}

fn convert_phenotypes_to_f32(vec: Vec<StringRecord>) -> Vec<f32> {
    vec[0].iter().map(|x| x.parse::<f32>().unwrap()).collect::<Vec<f32>>()
}

fn homomorphic_prs(
    genotypes: Vec<Vec<FheInt32>>,
    phenotypes: Vec<i32>,
    client_key: &ClientKey,
) -> Vec<FheInt32> {
    genotypes.par_iter().map(|individual| {
        let mut sum = FheInt32::encrypt(0, client_key);
        for (g_i, p_i) in individual.iter().zip(phenotypes.iter()) {
            let product = g_i * *p_i; // Homomorphic multiplication
            sum = &sum + &product;   // Homomorphic addition
        }
        sum
    }).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let config: Config = ConfigBuilder::default()
        .use_custom_parameters(
           tfhe::shortint::parameters::PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS
        )
        .build();
    let (client_key, server_key) = generate_keys(config);
    rayon::broadcast(|_| set_server_key(server_key.clone()));
    set_server_key(server_key);

    let genotypes = read_file("data/genotype_10kSNP_50individual.csv").unwrap();
    let phenotypes = read_file("data/beta_10kSNP_phenotype0.csv").unwrap();

    // Input data
    let a: Vec<Vec<i32>> = convert_genotypes_to_i32(genotypes);
    let b: Vec<f32> = convert_phenotypes_to_f32(phenotypes);
    let scale = 1e6;

    // Scale B to integers
    let b_int: Vec<i32> = b.iter().map(|x| (x * scale as f32) as i32).collect();

    // Encrypt arrays
    let enc_a: Vec<Vec<FheInt32>> = a.iter().map(|vec|
        vec.iter().map(|x|
            FheInt32::encrypt(*x, &client_key)
        ).collect()
    ).collect();
    // let enc_b: Vec<FheInt32> = b_int.iter().map(|x| FheInt32::encrypt(*x, &client_key)).collect();

    let start_time = Instant::now();

    // Compute element-wise multiplication and sum
    let results = homomorphic_prs(enc_a, b_int, &client_key);

    println!("Server execution time: {:.2?}", start_time.elapsed());

    // Decrypt and scale down
    let int_results: Vec<i32> = results.iter().map(|result| 
        result.decrypt(&client_key)
    ).collect::<Vec<i32>>();

    let float_results = int_results.iter().map(|result| 
        *result as f32 / scale as f32
    ).collect::<Vec<f32>>();

    println!("Results: {:?}", float_results);
    Ok(())
}
