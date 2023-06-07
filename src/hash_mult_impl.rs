pub mod data_set;
pub mod species;

use rayon::prelude::*;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    error::Error,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use species::Species;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
#[derive(Default, Debug)]
pub struct Storage {
    data: Arc<Mutex<HashMap<String, Arc<Mutex<Species>>>>>,
}

impl Storage {
    pub fn load_tax_file<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = read_file(path)?;
        contents
            .par_lines()
            .for_each_with(Arc::clone(&self.data), |hash, line| {
                let Ok(species) = Species::build(line) else {return;};
                let name = species.name.to_owned();
                let lock = &mut hash.lock();
                let Ok(hash) = lock.as_mut() else{return;};
                hash.insert(name, Arc::new(Mutex::new(species)));
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
            .for_each_with(Arc::clone(&self.data), |hash, (name, data)| {
                let lock = &mut hash.lock();
                let Ok(hash) = lock.as_mut() else {return;};
                if let Some(x) = hash.get(*name) {
                    let mut x = Arc::clone(&x).borrow_mut();
                    let mut lock = match x.lock() {
                        Ok(a) => a,
                        Err(_) => return,
                    };
                    lock.borrow_mut().genome = (*data).to_owned();
                } else {
                    let new_species = Species {
                        name: (*name).to_owned(),
                        genome: (*data).to_owned(),
                        ..Default::default()
                    };
                    hash.insert((*name).to_owned(), Arc::new(Mutex::new(new_species)));
                }
            });

        Ok(())
    }
}
