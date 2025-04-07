# BlurHash Generator

A high-performance Rust tool for generating BlurHash strings from a folder of images. BlurHash is a compact representation of a placeholder for an image, which can be used to show a blurred preview of an image while the full image is loading.

## Features

- 🚀 High-performance parallel processing using Rayon
- 📁 Processes entire directories of images
- 🖼️ Supports JPG, JPEG, and PNG formats
- 📊 Progress tracking and logging
- 💾 Outputs results in JSON format
- 🧠 Memory-efficient chunked processing
- ⚡ Automatic file extension correction

## Installation

### Prerequisites

- Rust and Cargo (Rust's package manager)
- A folder containing images to process

### Building from Source

```bash
git clone https://github.com/seoulcomix/blurhash-generator.git
cd blurhash-generator
cargo build --release
```

## Usage

```bash
./target/release/blurhash-generator <folder_path> <output_path> [--sample <number>] [--chunk <number>]
```

### Arguments

- `folder_path`: Path to the directory containing images
- `output_path`: Path where the JSON results will be saved
- `--sample <number>`: (Optional) Process only the first N images
- `--chunk <number>`: (Optional) Process images in chunks of specified size (default: 100)

### Example

```bash
./target/release/blurhash-generator ./images ./results.json --chunk 50
```

## Output Format

The tool generates a JSON file with the following structure:

```json
{
  "results": [
    {
      "file": "path/to/image.jpg",
      "blurhash": "L6PZfSi_.AyE_3t7t7R**0o#DgR4"
    },
    // ... more results
  ]
}
```

## Performance Considerations

- The tool automatically skips very large images (>10 million pixels) to prevent memory issues
- Processing is done in parallel using Rayon for optimal performance
- Memory usage is managed by processing images in configurable chunks

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [BlurHash](https://github.com/woltapp/blurhash) - The original BlurHash implementation
- [image-rs](https://github.com/image-rs/image) - Rust image processing library
- [Rayon](https://github.com/rayon-rs/rayon) - Data parallelism library for Rust
