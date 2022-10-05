
use ResizeImgZiper::Input_ZipFile;
fn main() {
    println!("Hello, world!");
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return ;
    }
    
    
    let mut izip = Input_ZipFile{
        debug_str:String::from("new"),
        InputPath_str:String::from(&*args[1]),
        UnzipFile:vec![image::DynamicImage::new_rgb32f(5, 5)]};
    
    &izip.Unzip();
    println!("{:?}",&izip.Debug_str());
    &izip.Debug();
    println!("{:?}",&izip.Debug_str());

   // println!("{}",izip.Unzip());
   // println!("{}",&izip.debug_str());
}

