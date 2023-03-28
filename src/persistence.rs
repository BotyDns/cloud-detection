use gdal::{
    errors::GdalError,
    raster::{Buffer, GdalType, RasterBand},
    Dataset,
};

pub fn save_tiff<T>(img_path: &str, buffer: &Buffer<T>)
where
    T: Copy + GdalType,
{
    let driver = gdal::DriverManager::get_driver_by_name("GTiff").unwrap();
    let buffer_size: (isize, isize) = (
        buffer.size.0.try_into().unwrap(),
        buffer.size.1.try_into().unwrap(),
    );
    let mut dataset = driver
        .create_with_band_type::<T, &str>(
            img_path,
            buffer.size.0.try_into().unwrap(),
            buffer.size.1.try_into().unwrap(),
            1,
        )
        .unwrap();

    let mut band = dataset.rasterband(1).unwrap();
    band.write(buffer_size, buffer.size, &buffer).unwrap();
    band.set_no_data_value(Some(0.0)).unwrap();
    dataset.flush_cache();
}
