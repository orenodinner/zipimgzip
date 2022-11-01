use std::env;
use std::fmt::Error;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::vec;
use walkdir::{DirEntry, WalkDir};
use zipimgzip::*;

fn main() -> Result<(), io::Error> {
    let test_pixels: [u32; 2] = [750, 1334];
    let test_quality: u8 = 90;
    let THREAD_NUM = 12;
    let args: Vec<_> = env::args().collect();

    let mut Flist = vec![];

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return Err(io::Error::new(io::ErrorKind::NotFound, "Folder NotFound"));
    }

    let fname = std::path::Path::new(&*args[1]);
    let oname = std::path::Path::new(&*args[2]);

    let mut _i = 0;
    for entry in WalkDir::new(fname) {
        let entry = entry?;
        Flist.push(entry);
        if Flist.len() > THREAD_NUM {
            let mut handles = vec![];
            for ent in Flist.clone() {
                let a = oname.clone().to_path_buf();

                let handle = thread::spawn(move || {
                    let res;
                    match do_multithread(ent, a, test_pixels, test_quality) {
                        Ok(r) => {
                            res = r;
                        }
                        Err(e) => {
                            res = String::from("err");
                        }
                    }
                    return res;
                });

                print!("\rconv_Start:{}", _i);
                _i += 1;
                handles.push(handle);
            }
            println!("");
            for h in handles {
                let ans = h.join();
                match ans {
                    Ok(r) => {
                    }
                    Err(e) => {
                        println!("ansErr{:?}", e);
                    }
                }  
            }
            Flist.clear();
        }
    }
    Ok(())
}

fn do_multithread(
    entry: DirEntry,
    oname: PathBuf,
    test_pixels: [u32; 2],
    test_quality: u8,
) -> Result<String, io::Error> {
    let a = oname.clone();
    let return_path;
    match entry.path().extension() {
        None => return_path = String::from("None"),
        Some(r) => match r {
            r if r == "zip" => {
                let outpath = a
                    .join(entry.path().file_name().unwrap())
                    .to_str()
                    .unwrap()
                    .to_string();
                let _ = unzip_to_memory(
                    entry.path().to_str().unwrap().to_string(),
                    PrintMode::Unprint,
                )?
                .convert_size_multithread(test_pixels[0], test_pixels[1], ConvMode::Height)?
                .create_zip_multithread(
                    outpath.clone(),
                    SaveFormat::Ref,
                    test_quality,
                )?;
                return_path = outpath;
                println!("conv:{:?}", return_path)
            }
            _ => return_path = String::from("None"),
        },
    }
    return Ok(return_path);
}
