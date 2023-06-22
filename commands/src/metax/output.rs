use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use rayon::{prelude::ParallelIterator, str::ParallelString};

use super::config::Config;

type Val = (String, Option<(String, String, String)>);

#[derive(Debug, Default, Clone)]
pub struct Metax {
    tax: HashMap<String, Val>,
}
impl Metax {
    fn make_tax(&mut self, tax: &Path) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(tax)?;
        let items: Vec<(&str, &str)> = contents
            .par_lines()
            .filter_map(|line| {
                let mut split = line.trim().split('\t');
                let Some(name) = split.next() else {
                    return None;
                };
                let Some(line) = split.next() else {
                    return None;
                };
                Some((name, line))
            })
            .collect();
        for (name, line) in items {
            self.tax.insert(name.to_owned(), (line.to_owned(), None));
        }
        Ok(())
    }
    fn use_vsearch(&mut self, vsearch: &Path) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(vsearch)?;
        let things: Vec<_> = contents
            .par_lines()
            .filter_map(Self::filter_vsearch_line)
            .collect();

        for (id, o) in things.into_iter() {
            let Some(thing) = self.tax.get_mut(id) else {
                eprintln!("No key named {}.", id);
                continue;
            };
            thing.1 = Some(o);
        }

        Ok(())
    }
    fn out(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path)?;
        let mut writer = BufWriter::new(&mut file);
        for value in self.tax.iter() {
            let Some(o) = &value.1.1 else {
                continue;
            };
            let Some(other) = self.tax.get(&o.0) else {
                eprintln!("Failed to get taxonomy of {}", o.0);
                continue;
            };
            writeln!(writer, "{}\t{}\t{}\t{}", value.0, other.0, o.1, o.2)?;
        }
        Ok(())
    }
    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        let new = Self::build(config)?;
        new.out(&config.output)?;
        Ok(())
    }
    pub fn build(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut out = Self::default();
        out.make_tax(&config.taxonomy)?;
        out.use_vsearch(&config.vsearch_output)?;

        Ok(out)
    }

    fn filter_vsearch_line(line: &str) -> Option<(&str, (String, String, String))> {
        let mut split = line.split('\t');
        let Some(id) = split.next() else {
        eprintln!("No id for {}", line);
        return None;
    };
        let Some(target) = split.next() else {
        eprintln!("No target id for {}", line);
        return None;
    };
        let Some(identity) = split.next() else {
        eprintln!("No percentage of identity found for {}", line);
        return None;
    };
        let Some(len) = split.next() else {
        eprintln!("No length found for {}", line);
        return None;
    };
        Some((id, (target.to_owned(), identity.to_owned(), len.to_owned())))
    }
}
