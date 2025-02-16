use std::{fs, path::Path, env, time::Instant};
use libheif_rs::{RgbChroma, ColorSpace, HeifContext, LibHeif};
use image::{ImageBuffer, Rgb};

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Missing parameters: heic2jpg <input directory> <output directory>");
    }

    let input_directory = &args[1];
    let output_directory = &args[2];

    let lib_heif = LibHeif::new();
    let output_path = Path::new(output_directory);
    get_heic_files(input_directory).iter().for_each(|file| convert_heic_to_jpeg(file.as_str(), &output_path, &lib_heif));

    println!("Conversion completed in: {:?}", start_time.elapsed());
}

fn convert_heic_to_jpeg(input_file: &str, output_directory: &Path, lib_heif: &LibHeif) {
    if !input_file.to_lowercase().ends_with(".heic") {
        println!("Skipping non-HEIC file: {}", input_file);
        return;
    }

    let context = HeifContext::read_from_file(input_file).unwrap();
    let image_handle = context.primary_image_handle().unwrap();
    let image_data = lib_heif.decode(
        &image_handle,
        ColorSpace::Rgb(RgbChroma::Rgb),
        None,
    ).unwrap();

    let width = image_handle.width() as usize;
    let height = image_handle.height() as usize;
    let rgb_data: &[u8] = image_data.planes().interleaved.unwrap().data;
    let mut image_buffer = ImageBuffer::new(width as u32, height as u32);
    
    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * 3;
            let r = rgb_data[offset];
            let g = rgb_data[offset + 1];
            let b = rgb_data[offset + 2];
            image_buffer.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }
    
    let output_file = output_directory.join(Path::new(input_file).file_stem().unwrap()).with_extension("jpg");    
    println!("JPEG file saved: {:?}", output_file);
    image_buffer.save(output_file).unwrap();
}

fn get_heic_files(directory_path: &str) -> Vec<String> {
    fs::read_dir(directory_path)
        .unwrap_or_else(|_| { panic!("Failed to read directory: {}", directory_path); })
        .filter_map(|entry| {
            let entry = entry.unwrap_or_else(|_| { panic!("Failed to read directory entry."); });
            if entry.file_type().unwrap().is_file() {
                Some(entry.path().to_str().unwrap().to_string())
            } else {
                None
            }
        }).collect()
}
