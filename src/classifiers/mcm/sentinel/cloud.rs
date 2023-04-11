use crate::classifiers::mcm::landsat;
use crate::classifiers::Classification;

use gdal::errors::GdalError;
use gdal::raster::Buffer;

pub struct Classifier {
    implementation: landsat::cloud::Classifier,
}

impl Classifier {
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
    fn classify(self) -> Result<Buffer<u32>, GdalError> {
        self.implementation.classify()
    }
}
