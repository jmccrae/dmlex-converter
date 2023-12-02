
mod serialization;
mod model;
mod model_xml;
mod write_xml;
mod rdf;
use clap::{Parser,ValueEnum};
use thiserror::Error;
use crate::model::LexicographicResource;
use std::fs::File;
use std::io::{Read, Write};
use sophia::graph::inmem::{OpsWrapper, GenericGraph};
use sophia::term::factory::RcTermFactory;
use sophia::ns::Namespace;
use crate::rdf::ToRDF;
use sophia::serializer::turtle::{TurtleSerializer, TurtleConfig};
use sophia::serializer::TripleSerializer;
use std::convert::Infallible;
use sophia::prefix::PrefixBox;
use sophia::iri::IriBox;

static DMLEX : &str = "https://www.oasis-open.org/to-be-confirmed/dmlex#";
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
            let resource : crate::model_xml::LexicographicResource = serde_xml_rs::from_reader(input)?;
            Ok(resource.into())
        },
        Format::RDF => {
            panic!("RDF not implemented yet")
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
                let dmlex = Namespace::new(DMLEX).expect("DMLEX namespace is invalid");
                resource.to_rdf(&mut g, &ns2, &dmlex)?;
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
                                    IriBox::new_unchecked(DMLEX.into()),
                                ),
                                (
                                    PrefixBox::new_unchecked("lex".into()),
                                    IriBox::new(ns.clone().into())?,
                                )                                
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

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum WriteError {
    #[error("XML error: {0}")]
    XmlError(#[from] serde_xml_rs::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("RDF error: {0}")]
    RdfError(#[from] crate::rdf::RdfError),
    #[error("No default namespace specified")]
    NoDefaultNamespace,
    #[error("Invalid namespace: {0}")]
    InvalidNamespace(#[from] sophia::term::iri::error::InvalidIri),
    #[error("Turtle error: {0}")]
    TurtleError(#[from] sophia::triple::stream::StreamError<Infallible, std::io::Error>),
}
