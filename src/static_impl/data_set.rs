use super::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total<'a>(Vec<Dataset<'a>>);

#[derive(Debug, Default)]
pub struct Dataset<'a> {
    #[allow(dead_code)]
    training: Vec<&'a Species<'a>>,
    #[allow(dead_code)]
    testing: Vec<&'a Species<'a>>,
}

impl<'a> Dataset<'a> {
    pub fn new(training: Vec<&'a Species>, testing: Vec<&'a Species>) -> Self {
        Self { training, testing }
    }
}

impl<'a> Total<'a> {
    pub fn build(storage: &'a Storage, k: u32) -> Total<'a> {
        let mut total = Total::default();
        let mut values: Vec<&'a Species> = storage.data.values().collect();
        for i in 0..k {
            let len = values.len();
            let lower = len / k as usize * i as usize;
            let upper = if i == k - 1 {
                len
            } else {
                len / k as usize * (i + 1) as usize
            };

            let training: Vec<&'a Species> = values.drain(lower..upper).collect();
            let data_set = Dataset::new(training, values.to_vec());
            total.0.push(data_set);
        }
        total
    }
}
