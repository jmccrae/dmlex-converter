//use sophia_term::factory::RcTermFactory;
//use sophia_inmem::graph::*;
use crate::model::*;
use rand::Rng;
use rand;
use sophia::graph::{Graph, MutableGraph};
use sophia::ns::Namespace;
use sophia::term::RawValue;
use sophia::term::blank_node::BlankNode;
use sophia::term::literal::convert::AsLiteral;
use sophia::iri::Iri;
use sophia::term::ns::{rdf,owl};
use sophia::term::simple_iri::SimpleIri;
use sophia::term::{Term, TTerm, TermKind, CopyTerm};
use sophia::triple::Triple;
use thiserror::Error;

type Result<T> = std::result::Result<T, RdfError>;
pub static DMLEX : &str = "https://www.oasis-open.org/to-be-confirmed/dmlex#";

pub trait ToRDF {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>>;
}

pub trait FromRDF {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized;
}

pub fn read_lexicographic_resource<G : Graph, T: AsRef<str>>(g : &G, data : &Namespace<T>) -> Result<LexicographicResource> {
    let dmlex = Namespace::new(DMLEX).expect("DMLEX namespace is invalid");
    for triple in g.triples_with_po(&rdf::type_, &dmlex.get("LexicographicResource")?) {
        let o = Term::copy(triple.unwrap().s());
        return Ok(LexicographicResource::from_rdf(&o, g, &dmlex, data)?.1);
    }
    Err(RdfError::MissingLexicographicResource)
}

pub fn read_entry<G : Graph, T: AsRef<str>>(g : &G, data : &Namespace<T>) -> Result<Entry> {
    let dmlex = Namespace::new(DMLEX).expect("DMLEX namespace is invalid");
    for triple in g.triples_with_po(&rdf::type_, &dmlex.get("Entry")?) {
        let o = Term::copy(triple.unwrap().s());
        return Ok(Entry::from_rdf(&o, g, &dmlex, data)?.1);
    }
    Err(RdfError::MissingEntry)
}

impl ToRDF for LexicographicResource {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("LexicographicResource")?).expect("Error inserting triple");
        if let Some(s) = &self.title {
            graph.insert(
                &id,
                &dmlex.get("title")?,
                &s.as_literal()).expect("Error inserting triple");
        }
        if let Some(uri) = &self.uri {
            graph.insert(
                &id,
                &dmlex.get("uri")?,
                &uri.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("langCode")?,
            &self.lang_code.0.as_literal()).expect("Error inserting triple");
        for (i,entry) in self.entries.iter().enumerate() {
            let entry_id = entry.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("entry")?,
                &entry_id).expect("Error inserting triple");
        }
        for translation_language in self.translation_languages.iter() {
            graph.insert(
                &id,
                &dmlex.get("translationLanguage")?,
                &translation_language.as_literal()).expect("Error inserting triple");
        }
        for (i, definition_type_tag) in self.definition_type_tags.iter().enumerate() {
            let dtt_id = definition_type_tag.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("definitionTypeTag")?,
                &dtt_id).expect("Error inserting triple");
        }
        for (i, inflected_form_tag) in self.inflected_form_tags.iter().enumerate() {
            let inflected_form_tag_id = inflected_form_tag.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("inflectedFormTag")?,
                &inflected_form_tag_id).expect("Error inserting triple");
        }
        for (i, label_tag) in self.label_tags.iter().enumerate() {
            let label_tag_id = label_tag.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("labelTag")?,
                &label_tag_id).expect("Error inserting triple");
        }
        for (i, part_of_speech_tag) in self.part_of_speech_tags.iter().enumerate() {
            let part_of_speech_tag_id = part_of_speech_tag.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("partOfSpeechTag")?,
                &part_of_speech_tag_id).expect("Error inserting triple");
        }
        for (i, source_identity_tag) in self.source_identity_tags.iter().enumerate() {
            let source_identity_tag_id = source_identity_tag.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("sourceIdentityTag")?,
                &source_identity_tag_id).expect("Error inserting triple");
        }
        for (i, transcription_scheme_tag) in self.transcription_scheme_tags.iter().enumerate() {
            let transcription_scheme_tag_id = transcription_scheme_tag.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("transcriptionSchemeTag")?,
                &transcription_scheme_tag_id).expect("Error inserting triple");
        }
        for (i, relation) in self.relations.iter().enumerate() {
            let relation_id = relation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("relation")?,
                &relation_id).expect("Error inserting triple");
        }
        for (i, relation) in self.relation_types.iter().enumerate() {
            let relation_type_id = relation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("relationType")?,
                &relation_type_id).expect("Error inserting triple");
        }
        for  (i, etymon_language) in self.etymon_languages.iter().enumerate() {
            let etymon_language_id = etymon_language.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("etymonLanguage")?,
                &etymon_language_id).expect("Error inserting triple");
        }
        for (i, etymon_type) in self.etymon_types.iter().enumerate() {
            let etymon_type_id = etymon_type.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("etymonType")?,
                &etymon_type_id).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for LexicographicResource {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, LexicographicResource {
            title: get_zero_one_str(g, id, &dmlex.get("title")?)?,
            uri: get_zero_one_str(g, id, &dmlex.get("uri")?)?,
            lang_code: LangCode(get_one_str(g, id, &dmlex.get("langCode")?)?),
            entries: read_many(g, id, "entry", data, dmlex)?,
            translation_languages: read_many_str(g, id, "translationLanguage", data, dmlex)?,
            definition_type_tags: read_many(g, id, "definitionTypeTag", data, dmlex)?,
            inflected_form_tags: read_many(g, id, "inflectedFormTag", data, dmlex)?,
            label_tags: read_many(g, id, "labelTag", data, dmlex)?,
            label_type_tags: read_many(g, id, "labelTypeTag", data, dmlex)?,
            part_of_speech_tags: read_many(g, id, "partOfSpeechTag", data, dmlex)?,
            source_identity_tags: read_many(g, id, "sourceIdentityTag", data, dmlex)?,
            transcription_scheme_tags: read_many(g, id, "transcriptionSchemeTag", data, dmlex)?,
            relations: read_many(g, id, "relation", data, dmlex)?,
            relation_types: read_many(g, id, "relationType", data, dmlex)?,
            etymon_languages: read_many(g, id, "etymonLanguage", data, dmlex)?,
            etymon_types: read_many(g, id, "etymonType", data, dmlex)?,
        }))

    }
}


