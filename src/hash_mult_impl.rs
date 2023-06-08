pub mod data_set;
pub mod species;

use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use rayon::{prelude::ParallelIterator, slice::ParallelSlice, str::ParallelString};
use species::Species;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
type MultiMut<T> = Arc<Mutex<T>>;
#[derive(Default, Debug)]
pub struct Storage {
    data: HashMap<Arc<String>, MultiMut<Species>>,
}

impl Storage {
    pub fn load_tax_file<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = read_file(path)?;
        let thing: Vec<_> = contents
            .par_lines()
            .filter_map(|line| {
                let Ok(species) = Species::build(line) else {return None;};
                let name = Arc::clone(&species.name);
                let species = Arc::new(Mutex::new(species));
                Some((name, species))
            })
            .collect();
        for (name, species) in thing.into_iter() {
            self.data.insert(name, species);
        }
        Ok(())
    }
    pub fn load_fasta_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = read_file(path)?;

        contents
            .par_lines()
            .collect::<Vec<&str>>()
            .par_chunks_exact(2)
            .for_each(|line| {
                let name = &line[0][1..];
                let Some(x) = self.data.get(&Arc::new(name.to_owned())) else {
                    return;
                };
                let clone = Arc::clone(x);
                let mut species = clone.lock().unwrap();
                species.genome = line[1].to_owned();
            });

        Ok(())
    }
}
