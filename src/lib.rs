use std::{
    env::{self, Args},
    error::Error,
    path::PathBuf,
};

pub mod hash_mult_impl;

pub struct Config {
    tax: String,
    fasta: String,
    k_fold: u32,
    dir: PathBuf,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            k_fold: 5,
            dir: Default::default(),
            fasta: Default::default(),
            tax: Default::default(),
        }
    }
}
impl Config {
    pub fn build(mut args: Args) -> Result<Self, Box<dyn Error>> {
        args.next();
        let tax = args.next().ok_or("No tax argument")?;
        let fasta = args.next().ok_or("No Fasta argument")?;
        let dir = env::current_dir()?;
        if let Some(k) = args.next() {
            return Ok(Config {
                tax,
                fasta,
                k_fold: k.parse()?,
                dir,
            });
        }

        Ok(Config {
            tax,
            fasta,
            dir,
            ..Default::default()
        })
    }

    pub fn tax(&self) -> &str {
        self.tax.as_ref()
    }

    pub fn fasta(&self) -> &str {
        self.fasta.as_ref()
    }

    pub fn k_fold(&self) -> u32 {
        self.k_fold
    }
}
