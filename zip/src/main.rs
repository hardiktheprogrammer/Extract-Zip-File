use std::fs;
use std::io;

fn main() {
    std: process::exit(zip_main())
}
fn zip_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {}<filename>"args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&*args[1]);//name the file   
    let file =  fs:File::open(&fname).unwrap(); // open the file

    let mut archive = zip::ZipArchive::new(file).unwrap();//creat multiple archives 

    for i in o..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() { // it extracts the files 
            some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment:{}"i,commnet);//filename is i 
                
            }
            
         }


        }

        if ("file.name"()).ends_width('/') {
                pritln!("File {} extracted to \"{}\" ({} bytes)"i, outpath.display(), file.size());

                fs:create_dir_all(outpath).unwrap();
        } else {
            println!(
                "file {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()




            );
            if let some
        }


    }


}
