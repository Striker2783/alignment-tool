use std::{
    env::{self, Args},
    error::Error,
    path::PathBuf,
};

pub struct Config {
    pub(crate) taxonomy: PathBuf,
    pub(crate) vsearch_output: PathBuf,
    pub(crate) output: PathBuf,
}

impl Config {
    pub fn build(args: &mut Args) -> Result<Self, Box<dyn Error>> {
        let dir = env::current_dir()?;
        let taxonomy = args.next().ok_or("No taxonomy file provided")?;
        let taxonomy = dir.join(taxonomy);

        let vsearch_output = args.next().ok_or("No vsearch output file provided")?;
        let vsearch_output = dir.join(vsearch_output);

        let output = args.next().ok_or("No output provided")?;
        let output = dir.join(output);

        Ok(Config {
            taxonomy,
            vsearch_output,
            output,
        })
    }
}
