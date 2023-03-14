# cloud-detection

## Dependencies

See the next chapters to see options for downloading dependencies. If you would like to install dependencies manually, follow this description.

This project uses GDAL as a dependency. To build this project, GDAL 3.5 libraries must be installed.
This project was built and tested with GDAL 3.5.1 using the following release:
curl.exe -S -O https://download.gisinternals.com/sdk/downloads/release-1928-x64-dev.zip (for Windows)

The following paths need to be set:
GDAL_HOME=$PWD/release-1928-x64
PROJ_LIB=$GDAL_HOME/bin/proj7/share
PATH=$GDAL_HOME/bin:$GDAL_HOME/bin/gdal/apps:$PATH
also, for a weird reason you need to add the GDAL_VERSION to your variables as well, containing the version of gdal
