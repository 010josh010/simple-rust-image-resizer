use clap::Parser;
use image::{imageops::FilterType, GenericImageView};

#[derive(Parser)]
#[command(
    name = "Simple Rust Image Resizer",
    version = "1.0.0",
    author = "Joshua Vaughn <https://github.com/010josh010>",
    about = "Resizes an image file based on dimensions or scale and optionally saves it to a new file.",
    group(
        clap::ArgGroup::new("resize_by")
            .args(&["scale", "dimensions"])
            .required(true)
    )
)]
struct Cli {
    /// Input image file path
    #[arg(short, long)]
    input: String,

    /// Output image file path (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// Scale factor for resizing (percentage as a decimal)
    #[arg(short, long, group = "resize_by")]
    scale: Option<f32>,

    /// Dimensions in WIDTHxHEIGHT format (e.g., 800x600)
    #[arg(short, long, group = "resize_by")]
    dimensions: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // Open the image file
    let img = image::open(&args.input).expect("Failed to open the input image file");
    let (orig_width, orig_height) = img.dimensions();

    // Determine new dimensions
    let (new_width, new_height) = if let Some(scale) = args.scale {
        let scaled_width = (orig_width as f32 * scale) as u32;
        let scaled_height = (orig_height as f32 * scale) as u32;
        (scaled_width, scaled_height)
    } else if let Some(dimensions_str) = args.dimensions {
        // Parse dimensions from WIDTHxHEIGHT format
        let dims: Vec<&str> = dimensions_str.split('x').collect();
        if dims.len() != 2 {
            eprintln!("Invalid dimensions format. Use WIDTHxHEIGHT, e.g., 800x600.");
            std::process::exit(1);
        }
        let width = dims[0].parse::<u32>().expect("Invalid width value");
        let height = dims[1].parse::<u32>().expect("Invalid height value");
        (width, height)
    } else {
        // This should not happen due to clap's argument group enforcement
        eprintln!("You must provide either --scale or --dimensions.");
        std::process::exit(1);
    };

    // Resize the image
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

    // Determine the output path
    let output_path = args.output.unwrap_or_else(|| args.input.clone());

    // Save the resized image
    resized_img
        .save(&output_path)
        .expect("Failed to save the resized image");
}