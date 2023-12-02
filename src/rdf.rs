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

impl ToRDF for LexicographicResource {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::make(&self.id, data)?;
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
        for  (i, etymon_language) in self.etymon_language.iter().enumerate() {
            let etymon_language_id = etymon_language.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("etymonLanguage")?,
                &etymon_language_id).expect("Error inserting triple");
        }
        for (i, etymon_type) in self.etymon_type.iter().enumerate() {
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
        let title = get_zero_one_str(g, id, &dmlex.get("title")?)?;
        let uri = get_zero_one_str(g, id, &dmlex.get("uri")?)?;
        let lang_code = LangCode(get_one_str(g, id, &dmlex.get("langCode")?)?);

        let mut translation_languages = Vec::new();

        let mut definition_type_tags = Vec::new();

        let mut inflected_form_tags = Vec::new();

        let mut label_tags = Vec::new();

        let mut label_type_tags = Vec::new();

        let mut part_of_speech_tags = Vec::new();

        let mut source_identity_tags = Vec::new();

        let mut transcription_scheme_tags = Vec::new();

        let mut relations = Vec::new();

        let mut relation_types = Vec::new();

        let mut etymon_language = Vec::new();

        let mut etymon_type = Vec::new();

        Ok((0, LexicographicResource {
            id: get_id(id, data),
            title, 
            uri,
            lang_code,
            entries: read_many(g, id, "entry", data, dmlex)?,
            translation_languages,
            definition_type_tags,
            inflected_form_tags,
            label_tags,
            label_type_tags,
            part_of_speech_tags,
            source_identity_tags,
            transcription_scheme_tags,
            relations,
            relation_types,
            etymon_language,
            etymon_type
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
        for part_of_speech in &self.parts_of_speech {
            graph.insert(
                &id,
                &dmlex.get("partOfSpeech")?,
                &part_of_speech.as_literal()).expect("Error inserting triple");
        }
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
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
        for (i, etymology) in self.etymology.iter().enumerate() {
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
        let mut placeholder_markers = Vec::new();
        let mut homograph_number = None;
        let mut parts_of_speech = Vec::new();
        let mut labels = Vec::new();
        let mut pronunciations = Vec::new();
        let mut inflected_forms = Vec::new();
        let mut senses = Vec::new();
        let mut etymology = Vec::new();
        Ok((0, Entry {
            id : get_id(id, data),
            headword: get_one_str(g, id, &dmlex.get("headword")?)?,
            placeholder_markers,
            homograph_number,
            parts_of_speech,
            labels,
            pronunciations,
            inflected_forms,
            senses,
            etymology
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
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
        for part_of_speech in &self.parts_of_speech {
            graph.insert(
                &id,
                &dmlex.get("partOfSpeech")?,
                &part_of_speech.as_literal()).expect("Error inserting triple");
        }
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
        for label in &self.labels {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
            let member_id = member.to_rdf(graph, data, dmlex, i)?;
            graph.insert(
                &id,
                &dmlex.get("member")?,
                &member_id).expect("Error inserting triple");
        }
        graph.insert(
            &id,
            &dmlex.get("listingOrder")?,
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}


impl ToRDF for &Member {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::URI(data.get(&self.member_id)?);
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("obverseListingOrder")?,
            &self.obverse_listing_order.to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
        graph.insert(
            &id,
            &dmlex.get("role")?,
            &self.role.as_literal()).expect("Error inserting triple");
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


impl ToRDF for &Marker {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &dmlex.get("startIndex")?,
            &self.start_index.to_string().as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("endIndex")?,
            &self.end_index.to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
    }
}

impl ToRDF for &CollocateMarker {
    fn to_rdf<'a, G: MutableGraph, T1: AsRef<str>, T2: AsRef<str>>(&'a self, 
        graph: &mut G, _data : &'a Namespace<T1>, dmlex: &Namespace<T2>,
        _index : usize) -> 
        Result<URIOrBlank<'a>> {
        let id = URIOrBlank::gen();
        graph.insert(
            &id,
            &dmlex.get("startIndex")?,
            &self.start_index.to_string().as_literal()).expect("Error inserting triple");
        graph.insert(
            &id,
            &dmlex.get("endIndex")?,
            &self.end_index.to_string().as_literal()).expect("Error inserting triple");
        if let Some(lemma) = &self.lemma {
            graph.insert(
                &id,
                &dmlex.get("lemma")?,
                &lemma.as_literal()).expect("Error inserting triple");
        }
        for label in &self.label {
            graph.insert(
                &id,
                &dmlex.get("label")?,
                &label.as_literal()).expect("Error inserting triple");
        }
        Ok(id)
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
            &(index + 1).to_string().as_literal()).expect("Error inserting triple");
        Ok(id)
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
}
