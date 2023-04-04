use gdal::{
    errors::GdalError,
    raster::{Buffer, GdalType, RasterBand},
    Dataset,
};

use std::{collections::HashMap, ops::Sub};

pub struct MCMClassifier {
    targets: HashMap<String, Dataset>,
    reference: (String, Dataset),
}

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

fn calculate_diff(lhs: Vec<f32>, rhs: Vec<f32>) -> Vec<f32> {
    lhs.iter().zip(&rhs).map(|(l, r)| l - r).collect()
}

impl MCMClassifier {
    pub fn add_image(&mut self, path: &str) -> Result<Option<Dataset>, GdalError> {
        let image = Dataset::open(path);
        Ok(self.targets.insert(path.to_string(), image?))
    }

    pub fn classify(
        &self,
        scope: ClassificationType,
    ) -> Result<((usize, usize), Vec<Vec<u32>>), GdalError> {
        match scope {
            ClassificationType::Cloud => self.classify_cloud(),
            _ => Err(GdalError::BadArgument(
                ("This method was not yet implemented!".to_string()),
            )),
        }
    }

    fn classify_cloud(&self) -> Result<((usize, usize), Vec<Vec<u32>>), GdalError> {
        let ref_data = &self.reference.1;
        let band_count = ref_data.raster_count();

        if band_count < 11 {
            return Err(GdalError::BadArgument(
                "There are not enough bands in the image!".to_string(),
            ));
        }

        let mut ref_bands = Vec::new();
        for i in 1..(band_count + 1) {
            ref_bands.push(ref_data.rasterband(i)?.read_band_as::<f32>()?);
        }

        let mut masks = Vec::new();

        for target in &self.targets {
            let target_data = target.1;

            let mut target_bands = Vec::new();
            for i in 1..(band_count + 1) {
                target_bands.push(target_data.rasterband(i)?.read_band_as::<f32>()?);
            }

            let mut deltas: [Vec<f32>; 4];

            deltas[0] = calculate_diff(target_bands[2].data, ref_bands[2].data);
            deltas[1] = calculate_diff(target_bands[3].data, ref_bands[3].data);
            deltas[2] = calculate_diff(target_bands[4].data, ref_bands[4].data);
            deltas[3] = calculate_diff(target_bands[5].data, ref_bands[6].data);

            let mut mask = Vec::new();
            mask.resize(ref_bands[0].data.len(), 0);

            for i in 0..mask.len() {
                mask[i] = (deltas[0][i] > 0.04 && deltas[1][i] > 0.04) as u32;
            }
            masks.push(mask);
        }

        Ok((ref_bands[0].size, masks))
    }
}
