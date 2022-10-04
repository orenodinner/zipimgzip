
use std::ptr::null;

use ResizeImgZiper::Input_ZipFile;
fn main() {
    println!("Hello, world!");
    
    let mut izip = Input_ZipFile{
        debug_str:String::from("new"),
        InputPath_str:String::from("a"),
        UnzipFile:vec![image::DynamicImage::new_rgb32f(5, 5)]};
    
    
    println!("{}",izip.Unzip());
   // println!("{}",&izip.debug_str());
}

