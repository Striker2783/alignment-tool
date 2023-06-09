use k_fold_cross_validation::{hash_mult_impl::data_set::Total, Config};
use std::{env, process, time::Instant};

fn hash_mult_impl(config: &Config) {
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
    hash_mult_impl(&config);
}
