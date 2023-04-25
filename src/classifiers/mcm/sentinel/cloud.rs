use crate::classifiers::mcm::landsat;
use crate::classifiers::Classification;

use gdal::errors::GdalError;
use gdal::raster::Buffer;

/// Cloud classifier object for Landsat 8-9 images.
/// # Examples
/// ```
/// use cloud_detection::classifiers::mcm::sentinel;
///
/// // -- irrelevant code --
///
/// // Create classifier
/// let classifier = sentinel::cloud::Classifier::from_path("./reference.tif", "/target.tif").unwrap();
///
/// // Classify the image
/// let res_image = classifier.classify().unwrap();
///
/// // Save the results
/// persistence::tif::save("./reference.tif", "./result.tif", &res_image);
///
/// // -- irrelevant code --
/// ```
pub struct Classifier {
    implementation: landsat::cloud::Classifier,
}

impl Classifier {
    /// Creates a classifier from the given target and referenc image paths.
    pub fn from_path(
        reference_image_path: &str,
        target_image_path: &str,
    ) -> Result<Classifier, GdalError> {
        let implementation =
            landsat::cloud::Classifier::from_path(reference_image_path, target_image_path)?;
        Ok(Classifier { implementation })
    }
}

impl Classification<u32> for Classifier {
    /// Creates a cloud mask for Landsat 8-9 images.
    fn classify(self) -> Result<Buffer<u32>, GdalError> {
        self.implementation.classify()
    }
}
