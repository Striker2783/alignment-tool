use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

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
        if lineage.len() < 8 {
            for _ in 0..(8 - lineage.len()) {
                lineage.push("");
            }
        }

        if pi >= 98. {
            Some(lineage[0..7].join(";"))
        } else if pi >= 96. {
            Some(lineage[0..6].join(";"))
        } else if pi >= 85. {
            Some(lineage[0..5].join(";"))
        } else if pi >= 80. {
            Some(lineage[0..4].join(";"))
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
        for line in contents.lines() {
            let mut split = line.trim().split('\t');
            let id = split
                .next()
                .ok_or(format!("Failed to get id for {}", line))?
                .to_string();
            let mut lineage: Vec<&str> = split
                .next()
                .ok_or(format!("Failed to get lineage for {}", line))?
                .split(';')
                .collect();
            let _score: f64 = split.next_back().ok_or("No Score")?.parse()?;
            let pi: f64 = split
                .next_back()
                .ok_or(format!("No percentage for {}", line))?
                .parse()?;

            let Some(x) = Self::get_x(&mut lineage, pi) else {
                continue;
            };
            out.map.insert(id, x);
        }

        Ok(out)
    }
}
