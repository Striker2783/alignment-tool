use std::{
    env::{self, Args},
    error::Error,
    path::PathBuf,
};

pub struct TrimConfig {
    pub(crate) meta: PathBuf,
    pub(crate) out: PathBuf,
}

impl TrimConfig {
    pub fn new(meta: PathBuf, out: PathBuf) -> Self {
        Self { meta, out }
    }

    pub fn build(args: &mut Args) -> Result<Self, Box<dyn Error>> {
        let dir = env::current_dir()?;
        let meta = args.next().ok_or("Failed to get tax argument")?;
        let meta = dir.join(meta);

        let out = args.next().ok_or("Failed to get out argument")?;
        let out = dir.join(out);

        Ok(TrimConfig { meta, out })
    }
}
