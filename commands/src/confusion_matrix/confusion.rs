use std::error::Error;

use super::{config::Config, data::Data};

pub struct Confusion {}
impl Confusion {
    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        let data = Data::build(config)?;
        data.output(&config.out)?;
        Ok(())
    }
}
