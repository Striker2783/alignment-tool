pub mod config;
pub mod confusion;
pub mod data;
pub mod species;

/// Possible Values in a Confusion Matrix
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
