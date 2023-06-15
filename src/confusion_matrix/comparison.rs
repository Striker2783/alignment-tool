use std::sync::{Arc, Weak};

use super::species::Species;

/// Possible Values in a Confusion Matrix
#[derive(Debug)]
pub enum PossibleValues {
    /// True Negative: both don't have labels
    TN,
    /// False Positive: Predicted has data, Actual doesn't
    FP,
    /// False Negative: Predicted no data, Actual has data
    FN,
    /// True Positive +: Both have labels that are the same
    TPP,
    /// True Positive -: Both have labels that are difference
    TPN,
}
#[derive(Debug, Default)]
pub struct Comparison {
    species1: Weak<Species>,
    species2: Weak<Species>,
    values: Vec<PossibleValues>,
}

impl Comparison {
    pub fn build(actual: Arc<Species>, predicted: Arc<Species>) -> Self {
        Self {
            species1: Arc::downgrade(&actual),
            species2: Arc::downgrade(&predicted),
            values: vec![
                Self::get_something(&actual.kingdom, &predicted.kingdom),
                Self::get_something(&actual.phylum, &predicted.phylum),
                Self::get_something(&actual.class, &predicted.class),
                Self::get_something(&actual.order, &predicted.order),
                Self::get_something(&actual.family, &predicted.family),
                Self::get_something(&actual.genus, &predicted.genus),
                Self::get_something(&actual.species, &predicted.species),
            ],
        }
    }
    fn get_something(actual: &Option<String>, predicted: &Option<String>) -> PossibleValues {
        match (actual, predicted) {
            (None, None) => PossibleValues::TN,
            (None, Some(_)) => PossibleValues::FP,
            (Some(_), None) => PossibleValues::FN,
            (Some(actual), Some(predicted)) => {
                if actual == predicted {
                    PossibleValues::TPP
                } else {
                    PossibleValues::TPN
                }
            }
        }
    }
}
