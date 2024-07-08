use std::{
    fs::File,
    io::{self, BufReader},
};
use clap::Parser;
use noodles::gff as gff;

// Note: Might achieve speed up with Tokio library.

/// Validate a GFF3 file.
#[derive(Parser)]
struct Cli {
    /// The path to the GGF3 file to read
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("GFF3 validation begun.");

    let mut reader = File::open(&args.path)
        .map(BufReader::new)
        .map(gff::io::Reader::new)?;

    for result in reader.records() {
        let record = result?;

        println!(
            "{}\t{}\t{}",
            record.reference_sequence_name(),
            record.start(),
            record.end(),
        );
    }

    Ok(())
}
