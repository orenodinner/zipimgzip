use std::path::Path;
use std::path::PathBuf;

use std::fs::File;
use std::io::{Read,Write, stdout};
use std::{fs};
use std::io;
use image::imageops::FilterType;
use image::{DynamicImage};
use image::ImageEncoder;


pub  struct  InputZipFile {

   pub  input_path_str:String,
   pub print:bool,
    
}

pub enum ConvMode {
    Width,
    Height,
    Both,
}


pub struct Input_MemoryFiles{

    pub InputMemoryFiles:Vec<DynamicImage>,
    pub out_names:Vec<PathBuf>,
    pub OutputPath_str:String,
    
    pub Name:String,
    pub print:bool
    
}



impl  InputZipFile {
   pub fn Unzip_toMemory(&mut self)->(Option<Vec<DynamicImage>>,Vec<PathBuf>){

    let fname = std::path::Path::new(&self.input_path_str);
         let file = fs::File::open(&fname).unwrap();
         let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut MemoryFiles:Vec<DynamicImage> = Vec::new();
        let mut r_path = vec![];
        //let mut temp_len = archive.len().clone();
        //let p_bar = ProgressBar::new(temp_len as u64);
       
        let debug_Stime = std::time::Instant::now();
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
              if self.print{   println!("File {} ext \"{}\"", i, outpath.display());}
                 fs::create_dir_all(&outpath).unwrap();
                r_path.push(PathBuf::from(&outpath.to_str().unwrap()));
                 MemoryFiles.push(DynamicImage::new_rgb32f(1, 1));
             } else {
                let debug_Etime= std::time::Instant::now();
              if self.print{   print!(
                     "\rFile {} ext to \"{}\" ({} bytes){:?}",
                     i,
                     outpath.display(),
                     file.size(),debug_Etime.duration_since(debug_Stime)
                 );
                 stdout().flush().unwrap();}
                 let debug_Stime = std::time::Instant::now();
                 if let Some(p) = outpath.parent() {
                     if !p.exists() {
                         fs::create_dir_all(&p).unwrap();
                     }
                 }
                
                 let mut bf_out:Vec<u8> = Vec::new();
                 file.read_to_end(&mut bf_out);
             
                 let mut im = image::load_from_memory(Some(bf_out).as_deref().unwrap());
                //let mut temp_im = &im;
                //im.unwrap().save(&outpath);
                MemoryFiles.push(im.unwrap());
                r_path.push(PathBuf::from(&outpath.to_str().unwrap()));
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
            // p_bar.inc(1);
            
         }
        if MemoryFiles.len() > 1 { 
            if MemoryFiles.len() != r_path.len(){print!("len anomaly ImageLen:{} pathLen:{}",MemoryFiles.len(),r_path.len());}
            return (Some(MemoryFiles),r_path)}

        return (None,r_path)
    }

    pub fn Unzip_conv_toMemory(&mut self,as_width:u32,as_height:u32,ConvMode:ConvMode)-> Vec<DynamicImage>{
        let mut MemoryFiles:Vec<DynamicImage> = Vec::new();
        let mut origin_images = vec![];
        
        let result_unzip = self.Unzip_toMemory();
        let outpath_str = result_unzip.1;

        match result_unzip.0 {
             Some(r) =>{origin_images = r}
            None =>{println!("toM_ERR")}
        }

       for o_im in origin_images{

                match ConvMode {
                    ConvMode::Height =>{  
                        let w_p = &as_height / &o_im.height();
                        let as_width = &o_im.width() * &w_p;
                    }
                    ConvMode::Width => {
                        let h_p = &as_width / &o_im.width();
                        let as_height = &o_im.height() * &h_p;
                    }
                    ConvMode::Both =>{                       
                    }
    
}
        o_im.resize(as_width, as_height, FilterType::CatmullRom);
        MemoryFiles.push(o_im);
     
       }
       
       return  MemoryFiles;
    }


  

   pub fn Unzip2(&mut self) -> i32{

         let fname = std::path::Path::new(&self.input_path_str);
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
   
    pub fn Convert_Size(&mut self,outpath:String) {
        let outPath = std::path::Path::new(&outpath);
        //let mut temp_len = self.InputMemoryFiles.len().clone();
        //let p_bar = ProgressBar::new(temp_len as u64);
        for im in &self.InputMemoryFiles {
            
          match  im.save(outPath) {
            Ok(v) => println!("ok_save"),
            Err(e)=> println!("Err{}",e)
              
          } 
           // p_bar.inc(1);
        }
    }

    pub fn ZipArchive(&mut self){
      

    }
    pub fn CreateZipArchive(&mut self,
        outpath: String
    ) -> zip::result::ZipResult<()>
    {
        
        
        let path_temp = Path::new(&outpath);
        let file = File::create(&path_temp).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
       // let mut buf8 = &mut [];
       let mut i_ =0;
       let jpg_str = String::from(".jpg");
       let png_str = String::from(".png");
       let name_temp = String::from("111.jpg");
       let dir_temp = tempfile::tempdir()?;
       let mut file_temp = tempfile::tempfile()?;
       let temp_path = dir_temp.path().join(Path::new(&name_temp));
      // let mut temp_len = self.InputMemoryFiles.len().clone();
      // let p_bar = ProgressBar::new(temp_len as u64);

       //let mut _buffer = vec![];
let mut count_i = 0;
        for mut im in &self.InputMemoryFiles{
            let debug_Stime = std::time::Instant::now();
         /* 
           match im.save(&temp_path) {
            Ok(v) => println!("ok_save"),
            Err(e)=> println!("Err{}",e)
               
           }*/ 
           i_ += 1;
          // let name_i = i_.to_string() + &jpg_str;
           let name_i = i_.to_string() + &png_str;


           zip.start_file(self.out_names[count_i].to_str().unwrap(), options);
           //let mut f = File::open(&temp_path)?;
           let mut w = vec![];
           
          // image::codecs::png::PngEncoder::new(&mut w).write_image(im.as_bytes(), im.width(), im.height(), im.color());
           image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w,90).write_image(im.as_bytes(), im.width(), im.height(), im.color());
           // let mut outfile = fs::File::create(&name_i).unwrap();
          // io::copy(&mut f, &mut outfile).unwrap();

        // im.write_to(&mut file_temp, ImageFormat::Png);
          
       // file_temp.read_to_end(&mut _buffer);
           
           
          // zip.write_all(&*w)?;
          zip.write_all(&*w); 
          //buffer.clear();
          let debug_Etime = std::time::Instant::now();
           print!("\rok{}_{:?}",self.out_names[count_i].to_str().unwrap(),debug_Etime.duration_since(debug_Stime));
           stdout().flush().unwrap();
         //  p_bar.inc(1);
            // let name_ = name_.clone() ;
             //let i_str = i_.to_string();
             count_i +=1;
        }
        zip.finish()?;
        println!("FINSH");
    Ok(())
    }


}