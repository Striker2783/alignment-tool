use k_fold_cross_validation::{
    hash_mult_impl::{data_set::Total as MultTotal, Storage as MultStorage},
    Config,
};
use std::{env, fs, path::Path, process, time::Instant};

fn hash_mult_impl(config: &Config) {
    let time = Instant::now();
    let mut storage = MultStorage::default();

    if let Err(e) = storage.load_tax_file(config.tax()) {
        eprintln!("{e}");
    }
    if let Err(e) = storage.load_fasta_file(config.fasta()) {
        eprintln!("{e}");
    };
    let data_set = MultTotal::build(&storage, config.k_fold());

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
    let args = env::args();
    let config = match Config::build(args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    hash_mult_impl(&config);
}
