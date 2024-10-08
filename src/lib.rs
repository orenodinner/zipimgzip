//!
//! # Docs
//! Resize and ZipArchive the images in the Zip.
//! (Zip -> Image -> ResizeImage -> Zip )
//!
//! ## Example
//! Resize the images in the zip file to the specified size and compress them into a zip file
//! ```rust
//! fn main() -> Result<(), std::io::Error> {
//! let test_path = String::from("C:\\test\\original.zip");
//! let test_outpath = String::from("C:\\test\\conv.zip");
//! let test_pixels: [u32; 2] = [750, 1334];
//! let test_quality: u8 = 90;
//!
//! let _ = unzip_to_memory(test_path, PrintMode::Print)?
//!     .convert_size(test_pixels[0], test_pixels[1], ConvMode::Height)?
//!     .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
//! return Ok(());
//! }
//!
//! ```
//! ### MultiThread exmanple
//! ```rust
//! fn main() -> Result<(), io::Error> {
//! let test_pixels: [u32; 2] = [750, 1334];
//! let test_quality: u8 = 90;
//! let test_path = String::from("C:\\test\\original.zip");
//! let test_outpath = String::from("C:\\test\\conv.zip");
//!
//! let _ = unzip_to_memory(test_path, PrintMode::Print)?
//!     .convert_size_multithread(test_pixels[0], test_pixels[1], ConvMode::Height)?
//!     .create_zip_multithread(test_outpath, SaveFormat::Ref, test_quality)?;
//!
//! Ok(())
//! }
//!```

use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use std::fs;
use std::fs::File;
use std::io::{stdout, Read, Write};

use std::vec;

use encoding_rs;
use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageEncoder;


use oxipng;


use std::thread;

#[derive(Clone)]
pub enum PrintMode {
    Print,
    Unprint,
}

#[derive(Clone)]
pub enum ConvMode {
    Width,
    Height,
    Both,
}

#[derive(Clone)]
pub enum SaveFormat {
    Jpeg,
    Png,
    ComicPng,
    Avif,
    Ref,
}

/// Assign the images in the Zip file to MemoryImages.
/// MemoryImages {
///     pub input_memory_images: Vec<DynamicImage>,
///     pub out_names: Vec<PathBuf>,
///     pub print_mode: PrintMode,
/// }
pub fn unzip_to_memory(
    input_path_str: String,
    print_mode: PrintMode,
) -> Result<MemoryImages, io::Error> {
    let fname = std::path::Path::new(&input_path_str);
    let file = fs::File::open(&fname)?;

    let mut archive = zip::ZipArchive::new(file)?;

    let mut memory_images: Vec<DynamicImage> = Vec::new();
    let mut r_path = vec![];
    let debug_s_time = std::time::Instant::now();
    let print;
    match print_mode {
        PrintMode::Print => print = true,
        PrintMode::Unprint => print = false,
    }
    let archive_len = &archive.len() - 1;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let file_name = file.name_raw();

        let outpath: PathBuf;
        match std::str::from_utf8(file_name) {
            Ok(r) => {
                outpath = PathBuf::from(r);
            }
            Err(_e) => {
                let a = &shift_jis_encode(&file_name).clone();
                outpath = PathBuf::from(a);
            }
        }

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            if print {
                print!("\rFile {}/{} ext \"{}\"", i, archive_len, outpath.display());
            }

            let to_str: &str;
            match &outpath.to_str() {
                Some(r) => to_str = r,
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "to_str()_Error"));
                }
            }
            r_path.push(PathBuf::from(to_str));
            memory_images.push(DynamicImage::new_rgb32f(1, 1));
        } else {
            let debug_e_time = std::time::Instant::now();
            if print {
                print!(
                    "\rFile {}/{} ext to \"{}\" ({} bytes){:?}",
                    i,
                    archive_len,
                    outpath.display(),
                    file.size(),
                    debug_e_time.duration_since(debug_s_time)
                );
                stdout().flush()?;
            }


            let mut bf_out: Vec<u8> = Vec::new();
            let _ = file.read_to_end(&mut bf_out);
            let from_memory;
            let some_bf_out = Some(bf_out);
            match some_bf_out.as_deref() {
                Some(r) => from_memory = r,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::OutOfMemory,
                        "file_read_to_end_Buf_Error",
                    ))
                }
            }

            let im;
            match image::load_from_memory(from_memory) {
                Err(e) => {
                    return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
                }
                Ok(r) => im = r,
            }
            memory_images.push(im);

            let outpath_str;
            let outpath_opt = &outpath.to_str();
            match outpath_opt {
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "outpath_opt"));
                }
                Some(r) => outpath_str = r,
            }
            r_path.push(PathBuf::from(outpath_str));
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }
    if memory_images.len() > 1 {
        if memory_images.len() != r_path.len() {
            print!(
                "len anomaly ImageLen:{} pathLen:{}",
                memory_images.len(),
                r_path.len()
            );
        }

        return Ok(MemoryImages {
            input_memory_images: memory_images,
            out_names: r_path,
            print_mode: print_mode,
        });
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "images.len zero"))
}

