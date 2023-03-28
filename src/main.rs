use clap::Parser;
use gdal::{errors::GdalError, Dataset, DriverManager};

use crate::classifiers::MCMClassifier;

mod classifiers;
mod persistence;

fn process_layer_free(layer: gdal::raster::RasterBand) {
    let minmax = layer.compute_raster_min_max(false);
    match minmax {
        Ok(res) => println!("min:{} max:{}", res.min, res.max),
        Err(err) => panic!("{}", err.to_string()),
    };
}

fn process_layer(layer: Result<gdal::raster::RasterBand, gdal::errors::GdalError>) {
    match layer {
        Ok(res) => process_layer_free(res),
        Err(err) => panic!("{}", err.to_string()),
    };
}

fn open_image(path: &str) {
    let result = Dataset::open(path);

    match result {
        Ok(res) => process_layer(res.rasterband(1)),
        Err(err) => panic!("{}", err.to_string()),
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A command line tool to detect multitemporal cloud and cloud-shadow masks.
struct CMDArgs {
    /// Path to the reference image
    #[arg(short, long)]
    reference: String,

    /// Path to the target image
    #[arg(short, long)]
    target: String,
}

fn main() {
    DriverManager::register_all();
    let args = CMDArgs::parse();

    println!("reference: {}", args.reference);
    println!("target: {}", args.target);

    let mut classifier = classifiers::new_mcmclassifier(&args.reference).unwrap();
    //classifier.add_image(&args.target).unwrap();

    let res_image = classifier
        .classify::<f32>(classifiers::ClassificationType::Cloud)
        .unwrap();

    for i in 100..1000 {
        print!("{} ", res_image.data[i])
    }

    // open_image("E:/Programozas/terinfo/data/landsat_8-9/test/2023-02-10-00_00_2023-02-10-23_59_Landsat_8-9_L2_B05_(Raw).tiff");
}
