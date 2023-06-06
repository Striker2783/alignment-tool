pub mod data_set;
pub mod species;

use std::{collections::HashMap, error::Error, fs, path::Path};

use species::Species;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
#[derive(Default, Debug)]
pub struct Storage {
    data: HashMap<String, Species>,
}

impl Storage {
    pub fn load_tax_file<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = read_file(path)?;
        for x in contents.lines() {
            let species = Species::build(x)?;
            self.data.insert(species.name.clone(), species);
        }
        Ok(())
    }
    pub fn load_fasta_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = read_file(path)?;
        let mut line_iterator = contents.lines();

        while let Some(name) = line_iterator.next() {
            let name = &name[1..];
            let Some(data) = line_iterator
                .next()
                else {Err(format!("{name} has no corresponding genome"))?};

            if let Some(x) = self.data.get_mut(name) {
                x.genome = data.to_owned();
            } else {
                let new_species = Species {
                    name: name.to_owned(),
                    genome: data.to_owned(),
                    ..Default::default()
                };
                self.data.insert(name.to_owned(), new_species);
            };
        }

        Ok(())
    }
}
