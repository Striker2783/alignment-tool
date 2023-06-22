use std::error::Error;

use super::{config::ConfusionConfig, data::Data};

pub struct Confusion {}
impl Confusion {
    pub fn run(config: &ConfusionConfig) -> Result<(), Box<dyn Error>> {
        let data = Data::build(config)?;
        data.output(&config.out)?;
        Ok(())
    }
}
