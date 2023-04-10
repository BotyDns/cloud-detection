use crate::classifiers::util::diff;
use crate::classifiers::Classification;

use gdal::errors::GdalError;
use gdal::raster::Buffer;
use gdal::Dataset;

const MIN_RASTER_COUNT: isize = 6;

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

        Self::validate(&reference_image, &target_image)?;

        Ok(Classifier {
            target: target_image,
            reference: reference_image,
        })
    }

    fn validate(reference_dataset: &Dataset, target_dataset: &Dataset) -> Result<(), GdalError> {
        let reference_raster_count = reference_dataset.raster_count();
        if reference_raster_count < MIN_RASTER_COUNT {
            return Err(GdalError::BadArgument(format!("The reference image does not have enough bands!\nrequired:{MIN_RASTER_COUNT}\nactual:{reference_raster_count}")));
        }

        let target_raster_count = target_dataset.raster_count();
        if target_raster_count < MIN_RASTER_COUNT {
            return Err(GdalError::BadArgument(format!("The target image does not have enough bands!\nrequired:{MIN_RASTER_COUNT}\nactual:{target_raster_count}")));
        }

        let reference_raster_size = reference_dataset.raster_size();
        let target_raster_size = target_dataset.raster_size();

        if reference_raster_size != target_raster_size {
            return Err(GdalError::BadArgument(format!("The size of the target image does not match the size of the reference image!\nreference image size:({},{})\ntarget image size:({},{})",
                    reference_raster_size.0,
                    reference_raster_size.1,
                    target_raster_size.0,
                    target_raster_size.1
            )));
        }

        Ok(())
    }

    fn get_rasters(dataset: Dataset) -> Result<Vec<Buffer<f32>>, GdalError> {
        (1..MIN_RASTER_COUNT + 1)
            .map(|i| dataset.rasterband(i)?.read_band_as::<f32>())
            .collect()
    }
}

impl Classification<u32> for Classifier {
    fn classify(self) -> Result<Buffer<u32>, GdalError> {
        let reference_rasters = Classifier::get_rasters(self.reference)?;
        let target_rasters = Classifier::get_rasters(self.target)?;

        let deltas: Vec<Vec<f32>> = (2..6)
            .map(|i| diff(&target_rasters[i].data, &reference_rasters[i].data))
            .collect();

        let mask = deltas[0]
            .iter()
            .zip(&deltas[1])
            .map(|(&l, &r)| (l > 0.04 && r > 0.04) as u32)
            .collect();

        Ok(Buffer::new(reference_rasters[0].size, mask))
    }
}