impl ToRDF for &Entry {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::make(&self.id, data)?;
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Entry")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("headword")?,
            &self.headword.as_literal()).expect("Error inserting triple");
        for (i, placeholder_marker) in self.placeholder_markers.iter().enumerate() {
            let placeholder_marker_id = placeholder_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("placeholderMarker")?,
                &placeholder_marker_id).expect("Error inserting triple");
        }
        if let Some(homograph_number) = &self.homograph_number {
            graph.insert(
                &id,
                &dmlex.get("homographNumber")?,
                &homograph_number.as_literal()).expect("Error inserting triple");
        }
        for (i, part_of_speech) in self.parts_of_speech.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("partOfSpeech")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &part_of_speech.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        for (i, pronunciation) in self.pronunciations.iter().enumerate() {
            let pronunciation_id = pronunciation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("pronunciation")?,
                &pronunciation_id).expect("Error inserting triple");
        }
        for (i, sense) in self.senses.iter().enumerate() {
            let sense_id = sense.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("sense")?,
                &sense_id).expect("Error inserting triple");
        }
        for (i, etymology) in self.etymologies.iter().enumerate() {
            let etymology_id = etymology.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("etymology")?,
                &etymology_id).expect("Error inserting triple");
        }

        Ok(id)
    }
}

impl FromRDF for Entry {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, Entry {
            id : get_id(id, data),
            headword: get_one_str(g, id, &dmlex.get("headword")?)?,
            placeholder_markers: read_many(g, id, "placeholderMarker", data, dmlex)?,
            homograph_number: get_zero_one_u32(g, id, &dmlex.get("homographNumber")?)?,
            parts_of_speech: read_tag(g, id, "partOfSpeech", data, dmlex)?,
            labels: read_tag(g, id, "label", data, dmlex)?,
            pronunciations: read_many(g, id, "pronunciation", data, dmlex)?,
            inflected_forms: read_many(g, id, "inflectedForm", data, dmlex)?,
            senses: read_many(g, id, "sense", data, dmlex)?,
            etymologies: read_many(g, id, "etymology", data, dmlex)?,
        }))
    }
}

impl ToRDF for &InflectedForm {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("InflectedForm")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        if let Some(tag) = &self.tag {
            graph.insert(
                &id,
                &dmlex.get("tag")?,
                &tag.as_literal()).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        for (i, pronunciation) in self.pronunciations.iter().enumerate() {
            let pronunciation_id = pronunciation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("pronunciation")?,
                &pronunciation_id).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for InflectedForm {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, InflectedForm {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            tag: get_zero_one_str(g, id, &dmlex.get("tag")?)?,
            labels: read_tag(g, id, "label", data, dmlex)?,
            pronunciations: read_many(g, id, "pronunciation", data, dmlex)?,
        }))
    }
}

impl ToRDF for &Sense {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::make(&self.id, data)?;
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Sense")?).expect("Error inserting triple");
        for indicator in &self.indicator {
            graph.insert(
                &id,
                &dmlex.get("indicator")?,
                &indicator.as_literal()).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        for (i, definition) in self.definitions.iter().enumerate() {
            let definition_id = definition.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("definition")?,
                &definition_id).expect("Error inserting triple");
        }
        for (i, example) in self.examples.iter().enumerate() {
            let example_id = example.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("example")?,
                &example_id).expect("Error inserting triple");
        }
        for (i, headword_explanation) in self.headword_explanations.iter().enumerate() {
            let headword_explanation_id = headword_explanation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("headwordExplanation")?,
                &headword_explanation_id).expect("Error inserting triple");
        }
        for (i, headword_translation) in self.headword_translations.iter().enumerate() {
            let headword_translation_id = headword_translation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("headwordTranslation")?,
                &headword_translation_id).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for Sense {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Sense {
            id: get_id(id, data),
            indicator: get_zero_one_str(g, id, &dmlex.get("indicator")?)?,
            labels: read_tag(g, id, "label", data, dmlex)?,
            definitions: read_many(g, id, "definition", data, dmlex)?,
            examples: read_many(g, id, "example", data, dmlex)?,
            headword_explanations: read_many(g, id, "headwordExplanation", data, dmlex)?,
            headword_translations: read_many(g, id, "headwordTranslation", data, dmlex)?,
        }))
    }
}

impl ToRDF for &Definition {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Definition")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        if let Some(definition_type) = &self.definition_type {
            graph.insert(
                &id,
                &dmlex.get("definitionType")?,
                &definition_type.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for Definition {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Definition {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            definition_type: get_zero_one_str(g, id, &dmlex.get("definitionType")?)?,
        }))
    }
}


impl ToRDF for &Pronunciation {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Pronunciation")?).expect("Error inserting triple");
        if let Some(sound_file) = &self.sound_file {
            graph.insert(
                &id,
                &dmlex.get("soundFile")?,
                &sound_file.as_literal()).expect("Error inserting triple");
        }
        for (i, transcription) in self.transcriptions.iter().enumerate() {
            let transcription_id = transcription.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("transcription")?,
                &transcription_id).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for Pronunciation {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Pronunciation {
            sound_file: get_zero_one_str(g, id, &dmlex.get("soundFile")?)?,
            transcriptions: read_many(g, id, "transcription", data, dmlex)?,
            labels: read_tag(g, id, "label", data, dmlex)?,
        }))
    }
}

impl ToRDF for &Transcription {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Transcription")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        if let Some(scheme) = &self.scheme {
            graph.insert(
                &id,
                &dmlex.get("scheme")?,
                &scheme.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for Transcription {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Transcription {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            scheme: get_zero_one_str(g, id, &dmlex.get("scheme")?)?,
        }))
    }
}

