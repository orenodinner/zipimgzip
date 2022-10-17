use std::path::Path;
use std::path::PathBuf;

use std::fs::File;
use std::io::{Read,Write, stdout};
use std::{fs};

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


pub struct InputMemoryFiles{

    pub input_memory_files:Vec<DynamicImage>,
    pub out_names:Vec<PathBuf>,
    pub output_path_str:String,
    pub print:bool
    
}



impl  InputZipFile {
   pub fn unzip_to_memory(&mut self)->(Option<Vec<DynamicImage>>,Vec<PathBuf>){

    let fname = std::path::Path::new(&self.input_path_str);
         let file = fs::File::open(&fname).unwrap();
         let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut memory_files:Vec<DynamicImage> = Vec::new();
        let mut r_path = vec![];
        //let mut temp_len = archive.len().clone();
        //let p_bar = ProgressBar::new(temp_len as u64);
       
        let debug_s_time = std::time::Instant::now();
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
                 memory_files.push(DynamicImage::new_rgb32f(1, 1));
             } else {
                let debug_e_time= std::time::Instant::now();
              if self.print{   print!(
                     "\rFile {} ext to \"{}\" ({} bytes){:?}",
                     i,
                     outpath.display(),
                     file.size(),debug_e_time.duration_since(debug_s_time)
                 );
                 stdout().flush().unwrap();}
                 
                 if let Some(p) = outpath.parent() {
                     if !p.exists() {
                         fs::create_dir_all(&p).unwrap();
                     }
                 }
                
                 let mut bf_out:Vec<u8> = Vec::new();
                 let _ =  file.read_to_end(&mut bf_out);
             
                 let im = image::load_from_memory(Some(bf_out).as_deref().unwrap());
 
                memory_files.push(im.unwrap());
                r_path.push(PathBuf::from(&outpath.to_str().unwrap()));
  
                 
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
        if memory_files.len() > 1 { 
            if memory_files.len() != r_path.len(){print!("len anomaly ImageLen:{} pathLen:{}",memory_files.len(),r_path.len());}
            return (Some(memory_files),r_path)
        }
        return (None,r_path)
    }

    pub fn unzip_conv_to_memory(&mut self,as_width:u32,as_height:u32,conv_mode:ConvMode)-> Vec<DynamicImage>{
        let mut memory_files:Vec<DynamicImage> = Vec::new();
        let mut origin_images = vec![];
        
        let result_unzip = self.unzip_to_memory();
        let _outpath_str = result_unzip.1;

        match result_unzip.0 {
             Some(r) =>{origin_images = r}
            None =>{println!("toM_ERR")}
        }
        let mut conv_width = as_width.clone();
        let mut conv_height  =as_height.clone();

       for o_im in origin_images{

                match conv_mode {
                    ConvMode::Height =>{  
                        let w_p = &as_height / &o_im.height();
                        conv_width = &o_im.width() * &w_p;
                    }
                    ConvMode::Width => {
                        let h_p = &as_width / &o_im.width();
                        conv_height = &o_im.height() * &h_p;
                    }
                    ConvMode::Both =>{                       
                    }
                }
        o_im.resize(conv_width, conv_height, FilterType::CatmullRom);
        memory_files.push(o_im);
     
       }  
       return  memory_files;
    }

} 

impl InputMemoryFiles {
   
    pub fn convert_size(&mut self) {
        for im in &self.input_memory_files {
            
        
           // p_bar.inc(1);
        }
    }

    pub fn create_zip(&mut self,
        outpath: String
    ) -> zip::result::ZipResult<()>
    {
        
        
        let path_temp = Path::new(&outpath);
        let file = File::create(&path_temp).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let mut _i =0;
       
    
let mut count_i = 0;
        for mut im in &self.input_memory_files{
        let debug_s_time = std::time::Instant::now();
       
           _i += 1;
        

         let _ =  zip.start_file(self.out_names[count_i].to_str().unwrap(), options);
         
           let mut w = vec![];
           
          // image::codecs::png::PngEncoder::new(&mut w).write_image(im.as_bytes(), im.width(), im.height(), im.color());
         let _ =  image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w,90).write_image(im.as_bytes(), im.width(), im.height(), im.color());
      
         let _ = zip.write_all(&*w); 
        
          let debug_e_time = std::time::Instant::now();
           print!("\rok{}_{:?}",self.out_names[count_i].to_str().unwrap(),debug_e_time.duration_since(debug_s_time));
           stdout().flush().unwrap();
             count_i +=1;
        }
        zip.finish()?;
        println!("FINSH");
    Ok(())
    }


}