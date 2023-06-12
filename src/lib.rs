use std::{env::Args, error::Error};

use k_fold::config::Config as k_config;
use metax::config::Config as metax_config;

pub mod k_fold;
pub mod metax;

fn print_help() {
    eprintln!("k (tax_file) (fasta_file) (k = 5)");
    eprintln!("meta unimplemented");
}

pub enum Configs {
    K(k_config),
    Metax(metax_config),
    Help,
}

impl Configs {
    pub fn get(args: &mut Args) -> Result<Self, Box<dyn Error>> {
        let str = args.next().ok_or("Did not find args")?;
        match str.to_lowercase().as_str() {
            "k" => {
                let config = k_config::build(args)?;
                Ok(Self::K(config))
            }
            "meta" => {
                let config = metax_config::build(args)?;
                Ok(Self::Metax(config))
            }
            "help" | "h" => {
                print_help();
                Ok(Self::Help)
            }
            _ => Err("No Cmd found")?,
        }
    }
}
