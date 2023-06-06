pub mod data_set;
pub mod species;

use std::{collections::HashMap, error::Error};

use species::Species;

#[derive(Default, Debug)]
pub struct Storage<'a> {
    data: HashMap<&'a str, Species<'a>>,
}

impl<'a> Storage<'a> {
    pub fn load_tax(&mut self, content: &'a str) -> Result<(), Box<dyn Error>> {
        for x in content.lines() {
            let species = Species::build(x)?;
            self.data.insert(&species.name, species);
        }
        Ok(())
    }
    pub fn load_fasta(&mut self, content: &'a str) -> Result<(), Box<dyn Error>> {
        let mut line_iterator = content.lines();

        while let Some(name) = line_iterator.next() {
            let name = &name[1..];
            let Some(genome) = line_iterator
                .next()
                else {Err(format!("{name} has no corresponding genome"))?};

            if let Some(x) = self.data.get_mut(name) {
                x.genome = genome;
            } else {
                let new_species = Species {
                    name,
                    genome,
                    ..Default::default()
                };
                self.data.insert(&name, new_species);
            };
        }

        Ok(())
    }
}
