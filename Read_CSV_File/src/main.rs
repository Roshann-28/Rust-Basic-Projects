use anyhow::Result; // = Result<T, anyhow::Error>
use csv;

fn read_from_file(path: &str) -> Result<()> {
    // let mut reader = csv::Reader::from_path(path)?; -> treats the first row as headers by default to fix it
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

// where the program will enter for
fn main() {
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e);
    }
}
