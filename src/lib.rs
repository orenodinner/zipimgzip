use std::borrow::BorrowMut;
use std::fmt::Error;
use std::path::Path;

use std::fs::File;
use std::io::{Read, BufReader, Seek,Write, SeekFrom, Result, Stdout};
use std::fs;
use std::io;
use image::imageops::FilterType;
use image::{ImageFormat, buffer, DynamicImage, ImageResult, ImageError};

use zip::ZipWriter;
use zip::read::ZipFile;

use std::io::prelude::*;
use std::io::Cursor;

use std::iter::Iterator;
use zip::result::ZipError;
use zip::write::FileOptions;
use walkdir::{DirEntry, WalkDir};



pub  struct  Input_ZipFile {

   pub  InputPath_str:String,
   pub UnzipFile:Vec<image::DynamicImage>,
   pub debug_str:String,
    
}

pub struct Input_MemoryFiles{

    pub InputMemoryFiles:Vec<DynamicImage>,
    pub OutputPath_str:String,
    pub debug_str:String,
    pub ConvImages:Option<Vec<image::DynamicImage>>,
    pub Name:String
    
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
        let outPath = std::path::Path::new(&self.OutputPath_str);
        for im in &self.InputMemoryFiles{
            
          match  im.save(outPath) {
            Ok(v) => println!("ok_save"),
            Err(e)=> println!("Err{}",e)
              
          } 
            
        }
    }

    pub fn ZipArchive(&mut self){
      

    }
    pub fn CreateZipArchive(&mut self,
        outpath: String
    ) -> zip::result::ZipResult<()>
    {
        let it = &self.InputMemoryFiles;
        let mut buf = Cursor::new(vec![]);
        let path_temp = Path::new(&outpath);
        let file = File::create(&path_temp).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
       // let mut buf8 = &mut [];
        
        for im in it{
          match  im.write_to(&mut buf,
                ImageFormat::Jpeg){
                    Ok(e) => {
                        println!("s_buf");
                        let mut temp_zip = ZipWriter::new(buf.clone());
                       let temp_cursor= temp_zip.finish().unwrap();
                            // And get a reference to the original [u8] slice
    let r_buf = temp_cursor.get_ref();
    // These are the bytes we want
    
                       //buf.write(buf8).unwrap();
                        //let r_buf = buf.get_ref();
                        zip.start_file(&outpath,options)?;
                        zip.write_all(r_buf);
                        println!("F_buf");
                },
                    Err(r) => println!("NG_buf")
                }
        }
        zip.finish()?;
        println!("FINSH");
    Ok(())
    }


}