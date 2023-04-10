use gdal::{
    raster::{Buffer, GdalType},
    Dataset,
};

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
