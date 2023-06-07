use rayon::prelude::IntoParallelRefIterator;

use super::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total(Vec<Dataset>);

#[derive(Debug, Default)]
pub struct Dataset {
    #[allow(dead_code)]
    training: Vec<Species>,
    #[allow(dead_code)]
    testing: Vec<Species>,
}

impl Dataset {
    pub fn new(training: Vec<Species>, testing: Vec<Species>) -> Self {
        Self { training, testing }
    }
}

impl Total {
    pub fn build(storage: &Storage, k: u32) -> Total {
        let mut total = Total::default();
        let mut values: Vec<Species> = storage
            .data
            .borrow()
            .iter()
            .map(|(_, species)| species.clone())
            .collect();
        for i in 0..k {
            let len = values.len();
            let lower = len / k as usize * i as usize;
            let upper = if i == k - 1 {
                len
            } else {
                len / k as usize * (i + 1) as usize
            };

            let training = values.drain(lower..upper).collect();
            let data_set = Dataset::new(training, values.to_vec());
            total.0.push(data_set);
        }
        total
    }
}
