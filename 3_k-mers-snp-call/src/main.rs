use tfhe::prelude::*;
use tfhe::{
    Config,
    ConfigBuilder,
    generate_keys,
    set_server_key,
    ClientKey,
    FheUint32,
    FheUint32Id,
    FheUint,
    FheBool
};

fn equality_test(query_kmer: &FheUint<FheUint32Id>, subject_kmer: &str) -> FheBool {
    let subject_kmer_hash: u32 = murmur32(subject_kmer.as_bytes(), 123456789u32);
    query_kmer.eq(subject_kmer_hash)
}

fn snp_compare<'a>(
    query_kmers: &'a Vec<FheUint<FheUint32Id>>,
    k: usize,
    subject: &'a str,
    results: &'a mut Vec<FheBool>
) -> () {
    for i in 0..(query_kmers.len()) {
        // let mut checks: Vec<FheBool> = Vec::new();
        for subject_kmer in k_mer_lazy(subject, k) {
            let query_kmer: &FheUint<FheUint32Id> = &query_kmers[i];
            if results.len() == i {
                results.push(equality_test(query_kmer, subject_kmer));                
            } else {
                results[i] |= equality_test(query_kmer, subject_kmer);
            }
        }
    }
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
    let snps: [&str; 5] = [
        "ACGTTGACTA",
        "GCAATTGGAC",
        "CGGAAATTAC",
        "GAGTTAACCT",
        "GAGGATTTCT"
    ];
    let k: usize = 5;
    let seed: u32 = 123456789;

    //================================================================
    // Client Side
    //================================================================
    // Step 1: user splits sequence into k-mers
    let query_kmers: Vec<&str> = k_mer_lazy(query, k).collect();
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
    set_server_key(server_key);

    // Step 4: server compares query k-mers against all snp k-mers
    // in the surrounding positions homomorphically
    let mut results: Vec<FheBool> = Vec::with_capacity(enc_kmers.len());
    for snp in snps.iter() {
        snp_compare(&enc_kmers, k, &snp, &mut results);
    }

    //================================================================
    // Client Side
    //================================================================
    // Step 5: result (encrypted) is sent back to user
    // User decrypts result and prints it
    print!("Decrypted result: ");
    let dec_results: Vec<bool> = results.iter()
        .map(move |result: &FheBool| result.decrypt(&client_key))
        .collect();
    println!("{:?}", dec_results);
}
