# simple-rust-image-resizer
Resizes an image file and optionally saves it to a new file

## Supported Image Formats
The program supports various image formats, including:

- **PNG**
- **JPEG**
- **GIF**
- **BMP**
- **TIFF**

## Tips
- **Maintaining Aspect Ratio**: If you want to resize the image while maintaining its aspect ratio, use the `--scale` option.
- **Overwriting Original File**: Be cautious when omitting the `-o` option, as this will overwrite your original image file.
- **Image Quality**: The program uses the `Lanczos3` filter for high-quality resizing. You can modify the filter type in the source code if desired.

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

#### Command-Line Arguments:
*    -d, --dimensions \<DIMENSIONS\>    Dimensions in WIDTHxHEIGHT format (e.g., 800x600)
*    -h, --help                         Print help information
*    -i, --input \<INPUT\>              Input image file path
*    -o, --output \<OUTPUT\>            Output image file path (optional)
*    -s, --scale \<SCALE\>              Scale factor for resizing (percentage as a decimal)
*    -V, --version                      Print version information

- **Required Arguments**: You must provide either the `--scale` option or `--dimensions` *Note these arguments are mutually exclusive.
- **Scale Factor**: The scale factor should be a positive decimal number less than or equal to 1.0 (e.g., `0.5` for 50% scaling).
- **Dimensions**: Specified in the the format `WIDTHxHEIGHT`. These values should be the values in pixels that you want to resize the image to. (e.g., `1024x768`)

#### Examples:
- image-resizer --input \<INPUT\> [OPTIONS] --scale \<SCALE\>
- image-resizer --input \<INPUT\> [OPTIONS] --dimensions \<DIMENSIONS\>

Resize an image to 50% of its original size and save it as `output.jpg`.

```bash
./image_resizer -i input.jpg -o output.jpg --scale 0.5
```

Resize an image to 25% of its original size and overwrite the original file.
```bash
./image_resizer -i input.jpg --s 0.25
```

Resize an image to 1024x768 pixels and save it as `resized.jpg`.
```bash
./image_resizer -i input.jpg -o resized.jpg --dimensions 1024x768
```

Resize an image to 640x480 pixels, overwriting the original file.
```bash
./image_resizer -i input.jpg --d 640x480
```

## Version History
See [release history](https://github.com/010josh010/simple-rust-image-resizer/releases)

