pub mod config;
pub mod data_set;

use std::{collections::HashMap, error::Error, fs, path::Path};

use rayon::{prelude::ParallelIterator, str::ParallelString};

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
type Data = (String, Option<String>);

#[derive(Default, Debug)]
pub struct Storage {
    data: HashMap<String, Data>,
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
                let mut split = line.split('\t');
                let Some(name) = split.next() else {
                    return None;
                };
                let name = name.to_string();
                let Some(species) = split.next() else {
                    return None;
                };
                let species = species.to_string();
                Some((name, species))
            })
            .collect();
        for (name, species) in thing.into_iter() {
            self.data.insert(name, (species, None));
        }
        Ok(())
    }
    pub fn load_fasta_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>> {
        let contents = read_file(path)?;

        contents
            .lines()
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .for_each(|line| {
                let name = &line[0][1..];
                let genome = line[1];
                let Some((_,x)) = self.data.get_mut(name) else {
                    return;
                };
                *x = Some(genome.to_string());
            });

        Ok(())
    }
}
