use std::borrow::BorrowMut;
use std::fmt::Error;
use std::path::Path;

use std::fs::File;
use std::io::{Read, BufReader, Seek, SeekFrom, Result, Stdout};
use std::fs;
use std::io;
use image::imageops::FilterType;
use image::{ImageFormat, buffer, DynamicImage, ImageResult, ImageError};

use zip::read::ZipFile;

pub  struct  Input_ZipFile {

   pub  InputPath_str:String,
   pub UnzipFile:Vec<image::DynamicImage>,
   pub debug_str:String,
    
}

pub struct Input_MemoryFiles{

    pub InputMemoryFiles:Vec<DynamicImage>,
    pub OutputPath_str:String,
    pub debug_str:String,
    pub ConvImages:Option<Vec<image::DynamicImage>>
    
}



impl  Input_ZipFile {
   pub fn Unzip_toMemory(&mut self)->Option<Vec<DynamicImage>>{

    let fname = std::path::Path::new(&self.InputPath_str);
         let file = fs::File::open(&fname).unwrap();
     
         let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut MemoryFiles:Vec<DynamicImage> = Vec::new();
         for i in 0..archive.len() {
             let mut file = archive.by_index(i).unwrap();
             let outpath = match file.enclosed_name() {
                 Some(path) => path.to_owned(),
                 None => continue,
             };
     
             {
                 let comment = file.comment();
                 if !comment.is_empty() {
                     println!("File {} comment: {}", i, comment);
                 }
             }
     
             if (*file.name()).ends_with('/') {
                 println!("File {} extracted to \"{}\"", i, outpath.display());
                 fs::create_dir_all(&outpath).unwrap();
             } else {
                 println!(
                     "File {} extracted to \"{}\" ({} bytes)",
                     i,
                     outpath.display(),
                     file.size()
                 );
                 if let Some(p) = outpath.parent() {
                     if !p.exists() {
                         fs::create_dir_all(&p).unwrap();
                     }
                 }
                
                 let mut bf_out:Vec<u8> = Vec::new();
                 file.read_to_end(&mut bf_out);
             
                 let mut im = image::load_from_memory(Some(bf_out).as_deref().unwrap());
                
                MemoryFiles.push(im.unwrap());
                // let mut outfile = fs::File::create(&outpath).unwrap();
                 
             }
     
             // Get and Set permissions
             #[cfg(unix)]
             {
                 use std::os::unix::fs::PermissionsExt;
     
                 if let Some(mode) = file.unix_mode() {
                     fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                 }
             }
         }
        if MemoryFiles.len() > 1 { return Some(MemoryFiles);}
        return None
    }


    pub fn Debug_str(&mut self)-> &String{
        
        return &self.debug_str
    }
    pub fn Debug(&mut self){

      self.debug_str = String::from("okokok");
      
   }

   pub fn Unzip2(&mut self) -> i32{

         let fname = std::path::Path::new(&self.InputPath_str);
         let file = fs::File::open(&fname).unwrap();
     
         let mut archive = zip::ZipArchive::new(file).unwrap();
     
         for i in 0..archive.len() {
             let mut file = archive.by_index(i).unwrap();
             let outpath = match file.enclosed_name() {
                 Some(path) => path.to_owned(),
                 None => continue,
             };
     
             {
                 let comment = file.comment();
                 if !comment.is_empty() {
                     println!("File {} comment: {}", i, comment);
                 }
             }
     
             if (*file.name()).ends_with('/') {
                 println!("File {} extracted to \"{}\"", i, outpath.display());
                 fs::create_dir_all(&outpath).unwrap();
             } else {
                 println!(
                     "File {} extracted to \"{}\" ({} bytes)",
                     i,
                     outpath.display(),
                     file.size()
                 );
                 if let Some(p) = outpath.parent() {
                     if !p.exists() {
                         fs::create_dir_all(&p).unwrap();
                     }
                 }
                 let mut outfile = fs::File::create(&outpath).unwrap();
                 io::copy(&mut file, &mut outfile).unwrap();
             }
     
             // Get and Set permissions
             #[cfg(unix)]
             {
                 use std::os::unix::fs::PermissionsExt;
     
                 if let Some(mode) = file.unix_mode() {
                     fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
                 }
             }
         }
     
         0
     }


} 

impl Input_MemoryFiles {
   
    pub fn Convert_Size(&mut self) {
        for im in &self.InputMemoryFiles{

            
        }
    }

    pub fn ZipArchive(&mut self){
      

    }

}