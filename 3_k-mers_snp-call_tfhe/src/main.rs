use tfhe::prelude::*;
use tfhe::{
    Config,
    ConfigBuilder,
    generate_keys,
    set_server_key,
    ServerKey,
    ClientKey,
    FheUint32,
    FheUint32Id,
    FheUint,
    FheBool
};
use rayon::prelude::*;
use std::time::Instant;

fn equality_test(query_kmer: &FheUint<FheUint32Id>, subject_kmer: &str, seed: u32) -> FheBool {
    let subject_kmer_hash: u32 = murmur32(subject_kmer.as_bytes(), seed);
    query_kmer.eq(subject_kmer_hash)
}

fn snp_compare_single<'a>(
    query_kmers: &'a Vec<FheUint<FheUint32Id>>,
    k: usize,
    subjects: &'a Vec<&str>,
    seed: u32
) -> Vec<FheBool> {
    let results = query_kmers.iter().map(| query_kmer | {
        subjects
            .iter()
            .map(|subject| {
                k_mer_lazy(&subject, k)
                    .map(|subject_kmer| {
                        equality_test(query_kmer, subject_kmer, seed)
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
    query_kmers: &'a Vec<FheUint<FheUint32Id>>,
    k: usize,
    subjects: &'a Vec<&str>,
    seed: u32,
    server_key: ServerKey
) -> Vec<FheBool> {
    let results = query_kmers.par_iter().map(| query_kmer | {
        // Set server key for the parallel threads
        set_server_key(server_key.clone());
        // Compare k-mers
        subjects
            .iter()
            .map(|subject| {
                k_mer_lazy(&subject, k)
                    .map(|subject_kmer| {
                        equality_test(query_kmer, subject_kmer, seed)
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

fn encrypt_kmer_hashes(kmer_hashes: &Vec<u32>, client_key: &ClientKey) -> Vec<FheUint<FheUint32Id>> {
    kmer_hashes
        .iter()
        .map(|kmer_hash| FheUint32::try_encrypt(kmer_hash.clone(), client_key).unwrap())
        .collect()
}

fn murmur32(key: &[u8], seed: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;

    let mut hash: u32 = seed ^ (key.len() as u32);
    let mut i = 0;
    // first step: take care of data in groups of 4 bytes
    while i + 4 <= key.len() {
        let mut k: u32 = u32::from_le_bytes([
            key[i],
            key[i + 1],
            key[i + 2],
            key[i + 3],
        ]);

        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);

        hash ^= k;
        hash = hash.rotate_left(R2);
        hash = hash.wrapping_mul(M).wrapping_add(N);

        i += 4;
    }

    // second step: take care of the remaining 1 to 3 bytes, if any
    if i < key.len() {
        let mut k: u32 = 0;
        let mut j = 0;

        while i < key.len() {
            k ^= u32::from(key[i]) << j;
            i += 1;
            j += 8;
        }

        k = k.wrapping_mul(C1);
        k = k.rotate_left(R1);
        k = k.wrapping_mul(C2);

        hash ^= k;
    }

    // as a last step, a final scramble
    hash ^= key.len() as u32;
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash >> 16;

    hash
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
    let seed: u32 = 123456789;

    //================================================================
    // Client Side
    //================================================================
    // Step 1: user splits sequence into k-mers
    let query_kmers: Vec<&str> = k_mer_lazy(query, k).collect();
    println!("query sequence: {:?}", query);
    println!("query k-mers: {:?}", query_kmers);

    // Step 2: user generates homomorphic encryption keys and
    // encrypts query k-mers
    // User generates homomorphic encryption keys
    let config: Config = ConfigBuilder::default().build();
    let (client_key, server_key) = generate_keys(config);
    // User hashes the k-mers and encrypt
    let query_kmer_hashes: Vec<u32> = query_kmers
        .iter()
        .map(|kmer| murmur32(kmer.as_bytes(), seed))
        .collect();
    println!("query_kmer_hashes: {:?}", query_kmer_hashes);
    let enc_kmers: Vec<FheUint<FheUint32Id>> = encrypt_kmer_hashes(&query_kmer_hashes, &client_key);

    //================================================================
    // Server Side
    //================================================================
    // Step 3: user sends encrypted query k-mers and server key to server
    // Server takes server key and set it to perform homomorphic operations
    let server_key_clone = server_key.clone();
    set_server_key(server_key);

    // Step 4: server compares query k-mers against all snp k-mers
    // in the surrounding positions homomorphically
    let mut before = Instant::now();
    let result_single = snp_compare_single(&enc_kmers, k, &snps, seed);
    println!("Time execution running with single thread: {:.2?}", before.elapsed());

    before = Instant::now();
    let results_parallel = snp_compare_parallel(&enc_kmers, k, &snps, seed, server_key_clone);
    println!("Time execution running with parallel threads: {:.2?}", before.elapsed());

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
    let dec_results: Vec<bool> = results_parallel.iter()
        .map(move |result: &FheBool| result.decrypt(&client_key_clone))
        .collect();
    println!("{:?}", dec_results);
}
