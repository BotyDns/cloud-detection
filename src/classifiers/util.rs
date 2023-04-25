//! This module provides utilities for for different masking tasks

use std::ops::Sub;

use gdal::{
    errors::GdalError,
    raster::{Buffer, GdalType},
    Dataset,
};

/// Calculates the difference between the elements of two vectors and returns a new vector with the difference
pub fn diff<T: Sub<Output = T> + Copy>(lhs: &Vec<T>, rhs: &Vec<T>) -> Vec<T> {
    lhs.iter().zip(rhs).map(|(&l, &r)| l - r).collect()
}

/// Validates the reference and target datasets.
pub fn validate(
    reference_dataset: &Dataset,
    target_dataset: &Dataset,
    min_raster_count: isize,
) -> Result<(), GdalError> {
    let reference_raster_count = reference_dataset.raster_count();
    if reference_raster_count < min_raster_count {
        return Err(GdalError::BadArgument(format!("The reference image does not have enough bands!\nrequired:{min_raster_count}\nactual:{reference_raster_count}")));
    }

    let target_raster_count = target_dataset.raster_count();
    if target_raster_count < min_raster_count {
        return Err(GdalError::BadArgument(format!("The target image does not have enough bands!\nrequired:{min_raster_count}\nactual:{target_raster_count}")));
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

/// Extracts the required rasters from the dataset
pub fn get_rasters<T: Copy + GdalType>(
    dataset: Dataset,
    indices: &[isize],
) -> Result<Vec<Buffer<T>>, GdalError> {
    indices
        .iter()
        .map(|&i| dataset.rasterband(i)?.read_band_as::<T>())
        .collect()
}
