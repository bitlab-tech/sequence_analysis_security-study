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

fn equality_test(query_kmer: &FheUint<FheUint16Id>, subject_kmer: &str) -> FheBool {
    let subject_kmer_hash: KmerType = binary_encode(subject_kmer);
    query_kmer.eq(subject_kmer_hash.to_u16())
}

fn snp_compare_single<'a>(
    query_kmers: &'a Vec<FheUint<FheUint16Id>>,
    k: usize,
    subjects: &'a Vec<&str>
) -> Vec<FheBool> {
    let results = query_kmers.iter().map(|query_kmer| {
        subjects
            .iter()
            .map(|subject| {
                k_mer_lazy(&subject, k)
                    .map(|subject_kmer| {
                        equality_test(query_kmer, subject_kmer)
                    })
                    .reduce(|a, b| a | b)
                    .unwrap()
            })
            .reduce(|a, b| a | b)
            .unwrap()
    }).collect();

    results
}

fn snp_compare_parallel<'a>(
    query_kmers: &'a Vec<FheUint<FheUint16Id>>,
    k: usize,
    subjects: &'a Vec<&str>
) -> Vec<FheBool> {
    let results = query_kmers.par_iter().map(| query_kmer | {
        subjects
            .iter()
            .map(|subject| {
                k_mer_lazy(&subject, k)
                    .map(|subject_kmer| {
                        equality_test(query_kmer, subject_kmer)
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

fn encrypt_kmer_hashes(kmer_hashes: &Vec<KmerType>, client_key: &ClientKey) -> Vec<FheUint<FheUint16Id>> {
    kmer_hashes
        .par_iter()
        .map(|kmer_hash| FheUint16::encrypt(kmer_hash.to_u16(), client_key))
        .collect()
}

use std::fmt::{self, Binary, Debug, Error, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXorAssign, Shl, Shr};

#[derive(PartialEq, Eq)]
enum KmerType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128)
}

impl Shl<usize> for KmerType {
    type Output = KmerType;

    fn shl(self, rhs: usize) -> KmerType {
        match self {
            KmerType::U8(val) => KmerType::U8(val << rhs),
            KmerType::U16(val) => KmerType::U16(val << rhs),
            KmerType::U32(val) => KmerType::U32(val << rhs),
            KmerType::U64(val) => KmerType::U64(val << rhs),
            KmerType::U128(val) => KmerType::U128(val << rhs),
        }
    }
}

impl Shr<usize> for KmerType {
    type Output = KmerType;

    fn shr(self, rhs: usize) -> KmerType {
        match self {
            KmerType::U8(val) => KmerType::U8(val >> rhs),
            KmerType::U16(val) => KmerType::U16(val >> rhs),
            KmerType::U32(val) => KmerType::U32(val >> rhs),
            KmerType::U64(val) => KmerType::U64(val >> rhs),
            KmerType::U128(val) => KmerType::U128(val >> rhs),
        }
    }
}

impl BitOr for KmerType {
    type Output = KmerType;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => KmerType::U8(a | b),
            (KmerType::U16(a), KmerType::U16(b)) => KmerType::U16(a | b),
            (KmerType::U32(a), KmerType::U32(b)) => KmerType::U32(a | b),
            (KmerType::U64(a), KmerType::U64(b)) => KmerType::U64(a | b),
            (KmerType::U128(a), KmerType::U128(b)) => KmerType::U128(a | b),
            _ => panic!("Bitwise OR is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitAnd for KmerType {
    type Output = KmerType;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => KmerType::U8(a & b),
            (KmerType::U16(a), KmerType::U16(b)) => KmerType::U16(a & b),
            (KmerType::U32(a), KmerType::U32(b)) => KmerType::U32(a & b),
            (KmerType::U64(a), KmerType::U64(b)) => KmerType::U64(a & b),
            (KmerType::U128(a), KmerType::U128(b)) => KmerType::U128(a & b),
            _ => panic!("Bitwise AND is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitOrAssign<KmerType> for KmerType {
    fn bitor_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => *a |= b,
            (KmerType::U16(a), KmerType::U16(b)) => *a |= b,
            (KmerType::U32(a), KmerType::U32(b)) => *a |= b,
            (KmerType::U64(a), KmerType::U64(b)) => *a |= b,
            (KmerType::U128(a), KmerType::U128(b)) => *a |= b,
            _ => panic!("Bitwise OR is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitAndAssign<KmerType> for KmerType {
    fn bitand_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => *a &= b,
            (KmerType::U16(a), KmerType::U16(b)) => *a &= b,
            (KmerType::U32(a), KmerType::U32(b)) => *a &= b,
            (KmerType::U64(a), KmerType::U64(b)) => *a &= b,
            (KmerType::U128(a), KmerType::U128(b)) => *a &= b,
            _ => panic!("Bitwise AND is not supported for mismatched KmerType variants"),
        }
    }
}

impl BitXorAssign for KmerType {
    fn bitxor_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (KmerType::U8(a), KmerType::U8(b)) => *a ^= b,
            (KmerType::U16(a), KmerType::U16(b)) => *a ^= b,
            (KmerType::U32(a), KmerType::U32(b)) => *a ^= b,
            (KmerType::U64(a), KmerType::U64(b)) => *a ^= b,
            (KmerType::U128(a), KmerType::U128(b)) => *a ^= b,
            _ => panic!("Bitwise XOR is not supported for mismatched KmerType variants"),
        }
    }
}

impl Debug for KmerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::U8(arg0) => f.debug_tuple("U8").field(arg0).finish(),
            Self::U16(arg0) => f.debug_tuple("U16").field(arg0).finish(),
            Self::U32(arg0) => f.debug_tuple("U32").field(arg0).finish(),
            Self::U64(arg0) => f.debug_tuple("U64").field(arg0).finish(),
            Self::U128(arg0) => f.debug_tuple("U128").field(arg0).finish(),
        }
    }
}

impl Binary for KmerType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::U8(val) => fmt::Binary::fmt(&val, f),
            Self::U16(val) => fmt::Binary::fmt(&val, f),
            Self::U32(val) => fmt::Binary::fmt(&val, f),
            Self::U64(val) => fmt::Binary::fmt(&val, f),
            Self::U128(val) => fmt::Binary::fmt(&val, f),
        }
    }
}

