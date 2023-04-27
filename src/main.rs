use clap::{Parser, ValueEnum};
use cloud_detection::classifiers::Classification;
use gdal::raster::GdalType;
use gdal::DriverManager;
use std::path::Path;

use cloud_detection::classifiers::mcm::landsat;
use cloud_detection::classifiers::mcm::sentinel;
use cloud_detection::persistence;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
/// A command line tool to detect clouds and cloud-shadows
struct CMDArgs {
    #[arg(value_enum)]
    /// The satellite from which the image was created
    satellite: Satellites,

    #[arg(short, long)]
    /// Path to the reference image
    reference: String,

    #[arg(short, long)]
    /// Path to the target image
    target: String,

    #[arg(short, long)]
    /// Path to an already classified image, for confusion matrix comparison
    comparison_image: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Satellites {
    /// Use images made by the Sentinel-2 sensors.
    Sentinel,
    /// Use images made by the Landsat 8-9 sensors.
    Landsat,
}

fn classify_and_save<C, T>(classifier: C, reference_path: &str, target_path: &str)
where
    C: Classification<T>,
    T: Copy + GdalType,
{
    let res_image = classifier.classify().unwrap();

    let target_path_obj = Path::new(target_path);
    let parent_path = target_path_obj.parent().unwrap();
    let result_path = parent_path.join("result.tif");

    persistence::tif::save(reference_path, result_path.to_str().unwrap(), &res_image);

    println!("Classification successful!");
    println!("Classified image path: {}", result_path.to_str().unwrap());
}

fn main() {
    DriverManager::register_all();
    let args = CMDArgs::parse();

    println!("reference image path: {}", args.reference);
    println!("target image path: {}", args.target);

    match args.satellite {
        Satellites::Landsat => {
            let classifier =
                landsat::cloud::Classifier::from_path(&args.reference, &args.target).unwrap();
            classify_and_save(classifier, &args.reference, &args.target)
        }
        Satellites::Sentinel => {
            let classifier =
                sentinel::cloud::Classifier::from_path(&args.reference, &args.target).unwrap();
            classify_and_save(classifier, &args.reference, &args.target)
        }
    };

    
}
