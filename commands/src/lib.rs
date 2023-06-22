use std::{env::Args, error::Error};

use confusion_matrix::config::ConfusionConfig as confusion_config;
use k_fold::config::KFoldConfig as k_config;
use metax::config::MetaxConfig as metax_config;
use trim::config::TrimConfig as trim_config;

pub mod confusion_matrix;
pub mod k_fold;
pub mod metax;
pub mod trim;

fn print_help() {
    eprintln!("k (tax_file) (fasta_file) (k = 5)");
    eprintln!("meta (tax_file) (vsearch_file) (output file)");
    eprintln!("trim (meta_file) (output file)");
    eprintln!("conf_mat (tax_file) (trim files directory) (Output file)")
}

pub enum Configs {
    K(k_config),
    Metax(metax_config),
    Trim(trim_config),
    Confusion(confusion_config),
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
            "trim" => {
                let config = trim_config::build(args)?;
                Ok(Self::Trim(config))
            }
            "conf_mat" => {
                let config = confusion_config::build(args)?;
                Ok(Self::Confusion(config))
            }
            "help" | "h" => {
                print_help();
                Ok(Self::Help)
            }
            _ => Err("No Cmd found")?,
        }
    }
}
