use k_fold_cross_validation::{
    hash_impl::{data_set::Total, Storage},
    hash_mult_impl::{data_set::Total as MultTotal, Storage as MultStorage},
    static_impl::{data_set::Total as StaticTotal, Storage as StaticStorage},
};
use std::{fs, time::Instant};

fn hash_impl() {
    let time = Instant::now();
    let mut storage = Storage::default();

    if let Err(e) = storage.load_tax_file("TAX.txt") {
        eprintln!("{e}");
    }
    if let Err(e) = storage.load_fasta_file("FASTA.txt") {
        eprintln!("{e}");
    };

    let _data_set = Total::build(&storage, 5);

    println!("Hash: {} ms", time.elapsed().as_millis());
}
fn hash_mult_impl() {
    let time = Instant::now();
    let mut storage = MultStorage::default();

    if let Err(e) = storage.load_tax_file("TAX.txt") {
        eprintln!("{e}");
    }
    if let Err(e) = storage.load_fasta_file("FASTA.txt") {
        eprintln!("{e}");
    };

    let _data_set = MultTotal::build(&storage, 5);

    println!("Hash Concurrency: {} ms", time.elapsed().as_millis());
}

fn static_hash_impl() {
    let time = Instant::now();
    let mut storage = StaticStorage::default();

    let tax = fs::read_to_string("TAX.txt").expect("");
    let fasta = fs::read_to_string("FASTA.txt").expect("");

    if let Err(e) = storage.load_tax(&tax) {
        eprintln!("{e}");
    };
    if let Err(e) = storage.load_fasta(&fasta) {
        eprintln!("{e}");
    };

    let _ = StaticTotal::build(&storage, 5);

    println!("Static hash: {} ms", time.elapsed().as_millis());
}

fn main() {
    hash_impl();
    static_hash_impl();
    hash_mult_impl();
}
