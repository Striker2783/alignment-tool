use std::{
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use rayon::prelude::{IntoParallelRefIterator, ParallelDrainRange, ParallelIterator};

use crate::Config;

use super::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total(Vec<Dataset>);

type SpeciesVec = Vec<Arc<Mutex<Species>>>;
#[derive(Debug, Default)]
pub struct Dataset {
    #[allow(dead_code)]
    training: SpeciesVec,
    #[allow(dead_code)]
    testing: SpeciesVec,
}

impl Dataset {
    #[inline]
    pub fn new(training: SpeciesVec, testing: SpeciesVec) -> Self {
        Self { training, testing }
    }
    fn create_tax_file(species: &SpeciesVec, file: &mut File) {
        let mut writer = BufWriter::new(file);
        for species in species {
            let Ok(species) = species.lock() else {continue;};
            let _ = writeln!(writer, "{}", &species.get_tax());
        }
    }
    fn create_fasta_file(species: &SpeciesVec, file: &mut File) {
        let mut writer = BufWriter::new(file);
        for species in species {
            let Ok(species) = species.lock() else {continue;};
            let _ = writeln!(writer, "{}", &species.get_fasta());
        }
    }
    pub fn create_files(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        self.create_fasta_training_file(directory)?;
        self.create_tax_training_file(directory)?;
        self.create_fasta_testing_file(directory)?;
        self.create_tax_testing_file(directory)?;
        Ok(())
    }
    fn create_tax_training_file(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(directory.to_owned().join("training.tax"))?;
        Self::create_tax_file(&self.training, &mut file);
        Ok(())
    }
    fn create_fasta_training_file(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(directory.to_owned().join("training.fasta"))?;
        Self::create_fasta_file(&self.training, &mut file);
        Ok(())
    }
    fn create_tax_testing_file(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(directory.to_owned().join("testing.tax"))?;
        Self::create_tax_file(&self.testing, &mut file);
        Ok(())
    }
    fn create_fasta_testing_file(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(directory.to_owned().join("testing.fasta"))?;
        Self::create_fasta_file(&self.testing, &mut file);
        Ok(())
    }
}

impl Total {
    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        let mut storage = Storage::default();
        storage.load_tax_file(&config.tax)?;
        storage.load_fasta_file(&config.fasta)?;
        let data_set = Self::build(&storage, config.k_fold);
        let path = config.dir.join("files");
        let path = Path::new(&path);
        fs::create_dir(path)?;
        data_set.write_data(path)?;

        Ok(())
    }
    pub fn build(storage: &Storage, k: u32) -> Total {
        let mut total = Total::default();
        let values: Vec<_> = storage
            .data
            .par_iter()
            .map(|(_, species)| Arc::clone(species))
            .collect();

        for i in 0..k {
            let len = values.len();
            let lower = len / k as usize * i as usize;
            let upper = if i == k - 1 {
                len
            } else {
                len / k as usize * (i + 1) as usize
            };

            let data_set = Dataset::new(
                [&values[..lower], &values[upper..]]
                    .concat()
                    .par_iter()
                    .map(|species| Arc::clone(species))
                    .collect(),
                values[lower..upper]
                    .par_iter()
                    .map(|species| Arc::clone(species))
                    .collect(),
            );
            total.0.push(data_set);
        }

        total
    }
    pub fn write_data(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        (&self.0)
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .par_iter()
            .for_each(|(i, data_set)| {
                let path = path.to_owned().join(format!("Dataset{}", i));
                let _ = fs::create_dir(path.as_path());
                let _ = data_set.create_files(&path);
            });

        Ok(())
    }
}
