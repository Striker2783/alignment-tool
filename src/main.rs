use k_fold_cross_validation::{
    k_fold::{config::Config, data_set::Total},
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
fn main() {
    let mut args = env::args();
    args.next();

    let config = match Configs::get(args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
    match config {
        Configs::K(config) => k_fold(&config),
        Configs::Metax => todo!(),
        Configs::Help => (),
    }
}
