use std::{
    fs::File,
    io::{self, BufReader},
};
use clap::Parser;
use noodles::gff as gff;
use noodles::gff::Record;
use noodles::gff::record::Strand;
use noodles::gff::record::Phase;
use serde::Serialize;

// Note: Might achieve speed up with Tokio library.

/// Validate a GFF3 file.
#[derive(Parser)]
struct Cli {
    /// The path to the GGF3 file to read
    path: std::path::PathBuf,
}

#[derive(Serialize)]
struct MyGffRecord {
    seqname: String,
    source: String,
    feature: String,
    start: usize,
    end: usize,
    score: Option<f32>,
    strand: Option<char>,
    phase: Option<u8>,
    attribute: String,
}

pub trait StrandExt {
    fn to_option_char(&self) -> Option<char>;
}

impl StrandExt for Strand {
    fn to_option_char(&self) -> Option<char> {
        match self {
            Strand::None => None,
            Strand::Forward => Some('+'),
            Strand::Reverse => Some('-'),
            Strand::Unknown => Some('?'),
        }
    }
}

pub trait PhaseExt {
    fn to_option_u8(&self) -> Option<u8>;
}

impl PhaseExt for Option<Phase> {
    fn to_option_u8(&self) -> Option<u8> {
        match self {
            Some(Phase::Zero) => Some(0),
            Some(Phase::One) => Some(1),
            Some(Phase::Two) => Some(2),
            None => None,
        }
    }
}

impl From<Record> for MyGffRecord {
    fn from(record: Record) -> Self {
        MyGffRecord {
            seqname: record.reference_sequence_name().to_string(),
            source: record.source().to_string(),
            feature: record.ty().to_string(),
            start: record.start().get(),
            end: record.end().get(),
            score: record.score(),
            strand: record.strand().to_option_char(),
            phase: record.phase().to_option_u8(),
            attribute: record.attributes().to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("GFF3 validation begun.");

    let mut reader = File::open(&args.path)
        .map(BufReader::new)
        .map(gff::io::Reader::new)?;

    for result in reader.records() {
        let record = result?;

        // println!(
        //     "{}\t{}\t{}",
        //     record.reference_sequence_name(),
        //     record.start(),
        //     record.end(),
        // );
        let my_record: MyGffRecord = record.into();
        let json = serde_json::to_string(&my_record).unwrap();
        println!("{}", json);
    }

    Ok(())
}