pub struct MemoryImages {
    pub input_memory_images: Vec<DynamicImage>,
    pub out_names: Vec<PathBuf>,
    pub print_mode: PrintMode,
}

impl MemoryImages {
    /// MemoryImage is resized to the specified size.
    /// Resizes a MemoryImage to the specified size; the aspect ratio is maintained by conv_mode.
    pub fn convert_size(
        &self,
        as_width: u32,
        as_height: u32,
        conv_mode: ConvMode,
    ) -> Result<MemoryImages, io::Error> {
        let mut conv_images: Vec<DynamicImage> = Vec::new();
        let print;

        let mut conv_width = as_width.clone();
        let mut conv_height = as_height.clone();
        match self.print_mode {
            PrintMode::Print => {
                print = true;
            }
            PrintMode::Unprint => {
                print = false;
            }
        }
        if print {
            println!("");
        }

        let mut im_i = 0;
        for im in &self.input_memory_images {
            let debug_s_time = std::time::Instant::now();
            match conv_mode {
                ConvMode::Height => {
                    let w_p: f32 = as_height as f32 / im.height() as f32;
                    conv_width = ((im.width() as f32) * &w_p) as u32;
                }
                ConvMode::Width => {
                    let h_p: f32 = as_width as f32 / im.width() as f32;
                    conv_height = (im.height() as f32 * &h_p) as u32;
                }
                ConvMode::Both => {
                    let w_p: f32 = as_height as f32 / im.height() as f32;
                    conv_width = ((im.width() as f32) * &w_p) as u32;
                    if conv_width > as_width {
                        let h_p: f32 = as_width as f32 / im.width() as f32;
                        conv_height = (im.height() as f32 * &h_p) as u32;
                    }
                }
            }
            let conv_im = im.resize(conv_width, conv_height, FilterType::CatmullRom);
            conv_images.push(conv_im);
            let debug_e_time = std::time::Instant::now();

            if print {
                print!(
                    "\rimage {}/{} conv to [{},{}] :{:?}",
                    im_i,
                    (&self.input_memory_images.len() - 1),
                    conv_width,
                    conv_height,
                    debug_e_time.duration_since(debug_s_time)
                );
            }
            stdout().flush()?;
            im_i += 1;
        }
        if print {
            println!("")
        }
        return Ok(MemoryImages {
            input_memory_images: conv_images,
            out_names: self.out_names.clone(),
            print_mode: self.print_mode.clone(),
        });
    }

