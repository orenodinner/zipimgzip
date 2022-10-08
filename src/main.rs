
use ResizeImgZiper::Input_ZipFile;
use ResizeImgZiper::Input_MemoryFiles;
use image::DynamicImage;
fn main() {
    println!("Hello, world!");
    let test_path = String::from("C:\\temp\\test.zip");
    let test_outpath = String::from("C:\\temp\\");
    /* 
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return ;
    }*/
    
    
    let mut izip = Input_ZipFile{
        debug_str:String::from("new"),
       // InputPath_str:String::from(&*args[1]),
       InputPath_str:String::from(&test_path),
        UnzipFile:vec![image::DynamicImage::new_rgb32f(5, 5)]};
    
    let  MemoryFiles = izip.Unzip_toMemory();

    match MemoryFiles  {
        Some(r) =>  {println!("OKmem");
    WriteMemoryFiles(r, test_outpath)
    
    } ,
        None    => println!("NGmem")
        
        
    }


    fn WriteMemoryFiles(v:Vec<DynamicImage>,outpath: String){
    let mut mfiles = Input_MemoryFiles{
        InputMemoryFiles:v,
        OutputPath_str:String::from(outpath), debug_str:String::from("new"),
        ConvImages:Some(vec![image::DynamicImage::new_rgb32f(5, 5)])

    };}
  


   // println!("{}",izip.Unzip());
   // println!("{}",&izip.debug_str());
}

