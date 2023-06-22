use std::{
    env::{self, Args},
    error::Error,
    path::PathBuf,
};

pub struct ConfusionConfig {
    pub(crate) tax: PathBuf,
    pub(crate) predicted_dir: PathBuf,
    pub(crate) out: PathBuf,
}
impl ConfusionConfig {
    pub fn build(args: &mut Args) -> Result<Self, Box<dyn Error>> {
        let dir = env::current_dir()?;
        let tax = args.next().ok_or("No tax inputted")?;
        let tax = dir.join(tax);
        let predicted_dir = args.next().ok_or("No predicted directory inputted")?;
        let predicted_dir = dir.join(predicted_dir);
        let out = args.next().ok_or("No output file inputted")?;
        let out = dir.join(out);
        Ok(Self {
            tax,
            predicted_dir,
            out,
        })
    }
}