    ///Converts MemoryImage to the specified image format and Zip compresses it.
    ///quality is a jpg parameter.
    pub fn create_zip(
        &mut self,
        outpath: String,
        _save_format: SaveFormat,
        mut quality: u8,
    ) -> Result<File, io::Error> {
        let path_temp = Path::new(&outpath);
        let file = File::create(&path_temp)?;
        let mut zip = zip::ZipWriter::new(file);

        let print;
        match self.print_mode {
            PrintMode::Print => {
                print = true;
            }
            PrintMode::Unprint => {
                print = false;
            }
        }


        let mut _i = 0;
        let mut count_i = 0;
        if quality > 100 {
            quality = 100
        };

        for im in &self.input_memory_images {
            let debug_s_time = std::time::Instant::now();

            _i += 1;
            let start_name: &str;
            match self.out_names[count_i].to_str() {
                Some(r) => start_name = r,
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "to_str()_Error"));
                }
            }
            let options:zip::write:: FileOptions<zip::write::ExtendedFileOptions> =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

            let mut new_outpath = self.out_names[count_i].clone();



            //let _ = zip.start_file(start_name, options);

            let mut w = vec![];
            let _os_str_jpg = OsStr::new("jpg");
            let _os_str_jpeg = OsStr::new("jpeg");
            let _os_str_png = OsStr::new("png");

            let mut new_outpath = self.out_names[count_i].clone();

            match _save_format {
                SaveFormat::Jpeg => {
                    let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                        .write_image(im.as_bytes(), im.width(), im.height(), im.color().into());
                    new_outpath.set_extension("jpg");
                    self.out_names[count_i] = new_outpath.clone();
                }
                SaveFormat::Png => {
                    let _ = image::codecs::png::PngEncoder::new_with_quality(&mut w,image::codecs::png::CompressionType::Best,image::codecs::png::FilterType::Paeth).write_image(
                        im.as_bytes(),
                        im.width(),
                        im.height(),
                        im.color().into(),
                    );
                    new_outpath.set_extension("png");
                    self.out_names[count_i] = new_outpath.clone();

                }

                SaveFormat::ComicPng => {
                    let c_im = conv_gray_quantize(im, &self.print_mode);
                    let _ = image::codecs::png::PngEncoder::new_with_quality(&mut w,image::codecs::png::CompressionType::Best,image::codecs::png::FilterType::Adaptive).write_image(
                        c_im.as_bytes(),
                        c_im.width(),
                        c_im.height(),
                        c_im.color().into(),
                    );
                    new_outpath.set_extension("png");
                    self.out_names[count_i] = new_outpath.clone();

                        // oxipngで最適化
                    let options = oxipng::Options::from_preset(6); // 圧縮レベルを指定
                    match oxipng::optimize_from_memory(&w, &options) {
                        Ok(optimized_data) => {
                            w = optimized_data;
                        }
                        Err(e) => {
                            return Err(io::Error::new(
                                io::ErrorKind::Other,
                                format!("Failed to optimize PNG: {}", e),
                            ));
                        }}

                }

                SaveFormat::Avif => {
                    let _ = image::codecs::avif::AvifEncoder::new_with_speed_quality(&mut w,3,70).write_image(
                        im.as_bytes(),
                        im.width(),
                        im.height(),
                        im.color().into(),
                    );
                    new_outpath.set_extension("avif");
                    self.out_names[count_i] = new_outpath.clone();
                }



                SaveFormat::Ref => match self.out_names[count_i].extension() {
                    None => {}
                    Some(r) => match r {
                        r if r == _os_str_jpg => {
                            let _ =
                                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                                    .write_image(
                                        im.as_bytes(),
                                        im.width(),
                                        im.height(),
                                        im.color().into(),
                                    );
                            new_outpath.set_extension("jpg");
                            self.out_names[count_i] = new_outpath.clone();
                        }
                        r if r == _os_str_jpeg => {
                            let _ =
                                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                                    .write_image(
                                        im.as_bytes(),
                                        im.width(),
                                        im.height(),
                                        im.color().into(),
                                    );
                            new_outpath.set_extension("jpeg");
                            self.out_names[count_i] = new_outpath.clone();

                        }
                        r if r == _os_str_png => {
                            let _ = image::codecs::png::PngEncoder::new_with_quality(&mut w,image::codecs::png::CompressionType::Best,image::codecs::png::FilterType::Paeth).write_image(
                                im.as_bytes(),
                                im.width(),
                                im.height(),
                                im.color().into(),
                            );
                            new_outpath.set_extension("png");
                            self.out_names[count_i] = new_outpath.clone();
                        }

                        _ => {
                            new_outpath.set_extension("jpg");
                            self.out_names[count_i] = new_outpath.clone();
                            let _ =
                                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                                    .write_image(
                                        im.as_bytes(),
                                        im.width(),
                                        im.height(),
                                        im.color().into(),
                                    );
                           
                        }
                    },
                },
            }
            let new_start_name = new_outpath.to_str().unwrap();
            let _ = zip.start_file(new_start_name, options);



            let _ = zip.write_all(&*w);

            let debug_e_time = std::time::Instant::now();

            let to_str: &str;
            match self.out_names[count_i].to_str() {
                Some(r) => to_str = r,
                None => {
                    return Err(io::Error::new(io::ErrorKind::Other, "to_str()_Error"));
                }
            }

            if print {
                print!(
                    "\rArchive to {}_{:?}",
                    to_str,
                    debug_e_time.duration_since(debug_s_time)
                );
                stdout().flush()?;
            }

            count_i += 1;
        }
        let zip_file = zip.finish()?;
        if print {
            println!("\nFINSH")
        };
        return Ok(zip_file);
    }

    /// MultiThread
    /// MemoryImage is resized to the specified size.
    /// Resizes a MemoryImage to the specified size; the aspect ratio is maintained by conv_mode.
    pub fn convert_size_multithread(
        &self,
        as_width: u32,
        as_height: u32,
        conv_mode: ConvMode,
    ) -> Result<MemoryImages, io::Error> {
        let mut conv_images: Vec<DynamicImage> = Vec::new();
        let mut conv_outpath = vec![];
        let print;

        match self.print_mode {
            PrintMode::Print => {
                print = true;
            }
            PrintMode::Unprint => {
                print = false;
            }
        }
        if print {
            println!("");
        }

        let mut im_i = 0;

        let conv_num;
        match conv_mode {
            ConvMode::Height => {
                conv_num = 1;
            }
            ConvMode::Width => {
                conv_num = 2;
            }
            ConvMode::Both => {
                conv_num = 3;
            }
        }

        thread::scope(|s| {
            let mut handles = vec![];

            for im in &self.input_memory_images {
                let out_path = &self.out_names[im_i];
                let print_mode = &self.print_mode;

                let handle = s.spawn(move || {
                    do_convert_image_multithread(
                        im, as_width, as_height, out_path, conv_num, print_mode,
                    )
                });
                handles.push(handle);
                im_i += 1;
            }

            for h in handles {
                let (im_conv, _outpath) = h.join().unwrap();
                conv_images.push(im_conv);
                conv_outpath.push(_outpath);
            }

            if print {
                println!("")
            }
        });
        return Ok(MemoryImages {
            input_memory_images: conv_images,
            out_names: conv_outpath,
            print_mode: self.print_mode.clone(),
        });
    }

    ///MultiThread
    ///Converts MemoryImage to the specified image format and Zip compresses it.
    ///quality is a jpg parameter.
    pub fn create_zip_multithread(
        &mut self,
        outpath: String,
        _save_format: SaveFormat,
        mut quality: u8,
    ) -> Result<File, io::Error> {
        let path_temp = Path::new(&outpath);
        let file = File::create(&path_temp)?;
        let mut zip = zip::ZipWriter::new(file);

        let print;
        match self.print_mode {
            PrintMode::Print => {
                print = true;
            }
            PrintMode::Unprint => {
                print = false;
            }
        }


        let mut _i = 0;

        if quality > 100 {
            quality = 100
        };
            // スレッドの処理結果を保持するベクタを初期化
    let num_images = self.input_memory_images.len();
    let mut vec_w = vec![None; num_images];
    let mut vec_startname = vec![None; num_images];

        thread::scope(|s| {
            let mut bit_handles = vec![];

            for im in &self.input_memory_images {
                let out_names = &self.out_names;
                let save_format =&_save_format;
                let bit_handle = s.spawn(move || {
                    do_create_imgtobit_multithread(_i, im, out_names, save_format, quality)
                });
                bit_handles.push(bit_handle);
                _i += 1;
            }
            for h in bit_handles {
                let (index,h_im, h_outpath) = h.join().unwrap();
                vec_w[index] = Some(h_im);
                vec_startname[index] = Some(h_outpath);
            }
        });
            // self.out_namesを更新
      for i in 0..num_images {
        if let Some(ref start_name) = vec_startname[i] {
            self.out_names[i] = PathBuf::from(start_name);
        }
    }




    // ZIPファイルに書き込む
    for i in 0..num_images {
        if let (Some(ref data), Some(ref start_name)) = (&vec_w[i], &vec_startname[i]) {
            let options: zip::write::FileOptions<zip::write::ExtendedFileOptions> =
                zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Stored);
            let _ = zip.start_file(start_name.clone(), options);
            let _ = zip.write_all(&data);
        }
    }

    let zip_file = zip.finish()?;
    if print {
        println!("\nFINSH")
    };
        return Ok(zip_file);
    }
}