trait ToUInt {
    // fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    // fn to_u32(&self) -> u32;
    // fn to_u64(&self) -> u64;
    // fn to_u128(&self) -> u128;
}

impl ToUInt for KmerType {
    // fn to_u8(&self) -> u8 {
    //     match self {
    //         Self::U8(val) => *val,
    //         Self::U16(_) => panic!("U16 is not supported"),
    //         Self::U32(_) => panic!("U32 is not supported"),
    //         Self::U64(_) => panic!("U64 is not supported"),
    //         Self::U128(_) => panic!("U128 is not supported"),
    //     }
    // }

    fn to_u16(&self) -> u16 {
        match self {
            Self::U8(_) => panic!("U8 is not supported"),
            Self::U16(val) => *val,
            Self::U32(_) => panic!("U32 is not supported"),
            Self::U64(_) => panic!("U64 is not supported"),
            Self::U128(_) => panic!("U128 is not supported"),
        }
    }
}

fn init_kmer_type(len: usize, val: u8) -> KmerType {
    if len <= 4 {
        KmerType::U8(val)
    } else if len <= 8 {
        KmerType::U16(val.into())
    } else if len <= 16 {
        KmerType::U32(val.into())
    } else if len <= 32 {
        KmerType::U64(val.into())
    } else if len <= 64 {
        KmerType::U128(val.into())
    } else {
        panic!("kmer length not supported");
    }
}

fn binary_encode(kmer: &str) -> KmerType {
    let kmer_len: usize = kmer.len();

    let mut result: KmerType = init_kmer_type(kmer_len, 0b00);

    for (i, c) in kmer.chars().enumerate() {
        let val: KmerType = match c {
            'A' => init_kmer_type(kmer_len, 0b00),
            'C' => init_kmer_type(kmer_len, 0b01),
            'G' => init_kmer_type(kmer_len, 0b10),
            'T' => init_kmer_type(kmer_len, 0b11),
            _ => panic!("Invalid character in kmer"),
        };
        result |= val << (kmer_len - 1 - i) * 2;
    }
    result
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

    // User hashes the k-mers and encrypt
    let query_kmer_values: Vec<KmerType> = query_kmers
        .iter()
        .map(|kmer| binary_encode(kmer))
        .collect();
    println!("query_kmer_values: {:?}", query_kmer_values);
    println!("==================================================");

    let mut before = Instant::now();
    let enc_kmers: Vec<FheUint<FheUint16Id>> = encrypt_kmer_hashes(&query_kmer_values, &client_key);
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
    let result_single = snp_compare_single(&enc_kmers, k, &snps);
    println!("Server execution time single thread: {:.2?}", before.elapsed());

    before = Instant::now();
    let results_parallel = snp_compare_parallel(&enc_kmers, k, &snps);
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
