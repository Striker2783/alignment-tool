use k_fold_cross_validation::{
    k_fold::{config::Config, data_set::Total},
    metax::{config::Config as meta_config, output::Metax},
    Configs,
};
use std::{env, process, time::Instant};

fn k_fold(config: &Config) {
    let time = Instant::now();

    if let Err(e) = Total::run(config) {
        eprintln!("{e}");
    };

    println!("Hash Concurrency: {} ms", time.elapsed().as_millis());
}
fn meta(config: &meta_config) {
    let time = Instant::now();

    if let Err(e) = Metax::run(config) {
        eprintln!("{e}");
    }

    println!("Meta Convertion: {} ms", time.elapsed().as_millis());
}
fn main() {
    let mut args = env::args();
    args.next();

    let config = match Configs::get(&mut args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    match config {
        Configs::K(config) => k_fold(&config),
        Configs::Metax(config) => meta(&config),
        Configs::Help => (),
    }
}