impl ToRDF for &Example {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Example")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        for (i, collocate_marker) in self.collocate_markers.iter().enumerate() {
            let collocate_marker_id = collocate_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("collocateMarker")?,
                &collocate_marker_id).expect("Error inserting triple");
        }
        for (i, headword_marker) in self.headword_markers.iter().enumerate() {
            let headword_marker_id = headword_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("headwordMarker")?,
                &headword_marker_id).expect("Error inserting triple");
        }
        if let Some(source_identity) = &self.source_identity {
            graph.insert(
                &id,
                &dmlex.get("sourceIdentity")?,
                &source_identity.as_literal()).expect("Error inserting triple");
        }
        if let Some(source_elaboration) = &self.source_elaboration {
            graph.insert(
&id,
                &dmlex.get("sourceElaboration")?,
                &source_elaboration.as_literal()).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        if let Some(sound_file) = &self.sound_file {
            graph.insert(
                &id,
                &dmlex.get("soundFile")?,
                &sound_file.as_literal()).expect("Error inserting triple");
        }
        for (i, example_translation) in self.example_translations.iter().enumerate() {
            let example_translation_id = example_translation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("exampleTranslation")?,
                &example_translation_id).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for Example {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Example {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            collocate_markers: read_many(g, id, "collocateMarker", data, dmlex)?,
            headword_markers: read_many(g, id, "headwordMarker", data, dmlex)?,
            source_identity: get_zero_one_str(g, id, &dmlex.get("sourceIdentity")?)?,
            source_elaboration: get_zero_one_str(g, id, &dmlex.get("sourceElaboration")?)?,
            labels: read_tag(g, id, "label", data, dmlex)?,
            sound_file: get_zero_one_str(g, id, &dmlex.get("soundFile")?)?,
            example_translations: read_many(g, id, "exampleTranslation", data, dmlex)?,
        }))
    }
}

impl ToRDF for &HeadwordTranslation {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("HeadwordTranslation")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        for (i, placeholder_marker) in self.placeholder_markers.iter().enumerate() {
            let placeholder_marker_id = placeholder_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("placeholderMarker")?,
                &placeholder_marker_id).expect("Error inserting triple");
        }
        if let Some(LangCode(lang_code)) = &self.lang_code {
            graph.insert(
                &id,
                &dmlex.get("langCode")?,
                &lang_code.as_literal()).expect("Error inserting triple");
        }
        for (i, part_of_speech) in self.parts_of_speech.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("partOfSpeech")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &part_of_speech.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        for (i, pronunciation) in self.pronunciations.iter().enumerate() {
            let pronunciation_id = pronunciation.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("pronunciation")?,
                &pronunciation_id).expect("Error inserting triple");
        }
        for (i, inflected_form) in self.inflected_forms.iter().enumerate() {
            let inflected_form_id = inflected_form.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("inflectedForm")?,
                &inflected_form_id).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for HeadwordTranslation {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, HeadwordTranslation {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            placeholder_markers: read_many(g, id, "placeholderMarker", data, dmlex)?,
            lang_code: get_zero_one_str(g, id, &dmlex.get("langCode")?)?.map(|x| LangCode(x)),
            parts_of_speech: read_tag(g, id, "partOfSpeech", data, dmlex)?,
            labels: read_tag(g, id, "label", data, dmlex)?,
            pronunciations: read_many(g, id, "pronunciation", data, dmlex)?,
            inflected_forms: read_many(g, id, "inflectedForm", data, dmlex)?,
        }))
    }
}

impl ToRDF for &HeadwordExplanation {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("HeadwordExplanation")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        for (i, collocate_marker) in self.collocate_markers.iter().enumerate() {
            let collocate_marker_id = collocate_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("collocateMarker")?,
                &collocate_marker_id).expect("Error inserting triple");
        }
        for (i, headword_marker) in self.headword_markers.iter().enumerate() {
            let headword_marker_id = headword_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("headwordMarker")?,
                &headword_marker_id).expect("Error inserting triple");
        }
        if let Some(LangCode(lang_code)) = &self.lang_code {
            graph.insert(
                &id,
                &dmlex.get("langCode")?,
                &lang_code.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}


impl FromRDF for HeadwordExplanation {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, HeadwordExplanation {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            collocate_markers: read_many(g, id, "collocateMarker", data, dmlex)?,
            headword_markers: read_many(g, id, "headwordMarker", data, dmlex)?,
            lang_code: get_zero_one_str(g, id, &dmlex.get("langCode")?)?.map(|x| LangCode(x)),
        }))
    }
}

impl ToRDF for &ExampleTranslation {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("ExampleTranslation")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        for (i, collocate_marker) in self.collocate_markers.iter().enumerate() {
            let collocate_marker_id = collocate_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("collocateMarker")?,
                &collocate_marker_id).expect("Error inserting triple");
        }
        for (i, headword_marker) in self.headword_markers.iter().enumerate() {
            let headword_marker_id = headword_marker.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("headwordMarker")?,
                &headword_marker_id).expect("Error inserting triple");
        }
        if let Some(LangCode(lang_code)) = &self.lang_code {
            graph.insert(
                &id,
                &dmlex.get("langCode")?,
                &lang_code.as_literal()).expect("Error inserting triple");
        }
        for (i, label) in self.labels.iter().enumerate() {
            let blank = URIOrBlank::gen();
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &blank).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("tag")?,
                &label.as_literal()).expect("Error inserting triple");
            graph.insert(
                &blank,
                &dmlex.get("listingOrder")?,
                &((i + 1) as u32).as_literal()).expect("Error inserting triple");
        }
        if let Some(sound_file) = &self.sound_file {
            graph.insert(
                &id,
                &dmlex.get("soundFile")?,
                &sound_file.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl FromRDF for ExampleTranslation {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, ExampleTranslation {
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            collocate_markers: read_many(g, id, "collocateMarker", data, dmlex)?,
            headword_markers: read_many(g, id, "headwordMarker", data, dmlex)?,
            lang_code: get_zero_one_str(g, id, &dmlex.get("langCode")?)?.map(|x| LangCode(x)),
            labels: read_tag(g, id, "label", data, dmlex)?,
            sound_file: get_zero_one_str(g, id, &dmlex.get("soundFile")?)?,
        }))
    }
}

impl ToRDF for &DefinitionTypeTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("DefinitionTypeTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        Ok(id)
    }
}


impl FromRDF for DefinitionTypeTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, DefinitionTypeTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            same_as: read_same_as(g, id, data)?,
        }))
    }
}


impl ToRDF for &InflectedFormTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("InflectedFormTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        if let Some(for_headwords) = &self.for_headwords {
            graph.insert(
                &id,
                &dmlex.get("forHeadwords")?,
                &for_headwords.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_translations) = &self.for_translations {
            graph.insert(
                &id,
                &dmlex.get("forTranslations")?,
                &for_translations.as_literal()).expect("Error inserting triple");
        }
        for for_language in &self.for_languages {
            graph.insert(
                &id,
                &dmlex.get("forLanguage")?,
                &for_language.as_literal()).expect("Error inserting triple");
        }
        for for_part_of_speech in &self.for_parts_of_speech {
            graph.insert(
                &id,
                &dmlex.get("forPartOfSpeech")?,
                &for_part_of_speech.as_literal()).expect("Error inserting triple");
        }

        Ok(id)
    }
}


