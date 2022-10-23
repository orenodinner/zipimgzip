//!
//! # Docs
//! Resize and ZipArchive the images in the Zip.
//! (Zip -> Image -> ResizeImage -> Zip )
//!
//! ## Example
//! ```rust
//! fn main() -> Result<(), io::Error> {
//! let test_path = String::from("C:\\temp\\test.zip");
//! let test_outpath = String::from("C:\\temp\\conv.zip");
//! let test_pixels: [u32; 2] = [750, 1334];
//! let test_quality: u8 = 90;
//!
//! let _ = unzip_to_memory(test_path, PrintMode::Print)?
//! .convert_size(test_pixels[0], test_pixels[1], ConvMode::Height)?
//! .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
//! }
//!
//! return OK(());
//! ```

use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use std::fs;
use std::fs::File;
use std::io::{stdout, Read, Write};

use encoding_rs;
use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageEncoder;

#[derive(Clone)]
pub enum PrintMode {
    Print,
    Unprint,
}

pub enum ConvMode {
    Width,
    Height,
    Both,
}

pub enum SaveFormat {
    Jpeg,
    Png,
    Ref,
}

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
        /*  let outpath = match file.name_raw() {
            Some(path) => path.to_owned(),
            None => continue,
        };*/

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
            fs::create_dir_all(&outpath)?;

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

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
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
    pub fn convert_size(
        &self,
        as_width: u32,
        as_height: u32,
        conv_mode: ConvMode,
    ) -> Result<MemoryImages, io::Error> {
        let mut conv_images: Vec<DynamicImage> = Vec::new();
        let mut print = false;

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

        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

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

            let _ = zip.start_file(start_name, options);

            let mut w = vec![];
            let _os_str_jpg = OsStr::new("jpg");
            let _os_str_jpeg = OsStr::new("jpeg");
            let _os_str_png = OsStr::new("png");

            match _save_format {
                SaveFormat::Jpeg => {
                    let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                        .write_image(im.as_bytes(), im.width(), im.height(), im.color());
                }
                SaveFormat::Png => {
                    let _ = image::codecs::png::PngEncoder::new(&mut w).write_image(
                        im.as_bytes(),
                        im.width(),
                        im.height(),
                        im.color(),
                    );
                }

                SaveFormat::Ref => match self.out_names[count_i].extension() {
                    None => {}
                    Some(r) => match r {
                        _os_str_jpg => {
                            let _ =
                                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                                    .write_image(
                                        im.as_bytes(),
                                        im.width(),
                                        im.height(),
                                        im.color(),
                                    );
                        }
                        _os_str_jpeg => {
                            let _ =
                                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                                    .write_image(
                                        im.as_bytes(),
                                        im.width(),
                                        im.height(),
                                        im.color(),
                                    );
                        }
                        _os_str_png => {
                            let _ = image::codecs::png::PngEncoder::new(&mut w).write_image(
                                im.as_bytes(),
                                im.width(),
                                im.height(),
                                im.color(),
                            );
                        }

                        _ => {
                            let _ =
                                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                                    .write_image(
                                        im.as_bytes(),
                                        im.width(),
                                        im.height(),
                                        im.color(),
                                    );
                        }
                    },
                },

                _ => {
                    let _ = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut w, quality)
                        .write_image(im.as_bytes(), im.width(), im.height(), im.color());
                }
            }

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
}

fn shift_jis_encode(input: &[u8]) -> String {
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(input);
    let a = res.into_owned();
    return a;
}
