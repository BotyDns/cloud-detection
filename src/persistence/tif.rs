use std::thread::current;

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

    if (reference_raster_size != current_classification.size) {
        return Err(GdalError::BadArgument(format!("The size of the reference image does not match the size of the classified image!\nclassified image size:({},{})\nreferenc image size:({},{})",
        reference_raster_size.0,
        reference_raster_size.1,
        current_classification.size.0,
        current_classification.size.1,
    )));
    }

    Ok(reference_dataset.rasterband(1)?.read_band_as::<u32>()?)
}