fn shift_jis_encode(input: &[u8]) -> String {
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(input);
    let a = res.into_owned();
    return a;
}

fn do_convert_image_multithread(
    im: &DynamicImage,
    as_width: u32,
    as_height: u32,
    out_path: &PathBuf,
    conv_num: i32,
    print_mode: &PrintMode,
) -> (DynamicImage, PathBuf) {
    let mut conv_width = as_width.clone();
    let mut conv_height = as_height.clone();
    let print;
    match print_mode {
        PrintMode::Print => {
            print = true;
        }
        PrintMode::Unprint => {
            print = false;
        }
    }

    match conv_num {
        1 => {
            let w_p: f32 = as_height as f32 / im.height() as f32;
            conv_width = ((im.width() as f32) * &w_p) as u32;
        }
        2 => {
            let h_p: f32 = as_width as f32 / im.width() as f32;
            conv_height = (im.height() as f32 * &h_p) as u32;
        }
        3 => {
            let w_p: f32 = as_height as f32 / im.height() as f32;
            conv_width = ((im.width() as f32) * &w_p) as u32;
            if conv_width > as_width {
                let h_p: f32 = as_width as f32 / im.width() as f32;
                conv_height = (im.height() as f32 * &h_p) as u32;
            }
        }
        _ => {
            let w_p: f32 = as_height as f32 / im.height() as f32;
            conv_width = ((im.width() as f32) * &w_p) as u32;
        }
    }

    let conv_im = im.resize(conv_width, conv_height, FilterType::CatmullRom);

    if print {
        print!("\rimage conv{:?}", out_path);
    }

    return (conv_im, out_path.to_path_buf());
}

