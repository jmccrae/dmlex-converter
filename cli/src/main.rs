use clap::{Parser,ValueEnum};
use dmlex::model::{LexicographicResource, Entry};
use dmlex::rdf::ToRDF;
use sophia::graph::inmem::LightGraph;
use sophia::graph::inmem::{OpsWrapper, GenericGraph};
use sophia::iri::IriBox;
use sophia::ns::Namespace;
use sophia::prefix::PrefixBox;
use sophia::serializer::TripleSerializer;
use sophia::serializer::turtle::{TurtleSerializer, TurtleConfig};
use sophia::term::factory::RcTermFactory;
use sophia::triple::stream::TripleSource;
use std::convert::Infallible;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};
use thiserror::Error;

type Graph = OpsWrapper<GenericGraph<u16, RcTermFactory>>;

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
enum Format {
    XML,
    RDF,
    JSON,
}

fn parse_file<R : Read>(input: R, format: &Format, args : &Args) -> Result<LexicographicResource, ParseError> {
    match format {
        Format::XML => {
            let resource : dmlex::model_xml::LexicographicResource = serde_xml_rs::from_reader(input)?;
            Ok(resource.into())
        },
        Format::RDF => {
            let mut buf_read = BufReader::new(input);
            let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples()
                .map_err(|e| ParseError::TurtleError(format!("{}", e)))?;
            if let Some(default_namespace) = &args.default_namespace {
                let ns = Namespace::new(default_namespace)?;
                Ok(dmlex::rdf::read_lexicographic_resource(&graph, &ns)?)
            } else {
                panic!("No default namespace specified");
            }
        },
        Format::JSON => {
            Ok(serde_json::from_reader(input)?)
        }
    }
}

