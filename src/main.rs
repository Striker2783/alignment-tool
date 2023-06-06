use k_fold_cross_validation::data_set::*;
use k_fold_cross_validation::*;
use std::time::Instant;

fn lib() {
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

fn main() {
    lib();
}
