mod kmer_type;
use kmer_type::{
    KmerType,
    ToUInt,
    binary_encode
};
use tfhe::prelude::*;
use tfhe::{
    Config,
    ConfigBuilder,
    generate_keys,
    set_server_key,
    ClientKey,
    FheUint16,
    FheUint16Id,
    FheUint,
    FheBool
};
use rayon::prelude::*;
use std::time::Instant;

fn equality_test(query_kmer: &FheUint<FheUint16Id>, subject_kmer: u16) -> FheBool {
    query_kmer.eq(subject_kmer)
}

fn snp_compare_single<'a>(
    query_kmers: &'a Vec<FheUint<FheUint16Id>>,
    k: usize,
    subjects: &'a Vec<&str>,
    min: u16,
    max: u16
) -> Vec<FheBool> {
    let results = query_kmers.iter().map(|query_kmer| {
        subjects
        .iter()
        .map(|subject| {
            k_mer_lazy(&subject, k)
                .map(|subject_kmer| {
                    binary_encode(subject_kmer).to_u16()
                })
                .filter(|&kmer| kmer >= min && kmer <= max)
                .map(|bin_kmer| {
                    equality_test(query_kmer, bin_kmer)
                })
                .reduce(|a, b| a | b)
                .unwrap()
        })
        .reduce(|a, b | a | b)
        .unwrap()
    }).collect();

    results
}

fn snp_compare_parallel<'a>(
    query_kmers: &'a Vec<FheUint<FheUint16Id>>,
    k: usize,
    subjects: &'a Vec<&str>,
    min: u16,
    max: u16
) -> Vec<FheBool> {
    let results = query_kmers.par_iter().map(| query_kmer | {
        subjects
            .iter()
            .map(|subject| {
                k_mer_lazy(&subject, k)
                    .map(|subject_kmer| {
                        binary_encode(subject_kmer).to_u16()
                    })
                    .filter(|&kmer| kmer >= min && kmer <= max)
                    .map(|bin_kmer| {
                        equality_test(query_kmer, bin_kmer)
                    })
                    .reduce(|a, b| a | b)
                    .unwrap()
            })
            .reduce(|a, b | a | b)
            .unwrap()
    }).collect();

    results
}

fn k_mer_lazy<'a>(seq: &'a str, k: usize) -> impl Iterator<Item = &'a str> {
    let l: usize = seq.len() as usize;
    if k == 0 || k > l {
        return k_mer_lazy(seq, l);
    }
    (0..(l - k)).map(move |i: usize| &seq[i..i + k])
}

fn encrypt_kmers(kmers: &Vec<KmerType>, client_key: &ClientKey) -> Vec<FheUint<FheUint16Id>> {
    kmers
        .par_iter()
        .map(|kmer_hash| FheUint16::encrypt(kmer_hash.to_u16(), client_key))
        .collect()
}

fn main() {
    // Example set up params
    let query: &str = "ACGTTAACT";
    let snps: Vec<&str> = [
        "ACGTTGACTA",
        "GCAATTGGAC",
        "CGGAAATTAC",
        "GAGTTAACCT",
        "GAGGATTTCT"
    ].to_vec();
    let k: usize = 5;

    //================================================================
    // Client Side
    //================================================================
    let start_time = Instant::now();
    // Step 1: user splits sequence into k-mers
    let query_kmers: Vec<&str> = k_mer_lazy(query, k).collect();
    println!("==================================================");
    println!("query sequence: {:?}", query);
    println!("query k-mers: {:?}", query_kmers);

    // Step 2: user generates homomorphic encryption keys and
    // encrypts query k-mers
    // User generates homomorphic encryption keys
    let config: Config = ConfigBuilder::default()
        .use_custom_parameters(
           tfhe::shortint::parameters::PARAM_MULTI_BIT_MESSAGE_2_CARRY_2_GROUP_3_KS_PBS
        )
        .build();
    let (client_key, server_key) = generate_keys(config);

    // User binary encodes the k-mers and encrypt
    let mut min: u16 = 0b1111111111111111;
    let mut max: u16 = 0b0;

    let query_kmer_values: Vec<KmerType> = query_kmers
        .iter()
        .map(|kmer| {
            let bin_kmer = binary_encode(kmer);
            let kmer_value = bin_kmer.to_u16();
            if kmer_value >= max {
                max = kmer_value;
            }
            if kmer_value <= min {
                min = kmer_value;
            }
            bin_kmer
        })
        .collect();
    println!("query_kmer_values: {:?}", query_kmer_values);
    println!("==================================================");

    let mut before = Instant::now();
    let enc_kmers: Vec<FheUint<FheUint16Id>> = encrypt_kmers(&query_kmer_values, &client_key);
    println!("K-mers encryption time: {:.2?}", before.elapsed());

    //================================================================
    // Server Side
    //================================================================
    // Step 3: user sends encrypted query k-mers and server key to server
    // Server takes server key and set it to perform homomorphic operations
    // Set server key in all paralell threads
    rayon::broadcast(|_| set_server_key(server_key.clone()));
    // Set server key in main thread
    set_server_key(server_key);

    // Step 4: server compares query k-mers against all snp k-mers
    // in the surrounding positions homomorphically
    before = Instant::now();
    let result_single = snp_compare_single(
        &enc_kmers,
        k,
        &snps,
        min,
        max
    );
    println!("Server execution time single thread: {:.2?}", before.elapsed());

    before = Instant::now();
    let results_parallel = snp_compare_parallel(
        &enc_kmers,
        k,
        &snps,
        min,
        max
    );
    println!("Server execution time parallel threads: {:.2?}", before.elapsed());
    println!("==================================================");

    //================================================================
    // Client Side
    //================================================================
    // Step 5: result (encrypted) is sent back to user
    // User decrypts result and prints it
    let client_key_clone = client_key.clone();
    print!("Decrypted result single thread: ");
    let dec_results: Vec<bool> = result_single.iter()
        .map(move |result: &FheBool| result.decrypt(&client_key))
        .collect();
    println!("{:?}", dec_results);

    print!("Decrypted result parallel threads: ");
    before = Instant::now();
    let dec_results: Vec<bool> = results_parallel.iter()
        .map(move |result: &FheBool| result.decrypt(&client_key_clone))
        .collect();
    println!("{:?}", dec_results);
    println!("Result decryption time: {:.2?}", before.elapsed());
    println!("==================================================");
    println!("Total execution time: {:.2?}", start_time.elapsed());
    println!("==================================================");
}
