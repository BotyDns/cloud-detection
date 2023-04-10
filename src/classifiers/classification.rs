use gdal::errors::GdalError;
use gdal::raster::{Buffer, GdalType};

pub trait Classification<T: GdalType> {
    fn classify(self) -> Result<Buffer<T>, GdalError>;
}
