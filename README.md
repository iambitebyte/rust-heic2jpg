# rust-heic2jpg
Batch convert HEIC files to JPEG format.

This tool allows you to batch convert HEIC (High Efficiency Image Coding) files to JPEG format using Rust. It leverages the `libheif_rs` library to read HEIC files and convert them to standard JPEG images.

## Install
This project uses the `libheif_rs` library to handle HEIC files. For more information about `libheif_rs`, you can refer to the official crate page: [libheif-rs on crates.io](https://crates.io/crates/libheif-rs).

On macOS, you can install `libheif` via Homebrew:
```bash
brew install libheif
```

### Prerequisites
Make sure you have Rust and Cargo installed on your machine. If you don’t have them, you can install them by following the instructions here: [Rust Installation](https://www.rust-lang.org/tools/install).

To build and run the project, simply clone the repository:

```bash
git clone git@github.com:iambitebyte/rust-heic2jpg.git
cd rust-heic2jpg
cargo build --release

cd target/release
./rust-heic2jpg {input folder} {output folder}

```