pub mod data_set;
pub mod species;

use rayon::prelude::*;
use std::{cell::RefCell, collections::HashMap, error::Error, fs, path::Path};

use species::Species;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
#[derive(Default, Debug)]
pub struct Storage {
    data: RefCell<HashMap<String, Species>>,
}

impl Storage {
    pub fn load_tax_file<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = read_file(path)?;
        contents
            .par_lines()
            .for_each_with(RefCell::clone(&self.data), |hash, line| {
                let hash = hash.get_mut();
                let Ok(species) = Species::build(line) else {return;};
                let name = species.name.to_owned();
                hash.insert(name, species);
            });

        Ok(())
    }
    pub fn load_fasta_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = read_file(path)?;
        let mut line_iterator = contents.lines();
        let mut lines: Vec<(&str, &str)> = vec![];
        while let Some(name) = line_iterator.next() {
            let name = &name[1..];
            let Some(data) = line_iterator
                .next()
                else {Err(format!("{name} has no corresponding genome"))?};
            lines.push((name, data));
        }
        lines
            .par_iter()
            .for_each_with(self.data.clone(), |init, (name, data)| {
                if let Some(x) = init.get_mut().get_mut(*name) {
                    x.genome = (*data).to_owned();
                } else {
                    let new_species = Species {
                        name: (*name).to_owned(),
                        genome: (*data).to_owned(),
                        ..Default::default()
                    };
                    init.get_mut().insert((*name).to_owned(), new_species);
                }
            });

        Ok(())
    }
}
