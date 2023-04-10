use clap::Parser;
use cloud_detection::classifiers::Classification;
use gdal::DriverManager;
use std::path::Path;

use cloud_detection::classifiers::mcm::landsat;
use cloud_detection::persistence;

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

    println!("reference image path: {}", args.reference);
    println!("target image path: {}", args.target);

    let classifier = landsat::cloud::Classifier::from_path(&args.reference, &args.target).unwrap();
    let res_image = classifier.classify().unwrap();

    let target_path = Path::new(&args.target);
    let parent_path = target_path.parent().unwrap();
    let result_path = parent_path.join("result.tif");

    println!("Classification successful!");
    println!("Classified image path: {}", result_path.to_str().unwrap());
    persistence::tif::save(&args.reference, result_path.to_str().unwrap(), &res_image);
}
