use std::{
    fmt::Display,
    sync::{Arc, Weak},
};

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

impl Display for PossibleValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PossibleValues::TN => write!(f, "TN"),
            PossibleValues::FP => write!(f, "FP"),
            PossibleValues::FN => write!(f, "FN"),
            PossibleValues::TPP => write!(f, "TPP"),
            PossibleValues::TPN => write!(f, "TPN"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Comparison {
    pub(crate) _actual: Weak<Species>,
    pub(crate) predicted: Weak<Species>,
    pub(crate) values: Vec<PossibleValues>,
}

impl Comparison {
    pub fn build(actual: Arc<Species>, predicted: Arc<Species>) -> Self {
        Self {
            _actual: Arc::downgrade(&actual),
            predicted: Arc::downgrade(&predicted),
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
    pub fn get_values(&self) -> String {
        format!(
            "k__{};p__{};c__{};o__{};f__{};g__{};s__{}",
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
            self.values[4],
            self.values[5],
            self.values[6]
        )
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