impl FromRDF for InflectedFormTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, InflectedFormTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            same_as: read_same_as(g, id, data)?,
            for_headwords: get_zero_one_bool(g, id, &dmlex.get("forHeadwords")?)?,
            for_translations: get_zero_one_bool(g, id, &dmlex.get("forTranslations")?)?,
            for_languages: read_many_str(g, id, "forLanguage", data, dmlex)?,
            for_parts_of_speech: read_many_str(g, id, "forPartOfSpeech", data, dmlex)?,
        }))
    }
}

impl ToRDF for &LabelTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("LabelTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        if let Some(type_tag) = &self.type_tag {
            graph.insert(
                &id,
                &dmlex.get("typeTag")?,
                &type_tag.as_literal()).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        if let Some(for_headwords) = &self.for_headwords {
            graph.insert(
                &id,
                &dmlex.get("forHeadwords")?,
                &for_headwords.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_translations) = &self.for_translations {
            graph.insert(
                &id,
                &dmlex.get("forTranslations")?,
                &for_translations.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_collocates) = &self.for_collocates {
            graph.insert(
                &id,
                &dmlex.get("forCollocates")?,
                &for_collocates.as_literal()).expect("Error inserting triple");
        }
        for for_language in &self.for_languages {
            graph.insert(
                &id,
                &dmlex.get("forLanguage")?,
                &for_language.as_literal()).expect("Error inserting triple");
        }
        for for_part_of_speech in &self.for_parts_of_speech {
            graph.insert(
                &id,
                &dmlex.get("forPartOfSpeech")?,
                &for_part_of_speech.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for LabelTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((0, LabelTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            type_tag: get_zero_one_str(g, id, &dmlex.get("typeTag")?)?,
            same_as: read_same_as(g, id, data)?,
            for_headwords: get_zero_one_bool(g, id, &dmlex.get("forHeadwords")?)?,
            for_translations: get_zero_one_bool(g, id, &dmlex.get("forTranslations")?)?,
            for_collocates: get_zero_one_bool(g, id, &dmlex.get("forCollocates")?)?,
            for_languages: read_many_str(g, id, "forLanguage", data, dmlex)?,
            for_parts_of_speech: read_many_str(g, id, "forPartOfSpeech", data, dmlex)?,
        }))
    }
}


impl ToRDF for &LabelTypeTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("LabelTypeTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for LabelTypeTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((0, LabelTypeTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            same_as: read_same_as(g, id, data)?,
        }))
    }
}

impl ToRDF for &PartOfSpeechTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("PartOfSpeechTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        if let Some(for_headwords) = &self.for_headwords {
            graph.insert(
                &id,
                &dmlex.get("forHeadwords")?,
                &for_headwords.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_translations) = &self.for_translations {
            graph.insert(
                &id,
                &dmlex.get("forTranslations")?,
                &for_translations.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_etymology) = &self.for_etymology {
            graph.insert(
                &id,
                &dmlex.get("forEtymology")?,
                &for_etymology.as_literal()).expect("Error inserting triple");
        }
        for for_language in &self.for_languages {
            graph.insert(
                &id,
                &dmlex.get("forLanguage")?,
                &for_language.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for PartOfSpeechTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((0, PartOfSpeechTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            same_as: read_same_as(g, id, data)?,
            for_headwords: get_zero_one_bool(g, id, &dmlex.get("forHeadwords")?)?,
            for_translations: get_zero_one_bool(g, id, &dmlex.get("forTranslations")?)?,
            for_etymology: get_zero_one_bool(g, id, &dmlex.get("forEtymology")?)?,
            for_languages: read_many_str(g, id, "forLanguage", data, dmlex)?,
        }))
    }
}


impl ToRDF for &SourceIdentityTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("SourceIdentityTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for SourceIdentityTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((0, SourceIdentityTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            same_as: read_same_as(g, id, data)?,
        }))
    }
}

