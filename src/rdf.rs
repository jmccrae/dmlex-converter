//use sophia_term::factory::RcTermFactory;
//use sophia_inmem::graph::*;
use crate::model::*;
use sophia::graph::MutableGraph;
use sophia::ns::Namespace;
use sophia::term::ns::rdf;
use sophia::term::blank_node::BlankNode;
use thiserror::Error;
use sophia::term::{TTerm, TermKind};
use sophia::term::simple_iri::SimpleIri;
use sophia::term::RawValue;

type Result<T> = std::result::Result<T, RdfError>;

pub trait ToRDF {
    fn to_rdf<G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&self, 
        graph: &mut G, data : &Namespace<T1>, dmlex: &Namespace<T2>) -> 
        Result<()>;
}

impl ToRDF for LexicographicResource {
    fn to_rdf<G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&self, 
        graph: &mut G, data : &Namespace<T1>, dmlex: &Namespace<T2>) -> 
        Result<()> {
        let id = if let Some(id) = &self.id {
            URIOrBlank::URI(data.get(id.as_str())?)
        } else {
            URIOrBlank::Blank(BlankNode::new("lexicographic_resource".to_string())?)
        };
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("LexicographicResource")?).expect("Error inserting triple");
        Ok(())

    }
}

enum URIOrBlank<'s> {
    URI(SimpleIri<'s>),
    Blank(BlankNode<String>),
}

impl TTerm for URIOrBlank<'_> {
    fn kind(&self) -> TermKind {
        match self {
            URIOrBlank::URI(_) => TermKind::Iri,
            URIOrBlank::Blank(_) => TermKind::BlankNode,
        }
    }

    fn value_raw(&self) -> RawValue<'_> {
        match self {
            URIOrBlank::URI(s) => s.value_raw(),
            URIOrBlank::Blank(s) => s.value_raw(),
        }
    }

    fn as_dyn(&self) -> &dyn TTerm {
        self
    }
}

#[derive(Error, Debug)]
pub enum RdfError {
    #[error("Invalid IRI: {0}")]
    InvalidIRI(#[from] sophia::term::iri::error::InvalidIri),
    #[error("Term Error: {0}")]
    TermError(#[from] sophia::term::TermError),
}
