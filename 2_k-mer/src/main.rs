// Naive implementation (https://en.wikipedia.org/wiki/K-mer)
fn k_mer(seq: &str, k: usize) -> Vec<&str> {
    let l: usize = seq.len();

    if k == 0 || k > l {
        return std::iter::empty::<&str>().collect();
    }

    let mut arr: Vec<&str> = Vec::with_capacity(l - k + 1);

    // iterate over the number of k-mers in seq, 
    // storing the nth k-mer in the output array
    for i in 0..(l - k) {
        arr.push(&seq[i..i + k]);
    }

    arr
}

// Lazy Evaluation: By returning an iterator, this function will 
// only calculate each k-mer when it’s needed, which is useful for 
// processing or filtering large sequences without storing intermediate results.
fn k_mer_lazy<'a>(seq: &'a str, k: usize) -> impl Iterator<Item = &'a str> {
    let l: usize = seq.len();
    if k == 0 || k > l {
        return k_mer_lazy(seq, l);
    }
    (0..(l - k)).map(move |i: usize| &seq[i..i + k])
}

fn main() {
    let seq: &str = "ACGTACGTACGT";
    let k: usize = 5;

    // Naive run
    let kmers: Vec<&str> = k_mer(&seq, k);
    println!("K-mers: {:?}", kmers);

    // Collect if you need all results at once
    let kmers: Vec<&str> = k_mer_lazy(seq, k).collect();
    println!("K-mers lazy: {:?}", kmers);

    // Or use the iterator directly for processing each k-mer one by one
    for kmer in k_mer_lazy(seq, k) {
        println!("K-mers lazy evaluation: {}", kmer);
    }
}
