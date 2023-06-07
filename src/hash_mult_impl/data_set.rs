use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;

use super::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total(Vec<Dataset>);

#[derive(Debug, Default)]
pub struct Dataset {
    #[allow(dead_code)]
    training: Vec<Arc<Species>>,
    #[allow(dead_code)]
    testing: Vec<Arc<Species>>,
}

impl Dataset {
    pub fn new(training: Vec<Arc<Species>>, testing: Vec<Arc<Species>>) -> Self {
        Self { training, testing }
    }
}

impl Total {
    pub fn build(storage: Storage, k: u32) -> Result<Total, Box<dyn Error>> {
        let mut total = Total::default();
        let mut lock = storage.data.lock();
        let Ok(lock) = lock.as_mut() else {
            Err("What")?
        };
        let mut values: Vec<Arc<Mutex<Species>>> = lock
            .par_iter()
            .map(|(_, species)| Arc::clone(species))
            .collect();
        for i in 0..k {
            let len = values.len();
            let lower = len / k as usize * i as usize;
            let upper = if i == k - 1 {
                len
            } else {
                len / k as usize * (i + 1) as usize
            };

            let training = values.par_drain(lower..upper).collect();
            let data_set = Dataset::new(training, values.clone());
            total.0.push(data_set);
        }
        Ok(total)
    }
}
