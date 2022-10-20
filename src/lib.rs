use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use std::fs;
use std::fs::File;
use std::io::{stdout, Read, Write};

use image::imageops::FilterType;
use image::DynamicImage;
use image::ImageEncoder;

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

pub fn unzip_to_memory(input_path_str: String, print_mode: PrintMode) -> MemoryImages {
    let fname = std::path::Path::new(&input_path_str);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let mut memory_images: Vec<DynamicImage> = Vec::new();
    let mut r_path = vec![];
    let debug_s_time = std::time::Instant::now();
    let mut print = false;
    match print_mode {
        PrintMode::Print => print = true,
        PrintMode::Unprint => print = false,
    }

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
            if print {
                println!("File {} ext \"{}\"", i, outpath.display());
            }
            fs::create_dir_all(&outpath).unwrap();
            r_path.push(PathBuf::from(&outpath.to_str().unwrap()));
            memory_images.push(DynamicImage::new_rgb32f(1, 1));
        } else {
            let debug_e_time = std::time::Instant::now();
            if print {
                print!(
                    "\rFile {} ext to \"{}\" ({} bytes){:?}",
                    i,
                    outpath.display(),
                    file.size(),
                    debug_e_time.duration_since(debug_s_time)
                );
                stdout().flush().unwrap();
            }

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            let mut bf_out: Vec<u8> = Vec::new();
            let _ = file.read_to_end(&mut bf_out);

            let im = image::load_from_memory(Some(bf_out).as_deref().unwrap());

            memory_images.push(im.unwrap());
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
    if memory_images.len() > 1 {
        if memory_images.len() != r_path.len() {
            print!(
                "len anomaly ImageLen:{} pathLen:{}",
                memory_images.len(),
                r_path.len()
            );
        }

        return MemoryImages {
            input_memory_images: memory_images,
            out_names: r_path,
            print_mode: print_mode,
        };
    }
    panic!("Zip Read Error");
}

pub struct MemoryImages {
    pub input_memory_images: Vec<DynamicImage>,
    pub out_names: Vec<PathBuf>,
    pub print_mode: PrintMode,
}

impl MemoryImages {
    pub fn convert_size(&self, as_width: u32, as_height: u32, conv_mode: ConvMode) -> MemoryImages {
        let mut conv_images: Vec<DynamicImage> = Vec::new();
        let mut print = false;
        let mut pm = PrintMode::Unprint;
        let mut conv_width = as_width.clone();
        let mut conv_height = as_height.clone();
        match self.print_mode {
            PrintMode::Print => {
                print = true;
                pm = PrintMode::Print
            }
            PrintMode::Unprint => {
                print = false;
                pm = PrintMode::Unprint
            }
        }

        for mut im in &self.input_memory_images {
            match conv_mode {
                ConvMode::Height => {
                    let w_p: f32 = as_height as f32 / im.height() as f32;
                    conv_width = ((im.width() as f32) * &w_p) as u32;
                }
                ConvMode::Width => {
                    let h_p: f32 = as_width as f32 / im.width() as f32;
                    conv_height = (im.height() as f32 * &h_p) as u32;
                }
                ConvMode::Both => {}
            }
            let conv_im = im.resize(conv_width, conv_height, FilterType::CatmullRom);
            conv_images.push(conv_im);
            // p_bar.inc(1);
        }
        return MemoryImages {
            input_memory_images: conv_images,
            out_names: self.out_names.clone(),
            print_mode: pm,
        };
    }

    pub fn create_zip(
        &mut self,
        outpath: String,
        _save_format: SaveFormat,
        mut quality: u8,
    ) -> zip::result::ZipResult<()> {
        let path_temp = Path::new(&outpath);
        let file = File::create(&path_temp).unwrap();
        let mut zip = zip::ZipWriter::new(file);

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

            let _ = zip.start_file(self.out_names[count_i].to_str().unwrap(), options);

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
            print!(
                "\rok{}_{:?}",
                self.out_names[count_i].to_str().unwrap(),
                debug_e_time.duration_since(debug_s_time)
            );
            stdout().flush().unwrap();
            count_i += 1;
        }
        zip.finish()?;
        println!("FINSH");
        Ok(())
    }
}
