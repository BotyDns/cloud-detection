use std::ops::Range;

use crate::classifiers::util;
use crate::classifiers::Classification;

use gdal::errors::GdalError;
use gdal::raster::Buffer;
use gdal::Dataset;

const MIN_RASTER_COUNT: isize = 6;
const USEFUL_BAND_RANGE: Range<isize> = 1..MIN_RASTER_COUNT + 1;

pub struct Classifier {
    target: Dataset,
    reference: Dataset,
}

impl Classifier {
    pub fn from_path(
        reference_image_path: &str,
        target_image_path: &str,
    ) -> Result<Classifier, GdalError> {
        let reference_image = Dataset::open(reference_image_path)?;
        let target_image = Dataset::open(target_image_path)?;

        util::validate(&reference_image, &target_image, MIN_RASTER_COUNT)?;

        Ok(Classifier {
            target: target_image,
            reference: reference_image,
        })
    }
}

impl Classification<u32> for Classifier {
    fn classify(self) -> Result<Buffer<u32>, GdalError> {
        let reference_rasters = util::get_rasters(self.reference, USEFUL_BAND_RANGE)?;
        let target_rasters = util::get_rasters(self.target, USEFUL_BAND_RANGE)?;

        let deltas: Vec<Vec<f32>> = (2..6)
            .map(|i| util::diff(&target_rasters[i].data, &reference_rasters[i].data))
            .collect();

        let mask = deltas[0]
            .iter()
            .zip(&deltas[1])
            .map(|(&l, &r)| (l > 0.04 && r > 0.04) as u32)
            .collect();

        Ok(Buffer::new(reference_rasters[0].size, mask))
    }
}
