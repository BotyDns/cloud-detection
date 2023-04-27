use clap::{Parser, ValueEnum};
use cloud_detection::classifiers::Classification;
use cloud_detection::comparison;
use gdal::raster::Buffer;
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

fn save<T>(classified_image: &Buffer<T>, reference_path: &str, target_path: &str)
where
    T: Copy + GdalType,
{
    let target_path_obj = Path::new(target_path);
    let parent_path = target_path_obj.parent().unwrap();
    let result_path = parent_path.join("result.tif");

    persistence::tif::save(
        reference_path,
        result_path.to_str().unwrap(),
        classified_image,
    );

    println!("Classified image path: {}", result_path.to_str().unwrap());
}

fn main() {
    DriverManager::register_all();
    let args = CMDArgs::parse();

    println!("reference image path: {}", args.reference);
    println!("target image path: {}", args.target);

    let res_image = match args.satellite {
        Satellites::Landsat => {
            let classifier =
                landsat::cloud::Classifier::from_path(&args.reference, &args.target).unwrap();
            classifier.classify().unwrap()
        }
        Satellites::Sentinel => {
            let classifier =
                sentinel::cloud::Classifier::from_path(&args.reference, &args.target).unwrap();
            classifier.classify().unwrap()
        }
    };

    if let Some(classified_image_path) = args.comparison_image {
        let classified_image =
            persistence::tif::open_classified_image(&res_image, &classified_image_path).unwrap();
        let matrix = comparison::create_confusion_matrix(&res_image.data, &classified_image.data);

        println!("Overall accuracy: {}", matrix.overall_accuracy());
        println!("Confusion matrix:");
        println!("{}", matrix);
    }

    save(&res_image, &args.reference, &args.target);

    println!("Classification successful!");
}
