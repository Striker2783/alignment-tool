use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use rayon::{prelude::ParallelIterator, str::ParallelString};

use super::config::Config;
#[derive(Debug, Default)]
pub struct Trim {
    map: HashMap<String, String>,
}

impl Trim {
    pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
        let trim = Self::build(&config.meta)?;
        trim.out(&config.out)?;
        Ok(())
    }
    fn get_x(lineage: &mut Vec<&str>, pi: f64) -> Option<String> {
        let len = lineage.len();

        if pi >= 98. {
            Some(lineage[0..7.min(len)].join(";"))
        } else if pi >= 96. {
            Some(lineage[0..6.min(len)].join(";"))
        } else if pi >= 85. {
            Some(lineage[0..5.min(len)].join(";"))
        } else if pi >= 80. {
            Some(lineage[0..4.min(len)].join(";"))
        } else {
            None
        }
    }
    fn out(&self, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(out)?;
        let mut writer = BufWriter::new(&mut file);
        for (key, value) in &self.map {
            writeln!(writer, "{}\t{}", key, value)?;
        }

        Ok(())
    }
    pub fn build(meta: &Path) -> Result<Self, Box<dyn Error>> {
        let mut out = Self::default();

        let contents = fs::read_to_string(meta)?;
        let things: Vec<_> = contents
            .par_lines()
            .filter_map(|line| {
                let mut split = line.trim().split('\t');
                let Some(id) = split.next() else {
                eprintln!("Failed to get id for {}", line);
                return None;
            };
                let Some(lineage) = split.next() else {
                eprintln!("Failed to get lineage for {}", line);
                return None;
            };
                let mut lineage: Vec<_> = lineage.split(';').collect();
                let Some(_score) = split.next_back() else {
                eprintln!("Failed to get score for {}", line);
                return None;
            };
                let _score: f64 = match _score.parse() {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("{e}");
                        return None;
                    }
                };
                let Some(pi) = split.next_back() else {
                eprintln!("Failed to get percentage for {}", line);
                return None;
            };
                let pi: f64 = match pi.parse() {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("{e}");
                        return None;
                    }
                };

                let Some(x) = Self::get_x(&mut lineage, pi) else {
                return None;
            };
                Some((id.to_string(), x))
            })
            .collect();
        for (id, x) in things {
            out.map.insert(id, x);
        }

        Ok(out)
    }
}
