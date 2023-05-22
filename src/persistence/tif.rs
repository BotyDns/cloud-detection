//! This module provides tools for saving and loading tif images.

use std::path::Path;

use gdal::{
    errors::GdalError,
    raster::{Buffer, GdalType},
    Dataset,
};

/// Saves the given buffer to the given path. Uses another images as reference to configure the output image.
pub fn save<T>(reference_img_path: &str, img_path: &str, buffer: &Buffer<T>)
where
    T: Copy + GdalType,
{
    let reference_dataset = Dataset::open(reference_img_path).unwrap();

    let driver = gdal::DriverManager::get_driver_by_name("GTiff").unwrap();
    let buffer_size: (isize, isize) = (
        buffer.size.0.try_into().unwrap(),
        buffer.size.1.try_into().unwrap(),
    );
    let mut dataset = driver
        .create_with_band_type::<T, &str>(img_path, buffer_size.0, buffer_size.1, 1)
        .unwrap();

    let geo_transform = reference_dataset.geo_transform().unwrap();
    let projection = reference_dataset.projection();
    dataset.set_geo_transform(&geo_transform).unwrap();
    dataset.set_projection(&projection).unwrap();

    let mut band = dataset.rasterband(1).unwrap();
    band.write((0, 0), buffer.size, &buffer).unwrap();
    dataset.flush_cache();
}

/// opens and validates an already classified image.
///  
/// **Parameters**:
/// - current_classification: the image that we just classified (it is used to validate the loaded image).
/// - reference_classification_path: the image that we want to load.
/// # Examples:
/// ```no_run
/// use cloud_detection::persistence;
/// use cloud_detection::classifiers::mcm::landsat;
/// use cloud_detection::classifiers::Classification;
/// use crate::persistence::config::CloudDetectionConfig;
///
/// fn main() {
///     let classifier = landsat::cloud::Classifier::from_path("./reference.tif", "./target.tif", &CloudDetectionConfig::default()).unwrap();
///     let result = classifier.classify().unwrap();
///
///     let validation_image = persistence::tif::open_classified_image(&result, "./validation_image.tif").unwrap();
/// }
///
/// ```
pub fn open_classified_image(
    current_classification: &Buffer<u32>,
    reference_classification_path: &str,
) -> Result<Buffer<u32>, GdalError> {
    let reference_dataset = Dataset::open(reference_classification_path)?;

    let reference_raster_count = reference_dataset.raster_count();
    if reference_raster_count != 1 {
        return Err(GdalError::BadArgument(format!(
            "The classified image should have exactly one band!"
        )));
    }

    let reference_raster_size = reference_dataset.raster_size();

    if reference_raster_size != current_classification.size {
        return Err(GdalError::BadArgument(format!("The size of the reference image does not match the size of the classified image!\nclassified image size:({},{})\nreference image size:({},{})",
        reference_raster_size.0,
        reference_raster_size.1,
        current_classification.size.0,
        current_classification.size.1,
    )));
    }

    Ok(reference_dataset.rasterband(1)?.read_band_as::<u32>()?)
}

/// Saves the classified image to the given target path.
/// A reference image path should be given as well to properly configure the output image.
/// # Examples
/// ## Extract a band from a tif file and save it to a separate file.
/// ```no_run
///
/// use cloud_detection::persistence;
/// use cloud_detection::classifiers::mcm::landsat;
/// use cloud_detection::classifiers::Classification;
/// use crate::persistence::config::CloudDetectionConfig;
///
/// fn main() {
///     let classifier = landsat::cloud::Classifier::from_path("./reference.tif", "./target.tif", &CloudDetectionConfig::default()).unwrap();
///     let result = classifier.classify().unwrap();
///
///     persistence::tif::save_classification(&result, "./reference.tif", "output.tif");
/// }
/// ```
pub fn save_classification<T>(classified_image: &Buffer<T>, reference_path: &str, output_path: &str)
where
    T: Copy + GdalType,
{
    let output_path_obj = Path::new(output_path);
    let parent_path = output_path_obj.parent().unwrap();
    let result_path = parent_path.join("result.tif");

    save(
        reference_path,
        result_path.to_str().unwrap(),
        classified_image,
    );

    println!("Classified image path: {}", result_path.to_str().unwrap());
}
