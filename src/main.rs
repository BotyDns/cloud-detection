use clap::Parser;
use gdal::{raster::Buffer, DriverManager};
use std::path::Path;

mod classifiers;
mod persistence;

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
    classifier.add_image(&args.target).unwrap();

    let res_images = classifier
        .classify(classifiers::ClassificationType::Cloud)
        .unwrap();

    for img in res_images.1 {
        let buffer = Buffer::<u32>::new(res_images.0, img);
        let target_path = Path::new(&args.target);
        let parent_path = target_path.parent().unwrap();
        let result_path = parent_path.join("test.tif");

        println!("{}", result_path.to_str().unwrap());

        persistence::save_tif(&args.reference, result_path.to_str().unwrap(), &buffer);
    }

    // open_image("E:/Programozas/terinfo/data/landsat_8-9/test/2023-02-10-00_00_2023-02-10-23_59_Landsat_8-9_L2_B05_(Raw).tiff");
}
