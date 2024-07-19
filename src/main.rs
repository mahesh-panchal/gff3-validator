/*
 * A tool to validate GFF3 files, before conversion to EMBL flatfile format and ENA webin submission.
 */

use std::{
    fs::File,
    io::{self, BufReader, Result},
};
use clap::Parser;
use noodles::gff as gff;
use noodles::gff::Record;
use noodles::gff::record::Strand;
use serde::{Serialize, Deserialize};
// use std::collections::HashMap;
// use serde_json::to_string;
// use std::collections::{HashMap, HashSet};
// use regex::Regex;
// Note: Might achieve speed up with Tokio library.

#[derive(Parser)]
struct Cli {
    /// The path to the GGF3 file to read
    path: std::path::PathBuf,
}

// Deserialise converts the input data into Serde data model
impl<'de> Deserialize<'de> for Strand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // TODO str type is wrong.
        match str::deserialize(deserializer)? {
            "none" => Ok(Strand::None),
            "forward" => Ok(Strand::Forward),
            "reverse" => Ok(Strand::Reverse),
            "unknown" => Ok(Strand::Unknown),
            _ => Err(serde::de::Error::custom("Expected none, forward, reverse, or unknown for strand")),
        }
    }
}
impl Serialize for Strand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Strand::None => "none",
            Strand::Forward => "forward",
            Strand::Reverse => "reverse",
            Strand::Unknown => "unknown",
        })
    }
}


fn main() -> Result<()> {
    let args = Cli::parse();
    println!("GFF3 validation begun.");

    let mut reader = File::open(&args.path)
        .map(BufReader::new)
        .map(gff::io::Reader::new)?;

    while let Some(record) = reader.read_record().expect("Error reading record") {
        let json_record = serde_json::to_string(&Gff3Feature::from(&record));
        println!("{}", json_record);
    }
    
    // for result in reader.records() {
    //     let record = result?;

    //     let json_record = serde_json::to_string(&record)?;
    //     println!("{}", json_record);
    //     Ok(json_record)
    //     // println!(
    //     //     "{}\t{}\t{}",
    //     //     record.reference_sequence_name(),
    //     //     record.start(),
    //     //     record.end(),
    //     // );
    // }

    Ok(())
}
