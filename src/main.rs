use k_fold_cross_validation::hash_mult_impl::{
    data_set::Total as MultTotal, Storage as MultStorage,
};
use std::{fs, path::Path, time::Instant};

fn hash_mult_impl() {
    let time = Instant::now();
    let mut storage = MultStorage::default();

    if let Err(e) = storage.load_tax_file("TAX.txt") {
        eprintln!("{e}");
    }
    if let Err(e) = storage.load_fasta_file("FASTA.txt") {
        eprintln!("{e}");
    };
    let data_set = MultTotal::build(&storage, 5);

    let path = Path::new("files");
    if let Err(e) = fs::create_dir(path) {
        eprintln!("{e}");
    };
    if let Err(e) = data_set.write_data(path) {
        eprintln!("{e}");
    };

    println!("Hash Concurrency: {} ms", time.elapsed().as_millis());
}

fn main() {
    hash_mult_impl();
}
