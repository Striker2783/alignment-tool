use std::{
    env::{self, Args},
    error::Error,
    path::PathBuf,
};

pub struct KFoldConfig {
    pub(crate) tax: String,
    pub(crate) fasta: String,
    pub(crate) k_fold: u32,
    pub(crate) dir: PathBuf,
}
impl Default for KFoldConfig {
    fn default() -> Self {
        Self {
            k_fold: 5,
            fasta: Default::default(),
            tax: Default::default(),
            dir: Default::default(),
        }
    }
}
impl KFoldConfig {
    pub fn new(tax: String, fasta: String, k_fold: u32, dir: PathBuf) -> Self {
        Self {
            tax,
            fasta,
            k_fold,
            dir,
        }
    }

    pub fn build(args: &mut Args) -> Result<Self, Box<dyn Error>> {
        let tax = args.next().ok_or("No tax argument")?;
        let fasta = args.next().ok_or("No Fasta argument")?;
        let dir = env::current_dir()?;
        if let Some(k) = args.next() {
            return Ok(KFoldConfig {
                tax,
                fasta,
                dir,
                k_fold: k.parse()?,
            });
        }

        Ok(KFoldConfig {
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
