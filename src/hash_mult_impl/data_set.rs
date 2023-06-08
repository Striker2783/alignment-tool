use std::sync::{Arc, Mutex};

use rayon::prelude::{IntoParallelRefIterator, ParallelDrainRange, ParallelIterator};

use super::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total(Vec<Dataset>);

type SpeciesVec = Vec<Arc<Mutex<Species>>>;
#[derive(Debug, Default)]
pub struct Dataset {
    #[allow(dead_code)]
    training: SpeciesVec,
    #[allow(dead_code)]
    testing: SpeciesVec,
}

impl Dataset {
    #[inline]
    pub fn new(training: SpeciesVec, testing: SpeciesVec) -> Self {
        Self { training, testing }
    }
}

impl Total {
    pub fn build(storage: &Storage, k: u32) -> Total {
        let mut total = Total::default();
        let mut values: SpeciesVec = storage
            .data
            .par_iter()
            .map(|(_, a)| Arc::clone(a))
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
        total
    }
}
