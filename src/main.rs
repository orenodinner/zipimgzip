use zipimgzip::unzip_to_memory;
use zipimgzip::ConvMode;

use std::io;
use zipimgzip::PrintMode;
use zipimgzip::SaveFormat;

fn main() -> Result<(), io::Error> {
    singlethread()?;
    multithread()?;
    m2ultithread()?;
    
    return Ok(());
}


fn singlethread() -> Result<(), io::Error> {
    let test_pixels: [u32; 2] = [750, 1334];
    let test_quality: u8 = 90;
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\conv_single.zip");
    let debug_s_time = std::time::Instant::now();

    let _ = unzip_to_memory(test_path, PrintMode::Print)?
        .convert_size(test_pixels[0], test_pixels[1], ConvMode::Height)?
        .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
    let debug_e_time = std::time::Instant::now();
    println!(
        "single_time_{:?}",
        debug_e_time.duration_since(debug_s_time)
    );
    Ok(())
}

fn m2ultithread() -> Result<(), io::Error> {
    let test_pixels: [u32; 2] = [750, 1334];
    let test_quality: u8 = 90;
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\conv_m2.zip");
    let debug_s_time = std::time::Instant::now();

    let _ = unzip_to_memory(test_path, PrintMode::Print)?
        .convert_size_multithread(test_pixels[0], test_pixels[1], ConvMode::Height)?
        .create_zip_multithread(test_outpath, SaveFormat::Ref, test_quality)?;
    let debug_e_time = std::time::Instant::now();
    println!(
        "m2ultithread_time_{:?}",
        debug_e_time.duration_since(debug_s_time)
    );
    Ok(())
}

fn multithread() -> Result<(), io::Error> {
    let test_pixels: [u32; 2] = [750, 1334];
    let test_quality: u8 = 90;
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\conv_m.zip");
    let debug_s_time = std::time::Instant::now();

    let _ = unzip_to_memory(test_path, PrintMode::Print)?
        .convert_size_multithread(test_pixels[0], test_pixels[1], ConvMode::Height)?
        .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
    let debug_e_time = std::time::Instant::now();

    println!(
        "multithread_time_{:?}",
        debug_e_time.duration_since(debug_s_time)
    );

    Ok(())
}
