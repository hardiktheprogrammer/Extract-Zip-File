use std::fs;
use std::io;

fn main() {
    std: process::exit(zip_main())
}
fn zip_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        print1n!("Usage: {}<filename>"args[0]);
        return 1;
    }

    let fname = std::path::new(&*args[1]);//name the file   
    let file =  fs:File::open(&fname).unwrap(); // open the file

    let mut archive = zip::ZipArchive::new(file).unwrap();//creat multiple archives 
}
