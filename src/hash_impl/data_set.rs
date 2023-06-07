use std::{cell::RefCell, rc::Rc};

use super::{Species, Storage};

#[derive(Debug, Default)]
pub struct Total(Vec<Dataset>);

#[derive(Debug, Default)]
pub struct Dataset {
    #[allow(dead_code)]
    training: Vec<Rc<RefCell<Species>>>,
    #[allow(dead_code)]
    testing: Vec<Rc<RefCell<Species>>>,
}

impl Dataset {
    pub fn new(training: Vec<Rc<RefCell<Species>>>, testing: Vec<Rc<RefCell<Species>>>) -> Self {
        Self { training, testing }
    }
}

impl Total {
    pub fn build(storage: &Storage, k: u32) -> Total {
        let mut total = Total::default();
        let mut values: Vec<Rc<RefCell<Species>>> =
            storage.data.values().map(|a| Rc::clone(a)).collect();
        for i in 0..k {
            let len = values.len();
            let lower = len / k as usize * i as usize;
            let upper = if i == k - 1 {
                len
            } else {
                len / k as usize * (i + 1) as usize
            };

            let training = values.drain(lower..upper).collect();
            let data_set = Dataset::new(training, values.clone());
            total.0.push(data_set);
        }
        total
    }
}