impl ToRDF for &TranscriptionSchemeTag {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("TranscriptionSchemeTag")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("tag")?,
            &self.tag.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_headwords) = &self.for_headwords {
            graph.insert(
                &id,
                &dmlex.get("forHeadwords")?,
                &for_headwords.as_literal()).expect("Error inserting triple");
        }
        if let Some(for_translations) = &self.for_translations {
            graph.insert(
                &id,
                &dmlex.get("forTranslations")?,
                &for_translations.as_literal()).expect("Error inserting triple");
        }
        for for_language in &self.for_languages {
            graph.insert(
                &id,
                &dmlex.get("forLanguage")?,
                &for_language.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for TranscriptionSchemeTag {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((0, TranscriptionSchemeTag {
            tag: get_one_str(g, id, &dmlex.get("tag")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            for_headwords: get_zero_one_bool(g, id, &dmlex.get("forHeadwords")?)?,
            for_translations: get_zero_one_bool(g, id, &dmlex.get("forTranslations")?)?,
            for_languages: read_many_str(g, id, "forLanguage", data, dmlex)?,
        }))
    }
}

impl ToRDF for &Relation {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Relation")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("type")?,
            &self._type.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for (i, member) in self.members.iter().enumerate() {
            let ref_ = member.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("member")?,
                &ref_).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for Relation {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Relation {
            _type: get_one_str(g, id, &dmlex.get("type")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            members: read_many(g, id, "member", data, dmlex)?,
        }))
    }
}


impl ToRDF for &Member {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &dmlex.get("ref")?,
            &self.ref_.as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Member")?).expect("Error inserting triple");
        if let Some(role) = &self.role {
            graph.insert(
                &id,
                &dmlex.get("role")?,
                &role.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("obverseListingOrder")?,
            &(self.obverse_listing_order as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for Member {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {

        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Member {
            ref_: get_one_str(g, id, &dmlex.get("ref")?)?,
            role: get_zero_one_str(g, id, &dmlex.get("role")?)?,
            obverse_listing_order: get_one_u32(g, id, &dmlex.get("obverseListingOrder")?)?,
        }))
    }
}

impl ToRDF for &RelationType {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("RelationType")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("type")?,
            &self._type.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        match self.scope_restriction {
            None => {},
            Some(ScopeRestriction::SameEntry) => {
                graph.insert(
                    &id,
                    &dmlex.get("scopeRestriction")?,
                    &dmlex.get("sameEntry")?).expect("Error inserting triple");
            },
            Some(ScopeRestriction::SameResource) => {
                graph.insert(
                    &id,
                    &dmlex.get("scopeRestriction")?,
                    &dmlex.get("sameResource")?).expect("Error inserting triple");
            },
            Some(ScopeRestriction::Any) => {
                graph.insert(
                    &id,
                    &dmlex.get("scopeRestriction")?,
                    &dmlex.get("any")?).expect("Error inserting triple");
            },
        }
        for (i, member_type) in self.member_types.iter().enumerate() {
            let member_type_id = member_type.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("memberType")?,
                &member_type_id).expect("Error inserting triple");
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for RelationType {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>,
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        let scope_restriction = match get_zero_one_str(g, id, &dmlex.get("scopeRestriction")?)? {
            None => None,
            Some(s) => match s.as_str() {
                "sameEntry" => Some(ScopeRestriction::SameEntry),
                "sameResource" => Some(ScopeRestriction::SameResource),
                "any" => Some(ScopeRestriction::Any),
                _ => return Err(RdfError::InvalidScopeRestriction(s))
            }
        };
        Ok((0, RelationType {
            _type: get_one_str(g, id, &dmlex.get("type")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            scope_restriction,
            member_types: read_many(g, id, "memberType", data, dmlex)?,
            same_as: read_same_as(g, id, data)?,
        }))
    }
}


impl ToRDF for &MemberType {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("MemberType")?).expect("Error inserting triple");
        if let Some(role) = &self.role {
            graph.insert(
                &id,
                &dmlex.get("role")?,
                &role.as_literal()).expect("Error inserting triple");
        }
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        match self._type {
            MemberTypeType::Sense => {
                graph.insert(
                    &id,
                    &dmlex.get("type")?,
                    &dmlex.get("sense")?).expect("Error inserting triple");
            },
            MemberTypeType::Collocate => {
                graph.insert(
                    &id,
                    &dmlex.get("type")?,
                    &dmlex.get("collocate")?).expect("Error inserting triple");
            },
            MemberTypeType::Entry => {
                graph.insert(
                    &id,
                    &dmlex.get("type")?,
                    &dmlex.get("entry")?).expect("Error inserting triple");
            },
        }
        if let Some(min) = &self.min {
            graph.insert(
                &id,
                &dmlex.get("min")?,
                &min.as_literal()).expect("Error inserting triple");
        }
        if let Some(max) = &self.max {
            graph.insert(
                &id,
                &dmlex.get("max")?,
                &max.as_literal()).expect("Error inserting triple");
        }
        match self.hint {
            None => {},
            Some(Hint::Embed) => {
                graph.insert(
                    &id,
                    &dmlex.get("hint")?,
                    &dmlex.get("embed")?).expect("Error inserting triple");
            },
            Some(Hint::Navigate) => {
                graph.insert(
                    &id,
                    &dmlex.get("hint")?,
                    &dmlex.get("navigate")?).expect("Error inserting triple");
            },
            Some(Hint::None) => {
                graph.insert(
                    &id,
                    &dmlex.get("hint")?,
                    &dmlex.get("none")?).expect("Error inserting triple");
            },
        }
        for same_as in &self.same_as {
            graph.insert(
                &id,
                &owl::sameAs,
                &Iri::new(same_as)?).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for MemberType {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        let _type = match get_one_str(g, id, &dmlex.get("type")?)?.as_str() {
            "sense" => MemberTypeType::Sense,
            "collocate" => MemberTypeType::Collocate,
            "entry" => MemberTypeType::Entry,
            s => return Err(RdfError::InvalidMemberType(s.to_string()))
        };
        let hint = match get_zero_one_str(g, id, &dmlex.get("hint")?)? {
            None => None,
            Some(s) => match s.as_str() {
                "embed" => Some(Hint::Embed),
                "navigate" => Some(Hint::Navigate),
                "none" => Some(Hint::None),
                s => return Err(RdfError::InvalidHint(s.to_string()))
            }
        };
        Ok((0, MemberType {
            role: get_zero_one_str(g, id, &dmlex.get("role")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            _type,
            min: get_zero_one_u32(g, id, &dmlex.get("min")?)?,
            max: get_zero_one_u32(g, id, &dmlex.get("max")?)?,
            hint,
            same_as: read_same_as(g, id, data)?,
        }))
    }
}

impl ToRDF for &Marker {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &dmlex.get("startIndex")?,
            &(self.start_index as u32).as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("endIndex")?,
            &(self.end_index as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for Marker {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, Marker {
            start_index: get_one_usize(g, id, &dmlex.get("startIndex")?)?,
            end_index: get_one_usize(g, id, &dmlex.get("endIndex")?)?,
        }))
    }
}

impl ToRDF for &CollocateMarker {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::make(&self.id, data)?;
        graph.insert(
            &id,
            &dmlex.get("startIndex")?,
            &(self.start_index as u32).as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("endIndex")?,
            &(self.end_index as u32).as_literal()).expect("Error inserting triple");
        if let Some(lemma) = &self.lemma {
            graph.insert(
                &id,
                &dmlex.get("lemma")?,
                &lemma.as_literal()).expect("Error inserting triple");
        }
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for CollocateMarker {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, CollocateMarker {
            id: get_id(id, data),
            start_index: get_one_usize(g, id, &dmlex.get("startIndex")?)?,
            end_index: get_one_usize(g, id, &dmlex.get("endIndex")?)?,
            lemma: get_zero_one_str(g, id, &dmlex.get("lemma")?)?,
            labels: read_many_str(g, id, "label", data, dmlex)?,
        }))
    }
}

impl ToRDF for &Etymology {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Etymology")?).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        for (i, etymon) in self.etymons.iter().enumerate() {
            let etymon_id = etymon.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("etymon")?,
                &etymon_id).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for Etymology {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Etymology {
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
            etymons: read_many(g, id, "etymon", data, dmlex)?,
        }))
    }
}


