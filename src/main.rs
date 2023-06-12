use k_fold_cross_validation::hash_mult_impl::{config::Config, data_set::Total};
use std::{env, process, time::Instant};

fn k_fold(config: &Config) {
    let time = Instant::now();

    if let Err(e) = Total::run(config) {
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
    k_fold(&config);
}