fn do_create_imgtobit_multithread(
    i: usize,
    im: &DynamicImage,
    out_names: &Vec<PathBuf>,
    _save_format: &SaveFormat,
    quality: u8,
) -> (usize,Vec<u8>, String) {
    let res_start_name;
    let mut w = vec![];
    match out_names[i].to_str() {
        Some(r) => res_start_name = r,
        None => {
            println!("to_str()_Error");
            panic!();
        }
    }

    let _os_str_jpg = OsStr::new("jpg");
    let _os_str_jpeg = OsStr::new("jpeg");
    let _os_str_png = OsStr::new("png");
    let _os_str_avif = OsStr::new("avif");

        // 修正点：ファイル名を変更するため、可変参照を取得
        let mut new_outpath = out_names[i].clone();

    match _save_format {
        SaveFormat::Jpeg => {
            let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                .write_image(im.as_bytes(), im.width(), im.height(), im.color().into());
            new_outpath.set_extension("jpg");
        }
        SaveFormat::Png => {
            let _ = image::codecs::png::PngEncoder::new_with_quality(&mut w,image::codecs::png::CompressionType::Best,image::codecs::png::FilterType::Paeth).write_image(
                im.as_bytes(),
                im.width(),
                im.height(),
                im.color().into(),
            );
            new_outpath.set_extension("png");
        }
        SaveFormat::ComicPng => {
            let c_im = conv_gray_quantize(im, &PrintMode::Unprint);
            let _ = image::codecs::png::PngEncoder::new_with_quality(&mut w,image::codecs::png::CompressionType::Best,image::codecs::png::FilterType::Adaptive).write_image(
                c_im.as_bytes(),
                c_im.width(),
                c_im.height(),
                c_im.color().into(),
            );
            new_outpath.set_extension("png");

            // oxipngで最適化
            let mut options = oxipng::Options::from_preset(6); // 圧縮レベルを指定
            options.scale_16 = true;
            options.optimize_alpha = true;
            options.deflate = oxipng::Deflaters::Libdeflater {
                compression: 12
            };


            match oxipng::optimize_from_memory(&w, &options) {
                Ok(optimized_data) => {
                    w = optimized_data;
                }
                Err(e) => {
                    panic!("Failed to optimize PNG: {}", e);
                }
            }


        }

        SaveFormat::Avif => {
            let _ = image::codecs::avif::AvifEncoder::new_with_speed_quality(&mut w,3,70).write_image(
                im.as_bytes(),
                im.width(),
                im.height(),
                im.color().into(),
            );
            new_outpath.set_extension("avif");
        }

        SaveFormat::Ref => match out_names[i].extension() {
            None => {}
            Some(r) => match r {
                r if r == _os_str_jpg => {
                    let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                        .write_image(im.as_bytes(), im.width(), im.height(), im.color().into());
                    new_outpath.set_extension("jpg");
                }
                r if r == _os_str_jpeg => {
                    let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                        .write_image(im.as_bytes(), im.width(), im.height(), im.color().into());
                    new_outpath.set_extension("jpeg");
                }
                r if r == _os_str_png => {
                    let _ = image::codecs::png::PngEncoder::new_with_quality(&mut w,image::codecs::png::CompressionType::Best,image::codecs::png::FilterType::Paeth).write_image(
                        im.as_bytes(),
                        im.width(),
                        im.height(),
                        im.color().into(),
                    );
                    new_outpath.set_extension("png");
                }

                r if r == _os_str_avif => {
                    let _ = image::codecs::avif::AvifEncoder::new(&mut w).write_image(
                        im.as_bytes(),
                        im.width(),
                        im.height(),
                        im.color().into(),
                    );
                    new_outpath.set_extension("avif");
                }

                _ => {
                    new_outpath.set_extension("jpg");

                    let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                        .write_image(im.as_bytes(), im.width(), im.height(), im.color().into());
                }
            },
        },
    }
    let res_start_name = new_outpath.to_str().unwrap_or("").to_string();
    let r = (&*w).to_vec();
    let p = res_start_name.to_string();
    return (i,r, p);
}


fn conv_gray_quantize(im: &DynamicImage, print_mode: &PrintMode) -> (DynamicImage) {
    let print;
    match print_mode {
        PrintMode::Print => {
            print = true;
        }
        PrintMode::Unprint => {
            print = false;
        }
    }

    // グレースケールに変換
    let gray_image = im.to_luma8();
    


    if print {
        print!("\rimage conv{:?}", "gray");
    }
    //quantized_imageをDynamicImageに変換して返す
    return DynamicImage::ImageLuma8(gray_image);

}