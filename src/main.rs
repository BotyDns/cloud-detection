use clap::{Parser, ValueEnum};
use cloud_detection::classifiers::Classification;
use cloud_detection::comparison;
use confusion_matrix::ConfusionMatrix;
use gdal::errors::GdalError;
use gdal::raster::Buffer;
use gdal::DriverManager;
use std::error::Error;

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
    /// Path to an already classified image. This will be used as reference to verify classification accuracy.
    comparison_image: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Satellites {
    /// Use images made by the Sentinel-2 sensors.
    Sentinel,
    /// Use images made by the Landsat 8-9 sensors.
    Landsat,
}

fn classify(args: &CMDArgs) -> Result<Buffer<u32>, GdalError> {
    match args.satellite {
        Satellites::Landsat => {
            let classifier = landsat::cloud::Classifier::from_path(&args.reference, &args.target)?;
            classifier.classify()
        }
        Satellites::Sentinel => {
            let classifier = sentinel::cloud::Classifier::from_path(&args.reference, &args.target)?;
            classifier.classify()
        }
    }
}

fn create_statistics(
    res_image: &Buffer<u32>,
    classified_image_path: &str,
) -> Result<ConfusionMatrix, GdalError> {
    let classified_image =
        persistence::tif::open_classified_image(&res_image, &classified_image_path)?;
    let matrix = comparison::create_confusion_matrix(&classified_image.data, &res_image.data);
    Ok(matrix)
}

fn main() -> Result<(), Box<dyn Error>> {
    DriverManager::register_all();
    let args = CMDArgs::parse();

    let res_image = match classify(&args) {
        Ok(img) => Ok(img),
        Err(GdalError::NullPointer { method_name: _, msg: _ }) => Err("Could not open files for classification. Reference image path or target image path is incorrect.".to_owned()),
        Err(err) => Err(err.to_string())
    }?;

    if let Some(classified_image_path) = args.comparison_image {
        let matrix = match create_statistics(&res_image, &classified_image_path) {
            Ok(mx) => Ok(mx),
            Err(GdalError::NullPointer {
                method_name: _,
                msg: _,
            }) => Err(
                "Could not open comparison image. The path for the comparison image is incorrect."
                    .to_owned(),
            ),
            Err(err) => Err(err.to_string()),
        }?;

        println!("Overall accuracy: {}", matrix.overall_accuracy());
        println!("false positive rate: {}", matrix.false_rate("1"));
        println!("false negative rate: {}", matrix.false_rate("0"));
        println!("Confusion matrix:");
        println!("{}", matrix);
    }

    persistence::tif::save_classification(&res_image, &args.reference, &args.target);

    println!("Classification successful!");
    Ok(())
}
