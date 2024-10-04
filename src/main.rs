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

    // Open the image file with improved error handling
    let img = match image::open(&args.input) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error: Failed to open the input image file.");
            eprintln!("Reason: {}", get_image_error_message(e));
            std::process::exit(1);
        }
    };

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
            eprintln!("Error: Invalid dimensions format. Use WIDTHxHEIGHT, e.g., 800x600.");
            std::process::exit(1);
        }
        let width = match dims[0].parse::<u32>() {
            Ok(w) => w,
            Err(_) => {
                eprintln!("Error: Invalid width value '{}'. Width must be a positive integer.", dims[0]);
                std::process::exit(1);
            }
        };
        let height = match dims[1].parse::<u32>() {
            Ok(h) => h,
            Err(_) => {
                eprintln!("Error: Invalid height value '{}'. Height must be a positive integer.", dims[1]);
                std::process::exit(1);
            }
        };
        (width, height)
    } else {
        // This should not happen due to clap's argument group enforcement
        eprintln!("Error: You must provide either --scale or --dimensions.");
        std::process::exit(1);
    };

    // Resize the image
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

    // Determine the output path
    let output_path = args.output.unwrap_or_else(|| args.input.clone());

    // Save the resized image with improved error handling
    if let Err(e) = resized_img.save(&output_path) {
        eprintln!("Error: Failed to save the resized image.");
        eprintln!("Reason: {}", e);
        std::process::exit(1);
    }

    println!("Image resized successfully and saved to '{}'.", output_path);
}

// Helper function to get a user-friendly error message
fn get_image_error_message(error: image::ImageError) -> String {
    match error {
        image::ImageError::IoError(io_err) => format!("I/O error: {}", io_err),
        image::ImageError::Unsupported(err) => format!("Unsupported image format: {}", err),
        image::ImageError::Decoding(err) => format!("Decoding error: {}", err),
        image::ImageError::Encoding(err) => format!("Encoding error: {}", err),
        _ => format!("An unknown error occurred: {}", error),
    }
}