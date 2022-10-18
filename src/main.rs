
use zipimgzip::MemoryImages;
use zipimgzip::PrintMode;
use zipimgzip::ConvMode;
use zipimgzip::unzip_to_memory;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};


fn main() {
    println!("Hello, world!");
    let test_path = String::from("C:\\temp\\www.zip");
    let test_outpath = String::from("C:\\temp\\convwww.zip");
    let test_pixels:[u32;2] =[750,1334];

   let _ = unzip_to_memory(test_path, PrintMode::Print).convert_size(test_pixels[0], test_pixels[1], ConvMode::Height).create_zip(test_outpath);
  
}

