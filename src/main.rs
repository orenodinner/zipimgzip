
use ResizeImgZiper::InputZipFile;
use ResizeImgZiper::Input_MemoryFiles;
use image::DynamicImage;
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::path::PathBuf;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;

use std::path::Path;
use walkdir::{DirEntry, WalkDir};





fn main() {
    println!("Hello, world!");
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\1.jpg");
    let test_pixels:[u32;2] =[750,1334];
    
    /* 
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return ;
    }*/
    
    
    let mut izip = InputZipFile{
       print:true,
       // InputPath_str:String::from(&*args[1]),
       input_path_str:String::from(&test_path),
       };
    
    let  MemoryFiles = izip.Unzip_toMemory();

    match MemoryFiles.0  {
        Some(r) =>  {println!("\nOKmem");
    WriteMemoryFiles(r,MemoryFiles.1, test_outpath)} ,
        None    => println!("NGmem")
    }



    fn WriteMemoryFiles(v:Vec<DynamicImage>,outnames:Vec<PathBuf>,outpath: String){
    
        let mut mfiles = Input_MemoryFiles{
        InputMemoryFiles:v,
        out_names:outnames,
        OutputPath_str:String::from(outpath), 
        Name:String::from("test"),print:true
    };
  //  mfiles.Convert_Size(String::from("0011.jpg"));
    mfiles.CreateZipArchive(String::from("C:\\temp\\test_conv.zip"));

}

   

   // println!("{}",izip.Unzip());
   // println!("{}",&izip.debug_str());
}

