# RustSlicer 🦀

A high-performance 3D slicer CLI tool written in Rust that converts STL files to G-code for 3D printing.

## Overview

RustSlicer is a command-line 3D slicing tool designed to provide fast, reliable, and efficient conversion of 3D models (STL format) into machine-readable G-code for 3D printers. Built with Rust, it leverages memory safety and performance to handle complex geometries with ease.

## Features

- **High Performance**: Written in Rust for maximum speed and efficiency
- **STL Support**: Import and process STL mesh files
- **G-code Generation**: Output optimized G-code for various 3D printers
- **CLI Interface**: Simple and powerful command-line interface
- **Cross-platform**: Works on Linux, macOS, and Windows

## Installation

### From Source

```bash
git clone https://github.com/1cedsoda/rustslicer.git
cd rustslicer
cargo build --release
```

The compiled binary will be available in `target/release/rustslicer`

### Using Cargo

```bash
cargo install rustslicer
```

## Usage

Basic usage:

```bash
rustslicer input.stl -o output.gcode
```

### Command Line Options

```
rustslicer [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Path to the STL file to slice

Options:
  -o, --output <FILE>           Output G-code file path
  -l, --layer-height <HEIGHT>   Layer height in mm (default: 0.2)
  -i, --infill <PERCENT>        Infill percentage (default: 20)
  -s, --speed <SPEED>           Print speed in mm/s (default: 60)
  -h, --help                    Print help information
  -V, --version                 Print version information
```

### Example

```bash
# Slice a model with custom settings
rustslicer model.stl -o output.gcode --layer-height 0.1 --infill 30 --speed 80
```

## Roadmap

- [ ] Basic STL parsing and mesh validation
- [ ] Layer slicing algorithm
- [ ] Infill pattern generation (rectilinear, honeycomb, gyroid)
- [ ] Support structure generation
- [ ] Multi-material support
- [ ] Configuration profiles for popular printers
- [ ] Web-based preview interface
- [ ] 3MF file format support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
```

## Architecture

RustSlicer is built with a modular architecture:

- **Parser**: STL file parsing and mesh representation
- **Slicer**: Layer generation and path calculation
- **Infill**: Various infill pattern generators
- **G-code Generator**: Converts toolpaths to G-code
- **CLI**: Command-line interface and configuration

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by projects like Slic3r, PrusaSlicer, and Cura
- Built with the amazing Rust ecosystem

## Contact

- GitHub: [@1cedsoda](https://github.com/1cedsoda)
- Issues: [GitHub Issues](https://github.com/1cedsoda/rustslicer/issues)

---

Made with ❤️ and 🦀 by Philipp
