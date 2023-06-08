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
    data: MultiMut<HashMap<Arc<String>, MultiMut<Species>>>,
}

impl Storage {
    pub fn load_tax_file<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = read_file(path)?;
        contents.par_lines().for_each(|line| {
            let Ok(species) = Species::build(line) else {return;};
            let name = Arc::clone(&species.name);
            let mut map = self.data.lock().unwrap();
            map.insert(name, Arc::new(Mutex::new(species)));
        });
        Ok(())
    }
    /// No multithreading because hashmap ;-;
    pub fn load_fasta_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = read_file(path)?;

        contents
            .par_lines()
            .collect::<Vec<&str>>()
            .par_chunks_exact(2)
            .for_each(|line| {
                let name = &line[0][1..];

                let mut map = self.data.lock().unwrap();
                let Some(x) = map.get(&Arc::new(name.to_owned())) else {
                let name = Arc::new(name.to_owned());
                let new_species = Species {
                    name: Arc::clone(&name),
                    genome: line[1].to_owned(),
                    ..Default::default()
                };
                map.insert(name, Arc::new(Mutex::new(new_species)));
                return;
            };
                let clone = Arc::clone(x);
                drop(map);
                let mut species = clone.lock().unwrap();
                species.genome = line[1].to_owned();
            });

        Ok(())
    }
}
