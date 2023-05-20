# cloud-detection

The purpose of this project is to reproduce the MCM algorithm provided in this [paper](https://isprs-archives.copernicus.org/articles/XLI-B2/95/2016/isprs-archives-XLI-B2-95-2016.pdf)

The program is able to currently mask thick clouds using landsat 8-9 images.

## Dependencies (Windows)

To install dependencies manually, follow this description.

This project uses GDAL as a dependency. To build this project, GDAL 3.5 libraries must be installed.
This project was built and tested with GDAL 3.5.1 using the following release:
curl.exe -S -O https://download.gisinternals.com/sdk/downloads/release-1928-x64-dev.zip (for Windows)

The following paths need to be set:
GDAL_HOME=$PWD/release-1928-x64
PROJ_LIB=$GDAL_HOME/bin/proj7/share
PATH=$GDAL_HOME/bin:$GDAL_HOME/bin/gdal/apps:$PATH
also, for a weird reason you need to add the GDAL_VERSION to your variables as well, containing the version of gdal

## Usage

The current implementation supports the `Float32` Tiff format.  
For cloud masking the images should be merged and should contain the following bands (in the given order, also):

1. Doesn't matter
2. Doesn't matter
3. Green
4. Blue

In the current implementation it is only important that the green band should land on band 3 and the blue band should land on band 4. The other bands are not used in the algorithm.

### Executing the binary file

Here's an example on how to run the executable:  
`./cloud-detection.exe landsat --reference reference.tif --target target.tif`

For more help, use `./cloud-detection.exe --help`

**Currently supported satellites:**

- `landsat`: creates a cloud mask using images from the Landsat 8-9 sensors
- (EXPERIMENTAL) `sentinel`: creates a cloud mask using images from the Sentinel-2 sensors

## Documentation

This crate's API Documentation can be compiled and opened using `cargo` and `rustdoc` with the following command:  
`cargo doc --open`
