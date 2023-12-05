use clap::{Parser,ValueEnum};
use dmlex::*;
use dmlex::model::{LexicographicResource, Entry};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long)]
    output: Option<String>,
    #[clap(long, default_value = "xml")]
    input_format: Format,
    #[clap(long, default_value = "xml")]
    output_format: Format,
    #[clap(long)]
    default_namespace: Option<String>,
    #[clap(long, default_value = "false")]
    entry: bool,
}

#[derive(Debug,Clone,ValueEnum)]
pub enum Format {
    XML,
    RDF,
    JSON,
}

impl Into<dmlex::Format> for Format {
    fn into(self) -> dmlex::Format {
        match self {
            Format::XML => dmlex::Format::XML,
            Format::RDF => dmlex::Format::RDF,
            Format::JSON => dmlex::Format::JSON,
        }
    }
}


fn main() {
    let args = Args::parse();

    if args.entry {
        let resource : Entry = if let Some(input) = &args.input {
            if let Ok(file) = File::open(input.clone()) {
                match parse_entry(file, &args.input_format.into(), &args.default_namespace) {
                    Ok(resource) => resource,
                    Err(e) => panic!("Could not parse input file {}: {}", input, e),
                }
            } else {
                panic!("Could not open input file {}", input);
            }
        } else {
            match parse_entry(std::io::stdin(), &args.input_format.into(), &args.default_namespace) {
                Ok(resource) => resource,
                Err(e) => panic!("Could not parse input file: {}", e),
            }
        };
        if let Some(output) = &args.output {
            if let Ok(file) = File::create(output.clone()) {
                match write_entry(file, &args.output_format.into(), &resource, &args.default_namespace) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not write output file: {}", e),
                }
            } else {
                panic!("Could not open output file {}", output);
            }
        } else {
            match write_entry(std::io::stdout(), &args.output_format.into(), &resource, &args.default_namespace) {
                Ok(_) => (),
                Err(e) => panic!("Could not write output file: {}", e),
            }
        }

    } else {
        let resource : LexicographicResource = if let Some(input) = &args.input {
            if let Ok(file) = File::open(input.clone()) {
                match parse(file, &args.input_format.into(), &args.default_namespace) {
                    Ok(resource) => resource,
                    Err(e) => panic!("Could not parse input file {}: {}", input, e),
                }
            } else {
                panic!("Could not open input file {}", input);
            }
        } else {
            match parse(std::io::stdin(), &args.input_format.into(), &args.default_namespace) {
                Ok(resource) => resource,
                Err(e) => panic!("Could not parse input file: {}", e),
            }
        };
        if let Some(output) = &args.output {
            if let Ok(file) = File::create(output.clone()) {
                match write(file, &args.output_format.into(), &resource, &args.default_namespace) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not write output file: {}", e),
                }
            } else {
                panic!("Could not open output file {}", output);
            }
        } else {
            match write(std::io::stdout(), &args.output_format.into(), &resource, &args.default_namespace) {
                Ok(_) => (),
                Err(e) => panic!("Could not write output file: {}", e),
            }
        }
    }
}
