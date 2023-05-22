use crate::classifiers::{util, Classification};
use crate::persistence::config::CloudDetectionConfig;

use gdal::errors::GdalError;
use gdal::raster::Buffer;
use gdal::Dataset;

/// Cloud classifier object for Sentinel-2 images.
/// # Examples
/// ```no_run
/// use cloud_detection::classifiers::Classification;
/// use cloud_detection::classifiers::mcm::sentinel;
/// use cloud_detection::persistence;
/// use crate::persistence::config::CloudDetectionConfig;
///
/// fn main () {
///     // Create classifier
///     let classifier = sentinel::cloud::Classifier::from_path("./reference.tif", "/target.tif", &CloudDetectionConfig::default()).unwrap();
///     // Classify the image
///     let res_image = classifier.classify().unwrap();
///     // Save the results
///     persistence::tif::save("./reference.tif", "./result.tif", &res_image);
/// }
/// ```
pub struct Classifier {
    target: Dataset,
    reference: Dataset,
    useful_bands: Vec<isize>,
}

impl Classifier {
    /// Creates a classifier from the given target and referenc image paths.
    pub fn from_path(
        reference_image_path: &str,
        target_image_path: &str,
        config: &CloudDetectionConfig,
    ) -> Result<Classifier, GdalError> {
        let reference_image = Dataset::open(reference_image_path)?;
        let target_image = Dataset::open(target_image_path)?;

        let useful_bands = vec![
            config.sentinel.green_band_index,
            config.sentinel.red_band_index,
        ];

        let min_band_count = isize::from(useful_bands.iter().max().unwrap().clone());

        util::validate(&reference_image, &target_image, min_band_count)?;

        Ok(Classifier {
            target: target_image,
            reference: reference_image,
            useful_bands: useful_bands,
        })
    }
}

impl Classification<u32> for Classifier {
    /// Creates a cloud mask for Landsat 8-9 images.
    fn classify(self) -> Result<Buffer<u32>, GdalError> {
        let reference_rasters = util::get_rasters(self.reference, &self.useful_bands)?;
        let target_rasters = util::get_rasters(self.target, &self.useful_bands)?;

        let deltas: Vec<Vec<f32>> = target_rasters
            .iter()
            .zip(&reference_rasters)
            .map(|(t, r)| util::diff(&t.data, &r.data))
            .collect();

        let mask = deltas[0]
            .iter()
            .zip(&deltas[1])
            .map(|(&l, &r)| (l > 0.15 && r > 0.15) as u32)
            .collect();

        Ok(Buffer::new(reference_rasters[0].size, mask))
    }
}