impl ToRDF for &Etymon {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("Etymon")?).expect("Error inserting triple");
        if let Some(when) = &self.when {
            graph.insert(
                &id,
                &dmlex.get("when")?,
                &when.as_literal()).expect("Error inserting triple");
        }
        if let Some(_type) = &self._type {
            graph.insert(
                &id,
                &dmlex.get("type")?,
                &_type.as_literal()).expect("Error inserting triple");
        }
        if let Some(note) = &self.note {
            graph.insert(
                &id,
                &dmlex.get("note")?,
                &note.as_literal()).expect("Error inserting triple");
        }
        for (i, etymon_unit) in self.etymon_units.iter().enumerate() {
            let etymon_unit_id = etymon_unit.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("etymonUnit")?,
                &etymon_unit_id).expect("Error inserting triple");
        }
        if let Some(translation) = &self.translation {
            graph.insert(
                &id,
                &dmlex.get("translation")?,
                &translation.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for Etymon {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, Etymon {
            when: get_zero_one_str(g, id, &dmlex.get("when")?)?,
            _type: get_zero_one_str(g, id, &dmlex.get("type")?)?,
            note: get_zero_one_str(g, id, &dmlex.get("note")?)?,
            etymon_units: read_many(g, id, "etymonUnit", data, dmlex)?,
            translation: get_zero_one_str(g, id, &dmlex.get("translation")?)?,
        }))
    }
}

impl ToRDF for &EtymonUnit {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("EtymonUnit")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("langCode")?,
            &self.lang_code.0.as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("text")?,
            &self.text.as_literal()).expect("Error inserting triple");
        if let Some(reconstructed) = &self.reconstructed {
            graph.insert(
                &id,
                &dmlex.get("reconstructed")?,
                &reconstructed.as_literal()).expect("Error inserting triple");
        }
        for part_of_speech in &self.parts_of_speech {
            graph.insert(
                &id,
                &dmlex.get("partOfSpeech")?,
                &part_of_speech.as_literal()).expect("Error inserting triple");
        }
        if let Some(translation) = &self.translation {
            graph.insert(
                &id,
                &dmlex.get("translation")?,
                &translation.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &((index + 1) as u32).as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl FromRDF for EtymonUnit {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((get_one_usize(g, id, &dmlex.get("listingOrder")?)?, EtymonUnit {
            lang_code: LangCode(get_one_str(g, id, &dmlex.get("langCode")?)?),
            text: get_one_str(g, id, &dmlex.get("text")?)?,
            reconstructed: get_zero_one_bool(g, id, &dmlex.get("reconstructed")?)?,
            parts_of_speech: read_many_str(g, id, "partOfSpeech", _data, dmlex)?,
            translation: get_zero_one_str(g, id, &dmlex.get("translation")?)?,
        }))
    }
}

impl ToRDF for &EtymonType {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("EtymonType")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("type")?,
            &self._type.as_literal()).expect("Error inserting triple");
        if let Some(description) = &self.description {
            graph.insert(
                &id,
                &dmlex.get("description")?,
                &description.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for EtymonType {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, EtymonType {
            _type: get_one_str(g, id, &dmlex.get("type")?)?,
            description: get_zero_one_str(g, id, &dmlex.get("description")?)?,
        }))
    }
}


impl ToRDF for &EtymonLanguage {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &rdf::type_,
            &dmlex.get("EtymonLanguage")?).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("langCode")?,
            &self.lang_code.0.as_literal()).expect("Error inserting triple");
        if let Some(display_name) = &self.display_name {
            graph.insert(
                &id,
                &dmlex.get("displayName")?,
                &display_name.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
    }
}

