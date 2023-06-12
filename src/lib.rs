use std::{env::Args, error::Error};

use k_fold::config::Config;

pub mod k_fold;

fn print_help() {
    eprintln!("k (tax_file) (fasta_file) (k = 5)");
    eprintln!("meta unimplemented");
}

pub enum Configs {
    K(Config),
    Metax,
    Help,
}

impl Configs {
    pub fn get(mut args: Args) -> Result<Self, Box<dyn Error>> {
        let str = args.next().ok_or("Did not find args")?;
        match str.to_lowercase().as_str() {
            "k" => {
                let config = Config::build(args)?;
                Ok(Self::K(config))
            }
            "meta" => Ok(Self::Metax),
            "help" | "h" => {
                print_help();
                Ok(Self::Help)
            }
            _ => Err("No Cmd found")?,
        }
    }
}
