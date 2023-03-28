use clap::Parser;
use gdal::DriverManager;

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
    //classifier.add_image(&args.target).unwrap();

    let res_image = classifier
        .classify::<f32>(classifiers::ClassificationType::Cloud)
        .unwrap();

    persistence::save_tiff(
        &args.reference,
        "E:/Programozas/terinfo/data/sentinel_2/not_cloudy/2023-03-20/test.tif",
        &res_image,
    );

    // open_image("E:/Programozas/terinfo/data/landsat_8-9/test/2023-02-10-00_00_2023-02-10-23_59_Landsat_8-9_L2_B05_(Raw).tiff");
}
