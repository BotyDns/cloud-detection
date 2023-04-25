use gdal::errors::GdalError;
use gdal::raster::{Buffer, GdalType};

/// This trait provides an abstraction over the classifiers implemented in this application.
pub trait Classification<T: GdalType> {
    fn classify(self) -> Result<Buffer<T>, GdalError>;
}
