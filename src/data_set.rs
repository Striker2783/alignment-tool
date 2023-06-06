use crate::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total<'a>(Vec<Dataset<'a>>);

#[derive(Debug, Default)]
pub struct Dataset<'a> {
    #[allow(dead_code)]
    training: Vec<&'a Species>,
    #[allow(dead_code)]
    testing: Vec<&'a Species>,
}

impl<'a> Dataset<'a> {
    pub fn new(training: Vec<&'a Species>, testing: Vec<&'a Species>) -> Self {
        Self { training, testing }
    }
}

impl<'a> Total<'a> {
    pub fn build(storage: &Storage, k: u32) -> Total {
        let mut total = Total::default();
        let values: Vec<&Species> = storage.data.values().collect();
        let mut values = &values[..];
        for i in 0..k {
            let len = values.len();
            let lower = len / k as usize * i as usize;
            let upper = if i == k - 1 {
                len
            } else {
                len / k as usize * (i + 1) as usize
            };

            let training = &values[lower..upper];
            let testing = [&values[upper..], &values[..lower]].concat();
            let data_set = Dataset::new(training.to_vec(), testing.to_vec());
            total.0.push(data_set);

            values = &values[upper..];
        }
        total
    }
}
