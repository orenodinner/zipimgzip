
# zipimgzip
Resize and ZipArchive the images in the Zip.  
(Zip -> Image -> ResizeImage -> Zip )

## Example
Resize the images in the zip file to the specified size and compress them into a zip file
```rust
 fn main() -> Result<(), io::Error> {
 let test_path = String::from("C:\\test\\original.zip");
 let test_outpath = String::from("C:\\test\\conv.zip");
 let test_pixels: [u32; 2] = [750, 1334];
 let test_quality: u8 = 90;

 let _ = unzip_to_memory(test_path, PrintMode::Print)?
 .convert_size(test_pixels[0], test_pixels[1], ConvMode::Height)?
 .create_zip(test_outpath, SaveFormat::Ref, test_quality)?;
 
 return Ok(());
 }
```
### MultiThread exmanple
```rust
 fn main() -> Result<(), io::Error> {
 let test_pixels: [u32; 2] = [750, 1334];
 let test_quality: u8 = 90;
 let test_path = String::from("C:\\test\\test.zip");
 let test_outpath = String::from("C:\\test\\conv.zip");

 let _ = unzip_to_memory(test_path, PrintMode::Print)?
     .convert_size_multithread(test_pixels[0], test_pixels[1], ConvMode::Height)?
     .create_zip_multithread(test_outpath, SaveFormat::Ref, test_quality)?;

return Ok(())
}
```

## Support
Jpeg/Jpg/Png/Avif
bmp/jif/tiff/webp ->convert jpg




## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
