// to import external crate extern is used
extern crate flate2;
// GzEncoder-a writer that compresses data using GZip format
use flate2::write::GzEncoder;
// Compression -> (a struct to control compression level) from the flate2 crate.
use flate2::Compression;
// args -> a function that returns an iterator over command-line arguments passed to the program.
use std::env::args;
// File -> Imports File for opening and creating files on the filesystem.
use std::fs::File;
// copy -> a utility function that reads from a source and writes all bytes into a destination.
use std::io::copy;
// BufReader -> fetches a big chunk of data from the file once, stores it in memory, and your program reads from that memory chunk instead of constantly hitting the disk.
use std::io::BufReader;
// Instant -> used for high-precision time measurement (like a stopwatch).
use std::time::Instant;

fn main() {
    // Checks exactly 2 user provided arguments -> Source and Target
    if args().len() != 3 {
        // If not, it prints an error to stderr (eprintln!) and exits early.
        eprintln!("Usage: `Source` `Target`");
        return;
    }

    // args().nth(1) → gets the first user argument (source file path)
    // .unwrap() → extracts the value or panics if None
    // File::open(...) → opens the source file in read mode
    // BufReader::new(...) → wraps it with buffering for efficient reading
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    // args().nth(2) → gets the second user argument (target file path)
    // File::create(...) → creates the output file (overwrites if it already exists)
    // Not mut in output because GzEncoder will take ownership of it
    let output = File::create(args().nth(2).unwrap()).unwrap();

    // Creates a GzEncoder that wraps the output file. Compression::default() uses level 6 (a balance between speed and compression ratio).
    let mut encoder = GzEncoder::new(output, Compression::default());

    // Records the current timestamp —> acts as a stopwatch start.
    let start = Instant::now();

    // Reads all bytes from input (the source file) and writes them into encoder
    copy(&mut input, &mut encoder).unwrap();

    // Think of GZip compression like packing a box to ship a package
    let output = encoder.finish().unwrap();

    // input.get_ref() → gets a reference to the inner File inside the BufReader
    // .metadata() → fetches file metadata (size, permissions, etc.)
    // .len() → extracts the file size in bytes
    // Prints the original file size
    println!("Source len:{:?}", input.get_ref().metadata().unwrap().len());

    // Prints the compressed file size
    println!("Target len:{:?}", output.metadata().unwrap().len());

    // start.elapsed() computes the time since Instant::now() was called, and prints the total time taken for compression.
    println!("Elapsed: {:?}", start.elapsed());
}

// unwrap() -> "just give me the value inside, and if it fails, crash the program."
