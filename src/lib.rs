use std::{env::Args, error::Error};

pub mod hash_mult_impl;

pub struct Config {
    tax: String,
    fasta: String,
    k_fold: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            tax: Default::default(),
            fasta: Default::default(),
            k_fold: 5,
        }
    }
}
impl Config {
    pub fn build(mut args: Args) -> Result<Self, Box<dyn Error>> {
        args.next();
        let tax = args.next().ok_or("No tax argument")?;
        let fasta = args.next().ok_or("No Fasta argument")?;
        if let Some(k) = args.next() {
            return Ok(Config {
                tax,
                fasta,
                k_fold: k.parse()?,
            });
        }

        Ok(Config {
            tax,
            fasta,
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
