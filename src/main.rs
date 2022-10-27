use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use zipimgzip::unzip_to_memory;
use zipimgzip::ConvMode;

use std::io;
use zipimgzip::PrintMode;
use zipimgzip::SaveFormat;

fn main() -> Result<(), io::Error> {
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\conv.zip");
    let test_pixels: [u32; 2] = [750, 1334];
    let test_quality: u8 = 90;
    let debug_s_time = std::time::Instant::now();

    let _ = unzip_to_memory(test_path, PrintMode::Print)?
        .convert_size(test_pixels[0], test_pixels[1], ConvMode::Height)?
        .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
    let debug_e_time = std::time::Instant::now();
    println!(
        "single_time_{:?}",
        debug_e_time.duration_since(debug_s_time)
    );

    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\conv.zip");

    let debug_s_time = std::time::Instant::now();
    let _ = unzip_to_memory(test_path, PrintMode::Print)?
        .convert_size_multiprocess(test_pixels[0], test_pixels[1], ConvMode::Height)?
        .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
    let debug_e_time = std::time::Instant::now();
    println!(
        "multiprocess_time_{:?}",
        debug_e_time.duration_since(debug_s_time)
    );

    return Ok(());
}