impl FromRDF for EtymonLanguage {
    fn from_rdf<G : Graph, T1 : AsRef<str>, T2: AsRef<str>>(id : &Term<String>, 
        g : &G, dmlex: &Namespace<T1>, _data: &Namespace<T2>) -> Result<(usize, Self)> where Self : Sized {
        Ok((0, EtymonLanguage {
            lang_code: LangCode(get_one_str(g, id, &dmlex.get("langCode")?)?),
            display_name: get_zero_one_str(g, id, &dmlex.get("displayName")?)?,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum URIOrBlank<'s> {
    URI(SimpleIri<'s>),
    Blank(BlankNode<String>),
}

impl URIOrBlank<'_> {
    fn gen<'s>() -> URIOrBlank<'s> {
        URIOrBlank::Blank(BlankNode::new(
                rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(20)
                .map(char::from)
                .collect::<String>()).unwrap())
    }

    fn make<'s, T: AsRef<str>>(s : &'s Option<String>, data : &'s Namespace<T>) -> Result<URIOrBlank<'s>> {
        if let Some(s) = s {
            Ok(URIOrBlank::URI(data.get(s)?))
        } else {
            Ok(URIOrBlank::gen())
        }
    }

}

fn get_id<T: AsRef<str>>(term : &Term<String>, data : &Namespace<T>) -> Option<String> {
    match term {
        Term::Iri(s) => {
            let s = s.value();
            let d = data.to_string();
            if s.len() > d.len() {
                if &s[..d.len()] == d {
                    Some(s[d.len()..].to_string())
                } else {
                    Some(s.to_string())
                }
            } else {
                Some(s.to_string())
            }

        }
        _ => None
    }
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

fn get_zero_one_str<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<Option<String>> {
    let mut iter = g.triples_with_sp(subj, prop);
    if let Some(triple) = iter.next() {
        let t = triple.unwrap();
        let obj = t.o();
        if obj.kind() == TermKind::Literal {
            Ok(Some(obj.value_raw().0.to_string()))
        } else {
            Err(RdfError::LiteralExpected(format!("{:?}", subj), format!("{:?}", prop)))
        }
    } else {
        Ok(None)
    }
}

fn get_zero_one_u32<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<Option<u32>> {
    match get_zero_one_str(g, subj, prop) {
        Ok(Some(s)) => Ok(Some(s.parse::<u32>()?)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

//
//fn get_zero_one_usize<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<Option<usize>> {
//    match get_zero_one_str(g, subj, prop) {
//        Ok(Some(s)) => Ok(Some(s.parse::<usize>()?)),
//        Ok(None) => Ok(None),
//        Err(e) => Err(e),
//    }
//}
//

fn get_zero_one_bool<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<Option<bool>> {
    match get_zero_one_str(g, subj, prop) {
        Ok(Some(s)) => Ok(Some(s.to_lowercase() == "true" || s == "1" || s == "yes" || s == "y")),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

fn get_one_str<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<String> {
    let mut iter = g.triples_with_sp(subj, prop);
    if let Some(triple) = iter.next() {
        let t = triple.unwrap();
        let obj = t.o();
        if obj.kind() == TermKind::Literal {
            Ok(obj.value_raw().0.to_string())
        } else {
            Err(RdfError::LiteralExpected(format!("{:?}", subj), format!("{:?}", prop)))
        }
    } else {
        Err(RdfError::MissingTriple(format!("{:?}", subj), format!("{:?}", prop)))
    }
}

fn get_one_usize<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<usize> {
    match get_one_str(g, subj, prop) {
        Ok(s) => Ok(s.parse::<usize>()?),
        Err(e) => Err(e),
    }
}


fn get_one_u32<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<u32> {
    match get_one_str(g, subj, prop) {
        Ok(s) => Ok(s.parse::<u32>()?),
        Err(e) => Err(e),
    }
}

fn get_many_str<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<Vec<String>> {
    let mut iter = g.triples_with_sp(subj, prop);
    let mut result = Vec::new();
    while let Some(triple) = iter.next() {
        let t = triple.unwrap();
        let obj = t.o();
        if obj.kind() == TermKind::Literal {
            result.push(obj.value_raw().0.to_string());
        } else {
            return Err(RdfError::LiteralExpected(format!("{:?}", subj), format!("{:?}", prop)))
        }
    }
    Ok(result)
}

fn get_many<G : Graph, S : TTerm + std::fmt::Debug, P : TTerm + std::fmt::Debug>(g : &G, subj : &S, prop : &P) -> Result<Vec<Term<String>>> {
    let mut iter = g.triples_with_sp(subj, prop);
    let mut result = Vec::new();
    while let Some(triple) = iter.next() {
        let t = triple.unwrap();
        let obj = t.o();
        if obj.kind() == TermKind::Iri {
            result.push(Term::copy(obj));
        } else if obj.kind() == TermKind::BlankNode {
            result.push(Term::copy(obj));
        } else {
            return Err(RdfError::ValueExpected(format!("{:?}", subj), format!("{:?}", prop)))
        }
    }
    Ok(result)
}

fn read_many<E : FromRDF, G : Graph, T1: AsRef<str>, T2: AsRef<str>>
    (g : &G, id : &Term<String>, prop : &str, data : &Namespace<T1>, dmlex : &Namespace<T2>) -> Result<Vec<E>> {
    let mut elems = Vec::new();
    for elem_id in get_many(g, id, &dmlex.get(prop)?)? {
        let (listing_order, entry) = E::from_rdf(&elem_id, g, dmlex, data)?;
        elems.push((listing_order, entry));
    }
    elems.sort_by_key(|(listing_order, _)| *listing_order);
    Ok(elems.into_iter().map(|(_, entry)| entry).collect())
}

fn read_many_str<G : Graph, T1: AsRef<str>, T2: AsRef<str>>
    (g : &G, id : &Term<String>, prop : &str, _data : &Namespace<T1>, dmlex : &Namespace<T2>) -> Result<Vec<String>> {
    get_many_str(g, id, &dmlex.get(prop)?)
}

fn read_same_as<G : Graph, T1: AsRef<str>>
    (g : &G, subj : &Term<String>, _data : &Namespace<T1>) -> Result<Vec<String>> {
    let mut iter = g.triples_with_sp(subj, &owl::sameAs);
    let mut result = Vec::new();
    while let Some(triple) = iter.next() {
        let t = triple.unwrap();
        let obj = t.o();
        if obj.kind() == TermKind::Iri {
            result.push(obj.value_raw().0.to_string());
        } else {
            return Err(RdfError::LiteralExpected(format!("{:?}", subj), format!("owl:sameAs")))
        }
    }
    Ok(result)
}


fn read_tag<G : Graph, T1: AsRef<str>, T2: AsRef<str>>
    (g : &G, id : &Term<String>, prop : &str, _data : &Namespace<T1>, dmlex : &Namespace<T2>) -> Result<Vec<String>> {
    let mut elems = Vec::new();
    for elem_id in get_many(g, id, &dmlex.get(prop)?)? {
        let tag = get_one_str(g, &elem_id, &dmlex.get("tag")?)?;
        let listing_order = get_one_usize(g, &elem_id, &dmlex.get("listingOrder")?)?;
        elems.push((listing_order, tag));
    }
    elems.sort_by_key(|(listing_order, _)| *listing_order);
    Ok(elems.into_iter().map(|(_, tag)| tag).collect())
}

#[derive(Error, Debug)]
pub enum RdfError {
    #[error("Invalid IRI: {0}")]
    InvalidIRI(#[from] sophia::term::iri::error::InvalidIri),
    #[error("Term Error: {0}")]
    TermError(#[from] sophia::term::TermError),
    #[error("Literal expected as value of {0} ={1}=>")]
    LiteralExpected(String, String),
    #[error("Missing triple for {0} ={1}=>")]
    MissingTriple(String, String),
    #[error("Value expected as object of {0} ={1}=>")]
    ValueExpected(String, String),
    #[error("Expected Integer but found: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Invalid scope restriction: {0}")]
    InvalidScopeRestriction(String),
    #[error("Invalid member type: {0}")]
    InvalidMemberType(String),
    #[error("Invalid hint: {0}")]
    InvalidHint(String),
    #[error("No resource of type dmlex:LexicographicResource in the graph")]
    MissingLexicographicResource,
    #[error("No resource of type dmlex:Entry in the graph")]
    MissingEntry,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use sophia::triple::stream::TripleSource;
    use sophia::graph::inmem::LightGraph;

    fn test_read_rdf_lexicon(fname : &str) {
        let mut buf_read = BufReader::new(File::open(fname).unwrap());
        let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples().unwrap();
        let data = Namespace::new("file:").unwrap();
        read_lexicographic_resource(&graph, &data).unwrap();
    }

    fn test_read_rdf_entry(fname : &str) {
        let mut buf_read = BufReader::new(File::open(fname).unwrap());
        let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples().unwrap();
        let data = Namespace::new("file:").unwrap();
        read_entry(&graph, &data).unwrap();
    }


    #[test]
    fn test_read_rdf_0() {
        test_read_rdf_lexicon("examples/0.rdf");
    }

    #[test]
    fn test_read_rdf_1() {
        test_read_rdf_entry("examples/1.rdf");
    }

    #[test]
    fn test_read_rdf_2() {
        test_read_rdf_entry("examples/2.rdf");
    }

    #[test]
    fn test_read_rdf_3() {
        test_read_rdf_entry("examples/3.rdf");
    }

    #[test]
    fn test_read_rdf_4() {
        test_read_rdf_entry("examples/4.rdf");
    }

    #[test]
    fn test_read_rdf_5() {
        test_read_rdf_lexicon("examples/5.rdf");
    }

    #[test]
    fn test_read_rdf_6() {
        test_read_rdf_lexicon("examples/6.rdf");
    }

    #[test]
    fn test_read_rdf_7() {
        test_read_rdf_lexicon("examples/7.rdf");
    }

    #[test]
    fn test_read_rdf_8() {
        test_read_rdf_entry("examples/8.rdf");
    }

    #[test]
    fn test_read_rdf_9() {
        test_read_rdf_entry("examples/9.rdf");
    }

    #[test]
    fn test_read_rdf_10() {
        test_read_rdf_lexicon("examples/10.rdf");
    }

    #[test]
    fn test_read_rdf_11() {
        test_read_rdf_entry("examples/11.rdf");
    }

    #[test]
    fn test_read_rdf_12() {
        test_read_rdf_lexicon("examples/12.rdf");
    }

    #[test]
    fn test_read_rdf_13() {
        test_read_rdf_lexicon("examples/13.rdf");
    }

    #[test]
    fn test_read_rdf_14() {
        test_read_rdf_lexicon("examples/14.rdf");
    }

    #[test]
    fn test_read_rdf_15() {
        test_read_rdf_lexicon("examples/15.rdf");
    }

    #[test]
    fn test_read_rdf_16() {
        test_read_rdf_lexicon("examples/16.rdf");
    }

    #[test]
    fn test_read_rdf_17() {
        test_read_rdf_lexicon("examples/17.rdf");
    }

    #[test]
    fn test_read_rdf_18() {
        test_read_rdf_lexicon("examples/18.rdf");
    }

    #[test]
    fn test_read_rdf_19() {
        test_read_rdf_entry("examples/19.rdf");
    }

    #[test]
    fn test_read_rdf_20() {
        test_read_rdf_entry("examples/20.rdf");
    }

    #[test]
    fn test_read_rdf_21() {
        test_read_rdf_entry("examples/21.rdf");
    }

    #[test]
    fn test_read_rdf_22() {
        test_read_rdf_entry("examples/22.rdf");
    }

    #[test]
    fn test_read_rdf_23() {
        test_read_rdf_entry("examples/23.rdf");
    }

    fn test_round_trip_rdf_lexicon(fname : &str) {
        let mut buf_read = BufReader::new(File::open(fname).unwrap());
        let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples().unwrap();
        let data = Namespace::new("file:").unwrap();
        let lexicon = read_lexicographic_resource(&graph, &data).unwrap();
        let mut graph2 = LightGraph::new();
        let dmlex = Namespace::new(DMLEX).expect("DMLEX namespace is invalid");
        lexicon.to_rdf(&mut graph2, &data, &dmlex, 0).unwrap();
        assert_eq!(graph.triples().count(), graph2.triples().count());
    }

    fn test_round_trip_rdf_entry(fname : &str) {
        let mut buf_read = BufReader::new(File::open(fname).unwrap());
        let graph : LightGraph = sophia::parser::turtle::parse_bufread(&mut buf_read).collect_triples().unwrap();
        let data = Namespace::new("file:").unwrap();
        let lexicon = read_entry(&graph, &data).unwrap();
        let mut graph2 = LightGraph::new();
        let dmlex = Namespace::new(DMLEX).expect("DMLEX namespace is invalid");
        (&lexicon).to_rdf(&mut graph2, &data, &dmlex, 0).unwrap();
        assert_eq!(graph.triples().count(), graph2.triples().count());
    }


    #[test]
    fn test_round_trip_rdf_0() {
        test_round_trip_rdf_lexicon("examples/0.rdf");
    }

    #[test]
    fn test_round_trip_rdf_1() {
        test_round_trip_rdf_entry("examples/1.rdf");
    }

    #[test]
    fn test_round_trip_rdf_2() {
        test_round_trip_rdf_entry("examples/2.rdf");
    }

    #[test]
    fn test_round_trip_rdf_3() {
        test_round_trip_rdf_entry("examples/3.rdf");
    }

    #[test]
    fn test_round_trip_rdf_4() {
        test_round_trip_rdf_entry("examples/4.rdf");
    }

    #[test]
    fn test_round_trip_rdf_5() {
        test_round_trip_rdf_lexicon("examples/5.rdf");
    }

    #[test]
    fn test_round_trip_rdf_6() {
        test_round_trip_rdf_lexicon("examples/6.rdf");
    }

    #[test]
    fn test_round_trip_rdf_7() {
        test_round_trip_rdf_lexicon("examples/7.rdf");
    }

    #[test]
    fn test_round_trip_rdf_8() {
        test_round_trip_rdf_entry("examples/8.rdf");
    }

    #[test]
    fn test_round_trip_rdf_9() {
        test_round_trip_rdf_entry("examples/9.rdf");
    }

    #[test]
    fn test_round_trip_rdf_10() {
        test_round_trip_rdf_lexicon("examples/10.rdf");
    }

    #[test]
    fn test_round_trip_rdf_11() {
        test_round_trip_rdf_entry("examples/11.rdf");
    }

    #[test]
    fn test_round_trip_rdf_12() {
        test_round_trip_rdf_lexicon("examples/12.rdf");
    }

    #[test]
    fn test_round_trip_rdf_13() {
        test_round_trip_rdf_lexicon("examples/13.rdf");
    }

    #[test]
    fn test_round_trip_rdf_14() {
        test_round_trip_rdf_lexicon("examples/14.rdf");
    }

    #[test]
    fn test_round_trip_rdf_15() {
        test_round_trip_rdf_lexicon("examples/15.rdf");
    }

    #[test]
    fn test_round_trip_rdf_16() {
        test_round_trip_rdf_lexicon("examples/16.rdf");
    }

    #[test]
    fn test_round_trip_rdf_17() {
        test_round_trip_rdf_lexicon("examples/17.rdf");
    }

    #[test]
    fn test_round_trip_rdf_18() {
        test_round_trip_rdf_lexicon("examples/18.rdf");
    }

    #[test]
    fn test_round_trip_rdf_19() {
        test_round_trip_rdf_entry("examples/19.rdf");
    }

    #[test]
    fn test_round_trip_rdf_20() {
        test_round_trip_rdf_entry("examples/20.rdf");
    }

    #[test]
    fn test_round_trip_rdf_21() {
        test_round_trip_rdf_entry("examples/21.rdf");
    }

    #[test]
    fn test_round_trip_rdf_22() {
        test_round_trip_rdf_entry("examples/22.rdf");
    }

    #[test]
    fn test_round_trip_rdf_23() {
        test_round_trip_rdf_entry("examples/23.rdf");
    }

    #[test]
    fn test_round_trip_rdf_24() {
        test_round_trip_rdf_lexicon("examples/24.rdf");
    }


 
}
