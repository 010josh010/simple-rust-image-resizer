use clap::Parser;
use image::{imageops::FilterType, GenericImageView};
use std::fs;
use std::path::Path;


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
    ),
    group(
        clap::ArgGroup::new("input_source")
            .args(&["input", "batch"])
            .required(true)
    )   
)]
struct Cli {
    /// Input image file path
    #[arg(short, long, group = "input_source")]
    input: Option<String>,

    /// Batch directory containing images to resize
    #[arg(short, long, group = "input_source")]
    batch: Option<String>,

    /// Output image file path (optional for single image)
    #[arg(short, long)]
    output: Option<String>,

    /// Output directory for resized images in batch mode (optional)
    #[arg(long)]
    output_dir: Option<String>,

    /// Scale factor for resizing (percentage as a decimal)
    #[arg(short, long, group = "resize_by")]
    scale: Option<f32>,

    /// Dimensions in WIDTHxHEIGHT format (e.g., 800x600)
    #[arg(short, long, group = "resize_by")]
    dimensions: Option<String>,
}

fn main() {
    let args = Cli::parse();

    // Determine new dimensions
    let (scale_factor, target_dimensions) = if let Some(scale) = args.scale {
        (Some(scale), None)
    } else if let Some(dimensions_str) = args.dimensions {
        let dims = parse_dimensions(&dimensions_str);
        (None, Some(dims))
    } else {
        eprintln!("Error: You must provide either --scale or --dimensions.");
        std::process::exit(1);
    };

    if let Some(input_path) = args.input {
        // Single image mode
        process_single_image(&input_path, args.output.as_deref(), scale_factor, target_dimensions);
    } else if let Some(batch_dir) = args.batch {
        // Batch mode
        let output_dir = args.output_dir.unwrap_or_else(|| format!("{}/resized", batch_dir));
        process_batch(&batch_dir, &output_dir, scale_factor, target_dimensions);
    } else {
        // This should not happen due to clap's argument group enforcement
        eprintln!("Error: You must provide either --input or --batch.");
        std::process::exit(1);
    }
}

// Function to process a single image
fn process_single_image(
    input_path: &str,
    output_path: Option<&str>,
    scale_factor: Option<f32>,
    target_dimensions: Option<(u32, u32)>,
) {
    // Open the image file with improved error handling
    let img = match image::open(input_path) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Error: Failed to open the input image file '{}'.", input_path);
            eprintln!("Reason: {}", get_image_error_message(e));
            std::process::exit(1);
        }
    };

    let (new_width, new_height) = calculate_new_dimensions(&img, scale_factor, target_dimensions);

    // Resize the image
    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

    // Determine the output path
    let output_path = output_path.unwrap_or(input_path);

    // Save the resized image with improved error handling
    if let Err(e) = resized_img.save(output_path) {
        eprintln!("Error: Failed to save the resized image to '{}'.", output_path);
        eprintln!("Reason: {}", e);
        std::process::exit(1);
    }

    println!(
        "Image '{}' resized successfully and saved to '{}'.",
        input_path, output_path
    );
}

// Function to process images in batch mode
fn process_batch(
    batch_dir: &str,
    output_dir: &str,
    scale_factor: Option<f32>,
    target_dimensions: Option<(u32, u32)>,
) {
    let input_dir = Path::new(batch_dir);
    let output_dir = Path::new(output_dir);

    // Create the output directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!(
            "Error: Failed to create output directory '{}'.",
            output_dir.display()
        );
        eprintln!("Reason: {}", e);
        std::process::exit(1);
    }

    // Read the directory entries
    let entries = match fs::read_dir(input_dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!(
                "Error: Failed to read the batch directory '{}'.",
                input_dir.display()
            );
            eprintln!("Reason: {}", e);
            std::process::exit(1);
        }
    };

    let mut processed_files = 0;
    let mut skipped_files = 0;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            // Skip if it's not a file
            if !path.is_file() {
                continue;
            }

            // Attempt to open the image file
            match image::open(&path) {
                Ok(img) => {
                    let (new_width, new_height) =
                        calculate_new_dimensions(&img, scale_factor, target_dimensions);

                    // Resize the image
                    let resized_img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

                    // Determine the output file path
                    let file_name = match path.file_name() {
                        Some(name) => name,
                        None => {
                            eprintln!("Warning: Unable to get file name for '{}'. Skipping.", path.display());
                            skipped_files += 1;
                            continue;
                        }
                    };

                    let output_path = output_dir.join(file_name);

                    // Save the resized image
                    if let Err(e) = resized_img.save(&output_path) {
                        eprintln!(
                            "Error: Failed to save the resized image to '{}'. Skipping.",
                            output_path.display()
                        );
                        eprintln!("Reason: {}", e);
                        skipped_files += 1;
                        continue;
                    }

                    println!(
                        "Image '{}' resized successfully and saved to '{}'.",
                        path.display(),
                        output_path.display()
                    );
                    processed_files += 1;
                }
                Err(_) => {
                    // Not an image file or failed to open; skip it
                    eprintln!(
                        "Warning: File '{}' is not a valid image or could not be opened. Skipping.",
                        path.display()
                    );
                    skipped_files += 1;
                }
            }
        }
    }

    println!(
        "Batch processing completed. {} files processed, {} files skipped.",
        processed_files, skipped_files
    );
}

// Function to parse dimensions from a string
fn parse_dimensions(dimensions_str: &str) -> (u32, u32) {
    let dims: Vec<&str> = dimensions_str.split('x').collect();
    if dims.len() != 2 {
        eprintln!("Error: Invalid dimensions format. Use WIDTHxHEIGHT, e.g., 800x600.");
        std::process::exit(1);
    }
    let width = match dims[0].parse::<u32>() {
        Ok(w) => w,
        Err(_) => {
            eprintln!(
                "Error: Invalid width value '{}'. Width must be a positive integer.",
                dims[0]
            );
            std::process::exit(1);
        }
    };
    let height = match dims[1].parse::<u32>() {
        Ok(h) => h,
        Err(_) => {
            eprintln!(
                "Error: Invalid height value '{}'. Height must be a positive integer.",
                dims[1]
            );
            std::process::exit(1);
        }
    };
    (width, height)
}

// Function to calculate new dimensions
fn calculate_new_dimensions(
    img: &image::DynamicImage,
    scale_factor: Option<f32>,
    target_dimensions: Option<(u32, u32)>,
) -> (u32, u32) {
    let (orig_width, orig_height) = img.dimensions();
    if let Some(scale) = scale_factor {
        let scaled_width = (orig_width as f32 * scale) as u32;
        let scaled_height = (orig_height as f32 * scale) as u32;
        (scaled_width, scaled_height)
    } else if let Some((width, height)) = target_dimensions {
        (width, height)
    } else {
        // This should not happen
        eprintln!("Error: Failed to determine new dimensions.");
        std::process::exit(1);
    }
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