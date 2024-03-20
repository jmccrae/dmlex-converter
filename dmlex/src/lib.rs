pub mod model;
pub mod rdf;
pub mod serialization;
pub mod read_xml;
pub mod write_xml;

use crate::model::{LexicographicResource, Entry};
use crate::rdf::ToRDF;
use crate::write_xml::WriteXML;
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
use std::io::BufReader;
use std::io::{Read, Write};
use thiserror::Error;

type Graph = OpsWrapper<GenericGraph<u16, RcTermFactory>>;

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Format {
    XML,
    RDF,
    JSON,
}

pub fn parse<R : Read>(input: R, format: &Format, default_namespace : &Option<String>) -> Result<LexicographicResource, ParseError> {
    match format {
        Format::XML => {
            Ok(read_xml::read_xml(input, "lexicographicResource")?)
        },
        Format::RDF => {
            let mut buf_read = BufReader::new(input);
            let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples()
                .map_err(|e| ParseError::TurtleError(format!("{}", e)))?;
            if let Some(default_namespace) = &default_namespace {
                let ns = Namespace::new(default_namespace)?;
                Ok(crate::rdf::read_lexicographic_resource(&graph, &ns)?)
            } else {
                panic!("No default namespace specified");
            }
        },
        Format::JSON => {
            Ok(serde_json::from_reader(input)?)
        }
    }
}

pub fn write<W : Write>(mut output: W, format: &Format, resource: &LexicographicResource,
    default_namespace : &Option<String>, ontolex : bool) -> Result<(), WriteError> {
    match format {
        Format::XML => {
            let mut writer = xml::EmitterConfig::new().perform_indent(true).create_writer(&mut output);
            Ok(resource.write_xml(&mut writer)?)
        },
        Format::RDF => {
            if let Some(ns) = &default_namespace {
                let mut g = Graph::new();
                let ns2 = Namespace::new(ns)?;
                let dmlex = Namespace::new(crate::rdf::DMLEX).expect("DMLEX namespace is invalid");
                resource.to_rdf(&mut g, &ns2, &dmlex, 0, ontolex)?;
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
                                    IriBox::new_unchecked(crate::rdf::DMLEX.into()),
                                ),
                            ]));
                serializer.serialize_graph(&g)?;
                Ok(())
            } else {
                Err(WriteError::NoDefaultNamespace)
            }
        },
        Format::JSON => {
            Ok(serde_json::to_writer_pretty(output, resource)?)
        }
    }
}


pub fn parse_entry<R : Read>(input: R, format: &Format, default_namespace : &Option<String>) -> Result<Entry, ParseError> {
    match format {
        Format::XML => {
            Ok(read_xml::read_xml(input, "entry")?)
        },
        Format::RDF => {
            let mut buf_read = BufReader::new(input);
            let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples()
                .map_err(|e| ParseError::TurtleError(format!("{}", e)))?;
            if let Some(default_namespace) = &default_namespace {
                let ns = Namespace::new(default_namespace)?;
                Ok(crate::rdf::read_entry(&graph, &ns)?)
            } else {
                panic!("No default namespace specified");
            }
        },
        Format::JSON => {
            Ok(serde_json::from_reader(input)?)
        }
    }
}

pub fn write_entry<W : Write>(mut output: W, format: &Format, resource: &Entry,
    default_namespace : &Option<String>, ontolex : bool) -> Result<(), WriteError> {
    match format {
        Format::XML => {
            let mut writer = xml::EmitterConfig::new().perform_indent(true).create_writer(&mut output);
            Ok(resource.write_xml(&mut writer)?)
        },
        Format::RDF => {
            if let Some(ns) = &default_namespace {
                let mut g = Graph::new();
                let ns2 = Namespace::new(ns)?;
                let dmlex = Namespace::new(crate::rdf::DMLEX).expect("DMLEX namespace is invalid");
                resource.to_rdf(&mut g, &ns2, &dmlex, 0, ontolex)?;
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
                                    IriBox::new_unchecked(crate::rdf::DMLEX.into()),
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


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML error: {0}")]
    XmlError(#[from] read_xml::XMLErrorWithPosition),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("RDF parse error: {0}")]
    RdfParseError(#[from] crate::rdf::RdfError),
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
    RdfError(#[from] crate::rdf::RdfError),
    #[error("No default namespace specified")]
    NoDefaultNamespace,
    #[error("Invalid namespace: {0}")]
    InvalidNamespace(#[from] sophia::term::iri::error::InvalidIri),
    #[error("Turtle error: {0}")]
    TurtleError(#[from] sophia::triple::stream::StreamError<Infallible, std::io::Error>),
    #[error("XML Write error: {0}")]
    XmlWriteError(#[from] xml::writer::Error),
}

