use std::fs; // gives us tools to work with files and folders on the computer
use std::io; // gives us tools to move/copy data from one place to another

fn main() {
    // rust always starts running your program from here
    std::process::exit(real_main()); // run real_main() and use its result to properly close the program
}

fn real_main() -> i32 {
    // this function does all the real work and returns 0 (success) or 1 (failure)
    let args: Vec<_> = std::env::args().collect(); // grab everything the user typed in the terminal into a list

    if args.len() < 2 {
        // if the user didn't provide a zip file name (only 1 thing in list = just the program name)
        println!("Usage: {} <filename>", args[0]); // show the user how to correctly run this program
        return 1; // stop the program early and signal that something went wrong
    }

    let fname = std::path::Path::new(&*args[1]); // convert the zip file name (text) into a proper file path rust can use
    let file = fs::File::open(&fname).unwrap(); // open the zip file from the disk (crashes if file doesn't exist)
    let mut archive = zip::ZipArchive::new(file).unwrap(); // treat the opened file as a zip archive so we can look inside it

    for i in 0..archive.len() {
        // go through every item inside the zip one by one (first=0, second=1, and so on)
        let mut file = archive.by_index(i).unwrap(); // pick out the item at position i from inside the zip

        let outpath = match file.enclosed_name() {
            // get the safe file path of this item (checks it won't escape our folder)
            Some(path) => path.to_owned(), // if the path is safe, use it
            None => continue, // if the path looks dangerous, skip this item and jump to the next one
        };

        {
            let comment = file.comment(); // read the comment/note stored with this file inside the zip (if any)
            if !comment.is_empty() {
                // only print the comment if there actually is one
                println!("File {} comment:{}", i, comment); // show the comment on screen
            }
        }
        if (*file.name()).ends_with('/') {
            // folder names always end with '/' — this checks if the item is a folder
            println!("File {} extracted to \"{}\"", i, outpath.display()); // tell the user a folder is being created
            fs::create_dir_all(&outpath).unwrap(); // create this folder on disk (also creates any missing parent folders)
        } else {
            // if it doesn't end with '/', it's a regular file
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(), // where the file will be saved on disk
                file.size()        // how big the file is in bytes
            );
        }
        if let Some(p) = outpath.parent() {
            // check if this file sits inside a sub-folder that needs to exist first
            if !p.exists() {
                // if that parent folder doesn't exist on disk yet
                fs::create_dir_all(&p).unwrap(); // create it now (so the file has somewhere to be saved)
            }
        }
        let mut outfile = fs::File::create(&outpath).unwrap(); // create a new empty file on disk at the target location
        io::copy(&mut file, &mut outfile).unwrap(); // pour all the bytes from the zip entry into the new file on disk
    }
    #[cfg(unix)]
    // everything inside this block only runs on unix systems like linux or macos — ignored on windows
    {
        use std::os::unix::fs::PermissionExt; // bring in the unix tool that lets us set read/write/execute permissions

        if let Some(mode) = file.unix_mode() {
            // check if the zip stored unix permission info for this file
            fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            // apply those permissions to the extracted file
        }
    };
    0 // return 0 to tell the system everything finished successfully (0 = all good in unix/windows exit codes)
}
