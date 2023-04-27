//! Contains tools for comparing classification results using different masking methods

use confusion_matrix::{self, ConfusionMatrix};

/// Creates a confusion matrix from a reference classification and target classification.
/// The resulting dataframe will have the reference image as the columns and the target image as the rows.
pub fn create_confusion_matrix(
    reference_classification: &Vec<u32>,
    target_classification: &Vec<u32>,
) -> ConfusionMatrix
where
{
    let mut matrix = confusion_matrix::new();

    for (&r, &t) in reference_classification.iter().zip(target_classification) {
        matrix.add_for(&r.to_string(), &t.to_string())
    }

    matrix
}
