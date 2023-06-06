use k_fold_cross_validation::{
    hash_impl::{data_set::Total, Storage},
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

    println!("Took {} ms", time.elapsed().as_millis());
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

    println!("{} ms", time.elapsed().as_millis());
}

fn main() {
    hash_impl();
    static_hash_impl();
}
