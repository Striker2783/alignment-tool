use std::{collections::HashMap, error::Error, fs, path::Path, sync::Arc};

use super::{comparison::Comparison, config::Config, species::Species};
#[derive(Debug, Default)]
pub struct Data {
    actual: HashMap<String, Arc<Species>>,
    predicted: HashMap<String, Arc<Species>>,
    comparisons: Vec<Comparison>,
}

impl Data {
    pub fn build(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut new = Self::default();
        new.fill_actual(&config.tax)?;
        new.fill_predicted(&config.predicted_dir)?;
        Ok(new)
    }
    pub fn fill_actual(&mut self, file: &Path) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(file)?;
        for line in contents.lines() {
            let species = Species::build(line)?;
            let name = species.name.to_string();
            self.actual.insert(name, Arc::new(species));
        }

        Ok(())
    }
    pub fn fill_predicted(&mut self, file: &Path) -> Result<(), Box<dyn Error>> {
        if file.is_file() {
            Err("Predicted should be a directory")?;
        }
        for path in fs::read_dir(file)? {
            let path = path?.path();
            let contents = fs::read_to_string(path)?;
            for line in contents.lines() {
                let species = Species::build(line)?;
                let name = species.name.to_string();
                self.predicted.insert(name, Arc::new(species));
            }
        }
        Ok(())
    }
}
