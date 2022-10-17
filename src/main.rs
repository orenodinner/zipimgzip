
use zipimgzip::InputZipFile;
use zipimgzip::InputMemoryFiles;
use image::DynamicImage;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};





fn main() {
    println!("Hello, world!");
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\1.jpg");
    let test_pixels:[u32;2] =[750,1334];
    
    let mut izip = InputZipFile{
       print:true,
       // InputPath_str:String::from(&*args[1]),
       input_path_str:String::from(&test_path),
       };
    
    let  MemoryFiles = izip.unzip_to_memory();

    match MemoryFiles.0  {
        Some(r) =>  {println!("\nOKmem");
    WriteMemoryFiles(r,MemoryFiles.1, test_outpath)},
        None    => println!("NGmem")
    }

    fn WriteMemoryFiles(v:Vec<DynamicImage>,outnames:Vec<PathBuf>,outpath: String){
        let mut mfiles = InputMemoryFiles{
        input_memory_files:v,
        out_names:outnames,
        output_path_str:String::from(outpath), 
        print:true
    };
    mfiles.create_zip(String::from("C:\\temp\\test_conv.zip"));

}

}

