pub mod data_set;
pub mod species;

use std::{cell::RefCell, collections::HashMap, error::Error, fs, path::Path, rc::Rc};

use species::Species;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}
#[derive(Default, Debug)]
pub struct Storage {
    data: HashMap<Rc<String>, Rc<RefCell<Species>>>,
}

impl Storage {
    pub fn load_tax_file<P>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let contents = read_file(path)?;
        for x in contents.lines() {
            let species = Species::build(x)?;
            let name = Rc::clone(&species.name);
            self.data.insert(name, Rc::new(RefCell::new(species)));
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

            if let Some(x) = self.data.get(&Rc::new(name.to_owned())) {
                let clone = &Rc::clone(x);
                let mut species = clone.borrow_mut();
                species.genome = data.to_owned();
            } else {
                let name = Rc::new(name.to_owned());
                let new_species = Species {
                    name: Rc::clone(&name),
                    genome: data.to_owned(),
                    ..Default::default()
                };
                self.data.insert(name, Rc::new(RefCell::new(new_species)));
            };
        }

        Ok(())
    }
}