fn write_file<W : Write>(output: W, format: &Format, resource: &LexicographicResource,
    args : &Args) -> Result<(), WriteError> {
    match format {
        Format::XML => {
            Ok(serde_xml_rs::to_writer(output, resource)?)
        },
        Format::RDF => {
            if let Some(ns) = &args.default_namespace {
                let mut g = Graph::new();
                let ns2 = Namespace::new(ns)?;
                let dmlex = Namespace::new(dmlex::rdf::DMLEX).expect("DMLEX namespace is invalid");
                resource.to_rdf(&mut g, &ns2, &dmlex, 0)?;
                let mut serializer = TurtleSerializer::new_with_config(output,
                    TurtleConfig::new().with_pretty(true)
                    .with_own_prefix_map(
                                vec![
                                (
                                    PrefixBox::new_unchecked("rdf".into()),
                                    IriBox::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("rdfs".into()),
                                    IriBox::new_unchecked("http://www.w3.org/2000/01/rdf-schema#".into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("xsd".into()),
                                    IriBox::new_unchecked("http://www.w3.org/2001/XMLSchema#".into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("dmlex".into()),
                                    IriBox::new_unchecked(dmlex::rdf::DMLEX.into()),
                                ),
                            ]));
                serializer.serialize_graph(&g)?;
                Ok(())
            } else {
                Err(WriteError::NoDefaultNamespace)
            }
        },
        Format::JSON => {
            Ok(serde_json::to_writer(output, resource)?)
        }
    }
}


fn parse_file_entry<R : Read>(input: R, format: &Format, args : &Args) -> Result<Entry, ParseError> {
    match format {
        Format::XML => {
            let resource : dmlex::model_xml::Entry = serde_xml_rs::from_reader(input)?;
            Ok(resource.into())
        },
        Format::RDF => {
            let mut buf_read = BufReader::new(input);
            let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples()
                .map_err(|e| ParseError::TurtleError(format!("{}", e)))?;
            if let Some(default_namespace) = &args.default_namespace {
                let ns = Namespace::new(default_namespace)?;
                Ok(dmlex::rdf::read_entry(&graph, &ns)?)
            } else {
                panic!("No default namespace specified");
            }
        },
        Format::JSON => {
            Ok(serde_json::from_reader(input)?)
        }
    }
}

fn write_file_entry<W : Write>(output: W, format: &Format, resource: &Entry,
    args : &Args) -> Result<(), WriteError> {
    match format {
        Format::XML => {
            Ok(serde_xml_rs::to_writer(output, resource)?)
        },
        Format::RDF => {
            if let Some(ns) = &args.default_namespace {
                let mut g = Graph::new();
                let ns2 = Namespace::new(ns)?;
                let dmlex = Namespace::new(dmlex::rdf::DMLEX).expect("DMLEX namespace is invalid");
                resource.to_rdf(&mut g, &ns2, &dmlex, 0)?;
                let mut serializer = TurtleSerializer::new_with_config(output,
                    TurtleConfig::new().with_pretty(true)
                    .with_own_prefix_map(
                                vec![
                                (
                                    PrefixBox::new_unchecked("rdf".into()),
                                    IriBox::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("rdfs".into()),
                                    IriBox::new_unchecked("http://www.w3.org/2000/01/rdf-schema#".into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("xsd".into()),
                                    IriBox::new_unchecked("http://www.w3.org/2001/XMLSchema#".into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("dmlex".into()),
                                    IriBox::new_unchecked(dmlex::rdf::DMLEX.into()),
                                ),
                            ]));
                serializer.serialize_graph(&g)?;
                Ok(())
            } else {
                Err(WriteError::NoDefaultNamespace)
            }
        },
        Format::JSON => {
            Ok(serde_json::to_writer(output, resource)?)
        }
    }
}

fn main() {
    let args = Args::parse();

    if args.entry {
        let resource : Entry = if let Some(input) = &args.input {
            if let Ok(file) = File::open(input.clone()) {
                match parse_file_entry(file, &args.input_format, &args) {
                    Ok(resource) => resource,
                    Err(e) => panic!("Could not parse input file {}: {}", input, e),
                }
            } else {
                panic!("Could not open input file {}", input);
            }
        } else {
            match parse_file_entry(std::io::stdin(), &args.input_format, &args) {
                Ok(resource) => resource,
                Err(e) => panic!("Could not parse input file: {}", e),
            }
        };
        if let Some(output) = &args.output {
            if let Ok(file) = File::create(output.clone()) {
                match write_file_entry(file, &args.output_format, &resource, &args) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not write output file: {}", e),
                }
            } else {
                panic!("Could not open output file {}", output);
            }
        } else {
            match write_file_entry(std::io::stdout(), &args.output_format, &resource, &args) {
                Ok(_) => (),
                Err(e) => panic!("Could not write output file: {}", e),
            }
        }

    } else {
        let resource : LexicographicResource = if let Some(input) = &args.input {
            if let Ok(file) = File::open(input.clone()) {
                match parse_file(file, &args.input_format, &args) {
                    Ok(resource) => resource,
                    Err(e) => panic!("Could not parse input file {}: {}", input, e),
                }
            } else {
                panic!("Could not open input file {}", input);
            }
        } else {
            match parse_file(std::io::stdin(), &args.input_format, &args) {
                Ok(resource) => resource,
                Err(e) => panic!("Could not parse input file: {}", e),
            }
        };
        if let Some(output) = &args.output {
            if let Ok(file) = File::create(output.clone()) {
                match write_file(file, &args.output_format, &resource, &args) {
                    Ok(_) => (),
                    Err(e) => panic!("Could not write output file: {}", e),
                }
            } else {
                panic!("Could not open output file {}", output);
            }
        } else {
            match write_file(std::io::stdout(), &args.output_format, &resource, &args) {
                Ok(_) => (),
                Err(e) => panic!("Could not write output file: {}", e),
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("RDF parse error: {0}")]
    RdfParseError(#[from] dmlex::rdf::RdfError),
    #[error("Invalid namespace: {0}")]
    InvalidNamespace(#[from] sophia::term::iri::error::InvalidIri),
    #[error("Turtle error: {0}")]
    TurtleError(String),
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("XML error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("RDF error: {0}")]
    RdfError(#[from] dmlex::rdf::RdfError),
    #[error("No default namespace specified")]
    NoDefaultNamespace,
    #[error("Invalid namespace: {0}")]
    InvalidNamespace(#[from] sophia::term::iri::error::InvalidIri),
    #[error("Turtle error: {0}")]
    TurtleError(#[from] sophia::triple::stream::StreamError<Infallible, std::io::Error>),
}
