use std::path::Path;

use std::fs::File;
use std::io::{Read, BufReader, Seek, SeekFrom, Result};

pub struct  Input_ZipFile {

   pub InputPath_str:String,
   pub UnzipFile:Vec<image::DynamicImage>,
   pub debug_str:String,
    
}



impl  Input_ZipFile {
   pub fn Unzip(mut self)-> String {

        self.UnzipFile = vec![image::DynamicImage::new_rgb32f(512, 512)];
       self.debug_str = String::from("change");
       return self.debug_str
    }
    pub fn debug_str(mut self)-> String{
        
        return self.debug_str
    }
} 