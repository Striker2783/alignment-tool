use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
    sync::Arc,
};

use super::{comparison::Comparison, config::ConfusionConfig, species::Species};
type Comparisons = Vec<Comparison>;

#[derive(Debug, Default)]
pub struct Data {
    actual: HashMap<String, Arc<Species>>,
    predicted: HashMap<String, Arc<Species>>,
    comparisons: Comparisons,
}

impl Data {
    pub fn build(config: &ConfusionConfig) -> Result<Self, Box<dyn Error>> {
        let mut new = Self::default();
        new.fill_actual(&config.tax)?;
        new.fill_predicted(&config.predicted_dir)?;
        new.comparisons();
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
    pub fn comparisons(&mut self) {
        for (name, predicted) in &self.predicted {
            let predicted = Arc::clone(predicted);
            let Some(actual) = self.actual.get(name) else {
                eprintln!("No taxonomy found for {}", name);
                continue;
            };
            let actual = Arc::clone(actual);
            let comparison = Comparison::build(actual, predicted);
            self.comparisons.push(comparison);
        }
    }
    pub fn output(&self, output: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(output)?;
        let mut buffer_writer = BufWriter::new(&mut file);
        for comparison in &self.comparisons {
            let Some(predicted) = comparison.predicted.upgrade() else {
                eprintln!("Somehow something got dropped");
                continue;
            };
            writeln!(
                buffer_writer,
                "{}\t{}",
                predicted.name,
                comparison.get_values()
            )?;
        }
        Ok(())
    }
}
