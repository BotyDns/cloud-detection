use gdal::{Dataset, DriverManager};

fn open_image(path: &str) {
    let result = Dataset::open(path);
    match result {
        Ok(res) => println!("{} {}", res.raster_size().0, res.raster_size().1),
        Err(err) => panic!("{}", err.to_string()),
    }
}

fn main() {
    DriverManager::register_all();

    open_image("E:/Programozas/terinfo/data/landsat_8-9/test/2023-02-10-00_00_2023-02-10-23_59_Landsat_8-9_L2_B05_(Raw).tiff");
}
