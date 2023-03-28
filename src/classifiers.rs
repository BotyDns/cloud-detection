use gdal::{
    errors::GdalError,
    raster::{Buffer, GdalType, RasterBand},
    Dataset,
};

use std::collections::HashMap;

pub enum ClassificationType {
    None = 0,
    Cloud = 1,
    CloudShadow = 2,
}

pub fn new_mcmclassifier(reference_image_path: &str) -> Result<MCMClassifier, GdalError> {
    let reference_image = Dataset::open(reference_image_path);

    Ok(MCMClassifier {
        targets: HashMap::new(),
        reference: (reference_image_path.to_string(), reference_image?),
    })
}

impl MCMClassifier {
    pub fn add_image(&mut self, path: &str) -> Result<Option<Dataset>, GdalError> {
        let image = Dataset::open(path);
        Ok(self.targets.insert(path.to_string(), image?))
    }

    pub fn classify<T>(&self, scope: ClassificationType) -> Result<Buffer<T>, GdalError>
    where
        T: Copy + GdalType,
    {
        let ref_data = &self.reference.1;
        let ref_layer = ref_data.rasterband(1)?;

        let ref_buffer = ref_layer.read_band_as::<T>()?;

        Ok(ref_buffer)
    }
}

pub struct MCMClassifier {
    targets: HashMap<String, Dataset>,
    reference: (String, Dataset),
}
