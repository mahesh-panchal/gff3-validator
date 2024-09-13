use clap::Parser;
use jsonschema::{Draft, JSONSchema};
use noodles::gff::{
    self,
    record::{Attributes, Phase, Strand},
    Record,
};
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, BufReader},
};

// Note: Might achieve speed up with Tokio library.

/// Validate a GFF3 file.
#[derive(Parser)]
struct Cli {
    // Path to GGF3 file to validate.
    gff3: std::path::PathBuf,
    // JSON Schema against which to validate.
    json_schema: std::path::PathBuf,
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
    attribute: HashMap<String, String>,
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

pub trait AttributesExt {
    fn to_hashmap(&self) -> HashMap<String, String>;
}

impl AttributesExt for Attributes {
    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (key, value) in self.iter() {
            map.insert(key.to_string(), value.to_string());
        }
        map
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
            attribute: record.attributes().to_hashmap(),
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("GFF3 validation begun.");

    let mut gff3_reader = File::open(&args.gff3)
        .map(BufReader::new)
        .map(gff::io::Reader::new)?;
    let schema_data = fs::read_to_string(&args.json_schema).unwrap();
    let schema_json: Value = serde_json::from_str(&schema_data)?;

    // Compile the schema
    let compiled_schema = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_json)
        .expect("A valid schema");

    // Validate gff3 records against schema
    for result in gff3_reader.records() {
        let record = result?;

        // println!(
        //     "{}\t{}\t{}",
        //     record.reference_sequence_name(),
        //     record.start(),
        //     record.end(),
        // );
        let my_record: MyGffRecord = record.into();
        let json = serde_json::to_string(&my_record).unwrap();
        let data_json: Value = serde_json::from_str(&json)?;

        // println!("{}", json);
        let validation_result = compiled_schema.validate(&data_json);
        if let Err(errors) = validation_result {
            println!("{}", json);
            for error in errors {
                println!("Validation error: {}", error);
                println!("Instance path: {}", error.instance_path);
            }
        }
    }

    println!("GFF3 validation complete.");
    Ok(())
}
