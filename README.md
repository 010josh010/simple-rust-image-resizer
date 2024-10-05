# simple-rust-image-resizer
The Simple Rust Image Resizer is a command-line tool that efficiently resizes images individually or in batches.

## Supported Image Formats
The program supports various image formats, including:

- **PNG**
- **JPEG**
- **GIF**
- **BMP**
- **TIFF**

## Getting Started

### Installing
* Install Rust from https://www.rust-lang.org/tools/install
* Clone the repo or download and extract the zip file
* Open a terminal in the project's root directory
* Run the following command from the terminal to build
```console
cargo build --bin image-resizer
```
- A `target/debug` folder should now be placed in your projects root directory containing the generated binary.
- Add the executable to your PATH to call the program from anywhere.

### Usage
    image-resizer --input <INPUT> [OPTIONS] --scale <SCALE>
    image-resizer --input <INPUT> [OPTIONS] --dimensions <DIMENSIONS>
    image-resizer --batch <BATCH> [OPTIONS] --scale <SCALE>
    image-resizer --batch <BATCH> [OPTIONS] --dimensions <DIMENSIONS>
    
#### Command-Line Arguments:
*    -b, --batch \<BATCH\>                Batch directory containing images to resize
*    -d, --dimensions \<DIMENSIONS\>    Dimensions in WIDTHxHEIGHT format (e.g., 800x600)
*    -h, --help                         Print help information
*    -i, --input \<INPUT\>              Input image file path
*    -o, --output \<OUTPUT\>              Output image file path (optional for single image)
*   --output-dir \<OUTPUT_DIR\>      Output directory for resized images in batch mode (optional)
*    -s, --scale \<SCALE\>              Scale factor for resizing (percentage as a decimal)
*    -V, --version                      Print version information


#### Important Notes Please Read *****
- **Overwriting Original File**: Be cautious when omitting the `-o` option, as this will overwrite your original image file.

- **Maintaining Aspect Ratio**: If you want to resize the image while maintaining its aspect ratio, use the `--scale` option.

- **Image Quality**: The program uses the `Lanczos3` filter for high-quality resizing. You can modify the filter type in the source code if desired.

- **Batch Processing**: In order to avoid overwriting the original images and to keep the resized images organized, the program will save the resized images into a subdirectory named `resized` within the specified batch directory. Alternatively, you can specify an output directory using a new `--output-dir` option. 
#### Examples:

Resize an image to 25% of its original size and overwrite the original file.
```bash
./image_resizer -i input.jpg -s 0.25
```

Resize an image to 1024x768 pixels and save it as `resized.jpg`.
```bash
./image_resizer -i input.jpg -o resized.jpg -d 1024x768
```

Resize all images in a directory to 10% of their original size.
```bash
./image-resizer --batch /path/to/image_directory --scale 0.1
```

Resize images to 350x350 pixels and save them to a specific output directory.
```bash
./image-resizer --batch /path/to/image_directory --output-dir /path/to/output_directory --dimensions 350x350
```

## Version History
See [release history](https://github.com/010josh010/simple-rust-image-resizer/releases)

