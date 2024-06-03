use std::io::Read;
use xml::reader::EventReader;
use crate::model::*;
use thiserror::Error;
use xml::reader::XmlEvent::{StartElement, EndElement, Characters, StartDocument, EndDocument, ProcessingInstruction, Comment, Whitespace, CData};
use xml::attribute::OwnedAttribute;
use xml::common::Position;
use regex::Regex;

pub fn read_xml<R: Read, E : XMLVisitor>(input: R, element_name : &'static str) -> std::result::Result<E, XMLErrorWithPosition> {
    let mut reader = EventReader::new_with_config(input, xml::ParserConfig::new()
        .trim_whitespace(false)
        .whitespace_to_characters(true));
    loop {
        match reader.next() {
            Ok(StartElement { name, attributes, .. }) => {
                if name.local_name == element_name {
                    return E::from_event_reader(&mut reader, attributes)
                        .map_err(|e| XMLErrorWithPosition::E(e, reader.position().row, reader.position().column));
                }
            },
            Ok(StartDocument { .. }) => { },
            Ok(ProcessingInstruction { .. }) => { },
            Ok(Comment(_)) => { },
            Ok(Whitespace(_)) => { },
            Ok(Characters(chars)) => { 
                return Err(XMLErrorWithPosition::E(
                        FromXMLError::UnexpectedCharacters(chars),
                        reader.position().row, reader.position().column));
            },
            Ok(CData(chars)) => { 
                return Err(XMLErrorWithPosition::E(
                        FromXMLError::UnexpectedCData(chars),
                        reader.position().row, reader.position().column));
            },
            Ok(EndElement { name }) => { 
                return Err(XMLErrorWithPosition::E(
                        FromXMLError::MismatchedEndElement(name.local_name),
                        reader.position().row, reader.position().column));
            },
            Ok(EndDocument { .. }) => { 
                return Err(XMLErrorWithPosition::E(
                        FromXMLError::UnexpectedEndDocument,
                        reader.position().row, reader.position().column));
            },
            Err(e) => { 
                return Err(XMLErrorWithPosition::E(
                        FromXMLError::XML(e),
                        reader.position().row, reader.position().column));
            },
        }
    }
}

type Result<T> = std::result::Result<T, FromXMLError>;

pub trait XMLVisitor : Default {
    fn from_event_reader<R: Read>(input: &mut EventReader<R>, mut attributes: Vec<OwnedAttribute>) -> Result<Self> {
        let mut visitor = Self::default();
        visitor.visit_attributes(&mut attributes)?;
        if attributes.len() > 0 {
            return Err(FromXMLError::UnexpectedAttributes(attributes.clone()));
        }
        loop {
            match input.next() {
                Ok(StartElement { name, attributes, .. }) => {
                    visitor.visit_start_element(&name.local_name, attributes, input)?;
                },
                Ok(Characters(characters)) => {
                    visitor.visit_characters(&characters)?;
                },
                Ok(CData(cdata)) => {
                    visitor.visit_characters(&cdata)?;
                },
                Ok(EndElement { name, .. }) => {
                    if name.local_name != visitor.name() {
                        return Err(FromXMLError::MismatchedEndElement(
                           name.local_name));
                    } else {
                        visitor.visit_end_element()?;
                        return Ok(visitor);
                    }
                },
                Ok(Whitespace(whitespace)) => {
                    visitor.visit_whitespace(&whitespace)?;
                },
                Ok(StartDocument { .. }) => {
                    return Err(FromXMLError::UnexpectedStartDocument);
                },
                Ok(EndDocument) => {
                    return Err(FromXMLError::UnexpectedEndDocument);
                },
                Ok(ProcessingInstruction { .. }) => {
                    return Err(FromXMLError::UnexpectedProcessingInstruction);
                },
                Ok(Comment(_)) => { },
                Err(e) => return Err(FromXMLError::XML(e)),
            }
        }
    }

    fn visit_attributes(&mut self, _attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        _attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        Err(FromXMLError::UnexpectedElement(name.to_string()))
    }

    fn visit_characters(&mut self, characters: &str) -> Result<()> {
        if characters.chars().all(|c| c.is_whitespace()) {
            return Ok(());
        } else {
            Err(FromXMLError::UnexpectedCharacters(characters.to_string()))
        }
    }

    fn visit_whitespace(&mut self, _whitespace: &str) -> Result<()> {
        Ok(())
    }

    fn visit_cdata(&mut self, cdata: &str) -> Result<()> {
        Err(FromXMLError::UnexpectedCData(cdata.to_string()))
    }

    fn visit_end_element(&mut self) -> Result<()> {
        Ok(())
    }

    fn name(&self) -> &'static str;
}

impl XMLVisitor for LexicographicResource {
    fn name(&self) -> &'static str {
        "lexicographicResource"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.uri = str_attr("uri", attributes);
        self.lang_code = LangCode(str_attr("langCode", attributes)
            .ok_or(FromXMLError::MissingAttribute("langCode"))?);
        self.title = str_attr("title", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "entry" => {
                self.entries.push(Entry::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "translationLanguage" => {
                self.translation_languages.push(lang_code_tag(reader, attributes)?);
                Ok(())
            },
            "definitionTypeTag" => {
                self.definition_type_tags.push(DefinitionTypeTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "inflectedFormTag" => {
                self.inflected_form_tags.push(InflectedFormTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "labelTag" => {
                self.label_tags.push(LabelTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "labelTypeTag" => {
                self.label_type_tags.push(LabelTypeTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "partOfSpeechTag" => {
                self.part_of_speech_tags.push(PartOfSpeechTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "sourceIdentityTag" => {
                self.source_identity_tags.push(SourceIdentityTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "transcriptionSchemeTag" => {
                self.transcription_scheme_tags.push(TranscriptionSchemeTag::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "relation" => {
                self.relations.push(Relation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "relationType" => {
                self.relation_types.push(RelationType::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "etymonLanguage" => {
                self.etymon_languages.push(EtymonLanguage::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "etymonType" => {
                self.etymon_types.push(EtymonType::from_event_reader(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Entry {
    fn name(&self) -> &'static str {
        "entry"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.id = str_attr("id", attributes);
        self.homograph_number = u32_attr("homographNumber", attributes)?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "headword" => {
                let (headword, markers) = headword_string(reader, attributes)?;
                self.headword = headword;
                self.placeholder_markers = markers;
                Ok(())
            },
            "partOfSpeech" => {
                self.parts_of_speech.push(tag(reader, attributes)?);
                Ok(())
            },
            "label" => {
                self.labels.push(tag(reader, attributes)?);
                Ok(())
            },
            "pronunciation" => {
                self.pronunciations.push(Pronunciation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "inflectedForm" => {
                self.inflected_forms.push(InflectedForm::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "sense" => {
                self.senses.push(Sense::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "etymology" => {
                self.etymologies.push(Etymology::from_event_reader(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for InflectedForm {
    fn name(&self) -> &'static str {
        "inflectedForm"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                self.text = text(reader, attributes)?;
                Ok(())
            },
            "label" => {
                self.labels.push(tag(reader, attributes)?);
                Ok(())
            },
            "pronunciation" => {
                self.pronunciations.push(Pronunciation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Sense {
    fn name(&self) -> &'static str {
        "sense"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.id = str_attr("id", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "indicator" => {
                self.indicator = Some(text(reader, attributes)?);
                Ok(())
            },
            "label" => {
                self.labels.push(tag(reader, attributes)?);
                Ok(())
            },
            "definition" => {
                self.definitions.push(Definition::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "example" => {
                self.examples.push(Example::from_event_reader(reader, attributes)?);
                Ok(())
            }, 
            "headwordExplanation" => {
                self.headword_explanations.push(HeadwordExplanation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "headwordTranslation" => {
                self.headword_translations.push(HeadwordTranslation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Definition {
    fn name(&self) -> &'static str {
        "definition"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.definition_type = str_attr("type", attributes);
        Ok(())
    }

    //fn visit_characters(&mut self, characters: &str) -> Result<()> {
    //    self.text.push_str(&characters);
    //    Ok(())
    //}
    fn visit_start_element<R: Read>(&mut self, name: &str, 
        _attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                let text = plain_string(reader)?;
                self.text = text;
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }

    fn visit_end_element(&mut self) -> Result<()> {
        self.text = self.text.trim().to_string();
        Ok(())
    }
}

impl XMLVisitor for Pronunciation {
    fn name(&self) -> &'static str {
        "pronunciation"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.sound_file = str_attr("soundFile", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "transcription" => {
                self.transcriptions.push(Transcription::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "label" => {
                self.labels.push(tag(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Transcription {
    fn name(&self) -> &'static str {
        "transcription"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.scheme = str_attr("scheme", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        _attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                self.text = plain_string(reader)?;
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }

    fn visit_end_element(&mut self) -> Result<()> {
        self.text = self.text.trim().to_string();
        Ok(())
    }

}

impl XMLVisitor for Example {
    fn name(&self) -> &'static str {
        "example"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.source_identity = str_attr("sourceIdentity", attributes);
        self.source_elaboration = str_attr("sourceElaboration", attributes);
        self.sound_file = str_attr("soundFile", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                let (text, markers, collocate_markers) = text_string(reader, attributes)?;
                self.text = text;
                self.headword_markers = markers;
                self.collocate_markers = collocate_markers;
                Ok(())
            },
            "label" => {
                self.labels.push(tag(reader, attributes)?);
                Ok(())
            },
            "exampleTranslation" => {
                self.example_translations.push(ExampleTranslation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for HeadwordTranslation {
    fn name(&self) -> &'static str {
        "headwordTranslation"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.lang_code = str_attr("langCode", attributes).map(|s| LangCode(s));
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                let (text, markers) = headword_string(reader, attributes)?;
                self.text = text;
                self.placeholder_markers = markers;
                Ok(())
            },
            "partOfSpeech" => {
                self.parts_of_speech.push(tag(reader, attributes)?);
                Ok(())
            },
            "label" => {
                self.labels.push(tag(reader, attributes)?);
                Ok(())
            },
            "pronunciation" => {
                self.pronunciations.push(Pronunciation::from_event_reader(reader, attributes)?);
                Ok(())
            },
            "inflectedForm" => {
                self.inflected_forms.push(InflectedForm::from_event_reader(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for HeadwordExplanation {
    fn name(&self) -> &'static str {
        "headwordExplanation"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.lang_code = str_attr("langCode", attributes).map(|s| LangCode(s));
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                let (text, markers, coll_markers) = text_string(reader, attributes)?;
                self.text = text;
                self.headword_markers = markers;
                self.collocate_markers = coll_markers;
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for ExampleTranslation {
    fn name(&self) -> &'static str {
        "exampleTranslation"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.lang_code = str_attr("langCode", attributes).map(|s| LangCode(s));
        self.sound_file = str_attr("soundFile", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                let (text, markers, coll_markers) = text_string(_reader, attributes)?;
                self.text = text;
                self.headword_markers = markers;
                self.collocate_markers = coll_markers;
                Ok(())
            },
            "label" => {
                self.labels.push(tag(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for DefinitionTypeTag {
    fn name(&self) -> &'static str {
        "definitionTypeTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for InflectedFormTag {
    fn name(&self) -> &'static str {
        "inflectedFormTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        self.for_= str_attr("for", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for LabelTag {
    fn name(&self) -> &'static str {
        "labelTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        self.for_= str_attr("for", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}
 
impl XMLVisitor for LabelTypeTag {
    fn name(&self) -> &'static str {
        "labelTypeTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for PartOfSpeechTag {
    fn name(&self) -> &'static str {
        "partOfSpeechTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        self.for_= str_attr("for", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}


impl XMLVisitor for SourceIdentityTag {
    fn name(&self) -> &'static str {
        "sourceIdentityTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for TranscriptionSchemeTag {
    fn name(&self) -> &'static str {
        "transcriptionSchemeTag"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.tag = str_attr("tag", attributes)
            .ok_or(FromXMLError::MissingAttribute("tag"))?;
        self.for_= str_attr("for", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }

}

impl XMLVisitor for Relation {
    fn name(&self) -> &'static str {
        "relation"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self._type = str_attr("type", attributes)
            .ok_or(FromXMLError::MissingAttribute("type"))?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(reader, attributes)?);
                Ok(())
            },
            "member" => {
                self.members.push(Member::from_event_reader(reader, attributes)?);
                Ok(())
            }
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Member {
    fn name(&self) -> &'static str {
        "member"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.ref_ = str_attr("ref", attributes)
            .ok_or(FromXMLError::MissingAttribute("ref"))?;
        self.role = str_attr("role", attributes);
        self.obverse_listing_order = u32_attr("obverseListingOrder", attributes)?;
        Ok(())
    }
}

impl XMLVisitor for RelationType {
    fn name(&self) -> &'static str {
        "relationType"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self._type = str_attr("type", attributes)
            .ok_or(FromXMLError::MissingAttribute("type"))?;
        self.scope_restriction = match str_attr("scopeRestriction", attributes) {
            Some(s) => {
                if s == "sameEntry" {
                    Some(ScopeRestriction::SameEntry)
                } else if s == "sameResource" {
                    Some(ScopeRestriction::SameResource)
                } else if s == "any" {
                    Some(ScopeRestriction::Any)
                } else {
                    return Err(FromXMLError::InvalidAttributeValue("scopeRestriction", s))
                }
            },
            None => None,
        };
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(_reader, attributes)?);
                Ok(())
            },
            "memberType" => {
                self.member_types.push(MemberType::from_event_reader(_reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for MemberType {
    fn name(&self) -> &'static str {
        "memberType"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.role = str_attr("role", attributes);
        self._type = match str_attr("type", attributes)
            .ok_or(FromXMLError::MissingAttribute("type"))? {
            s => {
                if s == "entry" {
                    MemberTypeType::Entry
                } else if s == "sense" {
                    MemberTypeType::Sense
                } else if s == "collocate" {
                    MemberTypeType::Collocate
                } else {
                    return Err(FromXMLError::InvalidAttributeValue("type", s))
                }
            }
        };
        self.min = u32_attr("min", attributes)?;
        self.max = u32_attr("max", attributes)?;
        self.hint = match str_attr("hint", attributes) {
            Some(s) => {
                if s == "embed" {
                    Some(Hint::Embed)
                } else if s == "navigate" {
                    Some(Hint::Navigate)
                } else if s == "none" {
                    Some(Hint::None)
                } else {
                    return Err(FromXMLError::InvalidAttributeValue("hint", s))
                }
            },
            None => None
        };

        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(_reader, attributes)?);
                Ok(())
            },
            "sameAs" => {
                self.same_as.push(uri(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Etymology {
    fn name(&self) -> &'static str {
        "etymology"
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(_reader, attributes)?);
                Ok(())
            },
            "etymon" => {
                self.etymons.push(Etymon::from_event_reader(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for Etymon {
    fn name(&self) -> &'static str {
        "etymon"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.when = str_attr("when", attributes);
        self._type = str_attr("type", attributes);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "note" => {
                self.note = Some(text(_reader, attributes)?);
                Ok(())
            },
            "etymonUnit" => {
                self.etymon_units.push(EtymonUnit::from_event_reader(_reader, attributes)?);
                Ok(())
            },
            "translation" => {
                self.translation = Some(text(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for EtymonUnit {
    fn name(&self) -> &'static str {
        "etymonUnit"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.lang_code = LangCode(str_attr("langCode", attributes)
            .ok_or(FromXMLError::MissingAttribute("langCode"))?);
        self.reconstructed = bool_attr("reconstructed", attributes)?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "text" => {
                self.text = text(reader, attributes)?;
                Ok(())
            },
            "partOfSpeech" => {
                self.parts_of_speech.push(tag(reader, attributes)?);
                Ok(())
            },
            "translation" => {
                self.translation = Some(text(reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for EtymonType {
    fn name(&self) -> &'static str {
        "etymonType"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self._type = str_attr("type", attributes)
            .ok_or(FromXMLError::MissingAttribute("type"))?;
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "description" => {
                self.description = Some(text(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

impl XMLVisitor for EtymonLanguage {
    fn name(&self) -> &'static str {
        "etymonLanguage"
    }

    fn visit_attributes(&mut self, attributes: &mut Vec<OwnedAttribute>) -> Result<()> {
        self.lang_code = LangCode(str_attr("langCode", attributes)
            .ok_or(FromXMLError::MissingAttribute("langCode"))?);
        Ok(())
    }

    fn visit_start_element<R: Read>(&mut self, name: &str, 
        attributes: Vec<OwnedAttribute>, _reader: &mut EventReader<R>) 
        -> Result<()> {
        match name {
            "displayName" => {
                self.display_name = Some(text(_reader, attributes)?);
                Ok(())
            },
            _ => Err(FromXMLError::UnexpectedElement(name.to_string())),
        }
    }
}

fn lang_code_tag<R: Read>(input: &mut EventReader<R>, mut attributes: Vec<OwnedAttribute>) -> Result<String> {
    let lang_code = str_attr("langCode", &mut attributes)
        .ok_or(FromXMLError::MissingAttribute("langCode"))?;
    if attributes.len() > 0 {
        return Err(FromXMLError::UnexpectedAttributes(attributes.clone()));
    }
    loop {
        match input.next() {
            Ok(StartElement { name, .. }) => {
                return Err(FromXMLError::UnexpectedElement(name.local_name));
            },
            Ok(Characters(characters)) => {
                return Err(FromXMLError::UnexpectedCharacters(characters));
            },
            Ok(CData(cdata)) => {
                return Err(FromXMLError::UnexpectedCData(cdata));
            },
            Ok(EndElement { .. }) => {
                return Ok(lang_code);
            },
            Ok(Whitespace(_)) => {
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

fn uri<R: Read>(input: &mut EventReader<R>, mut attributes: Vec<OwnedAttribute>) -> Result<String> {
    let uri = str_attr("uri", &mut attributes)
        .ok_or(FromXMLError::MissingAttribute("uri"))?;
    if attributes.len() > 0 {
        return Err(FromXMLError::UnexpectedAttributes(attributes));
    }
    loop {
        match input.next() {
            Ok(StartElement { name, .. }) => {
                return Err(FromXMLError::UnexpectedElement(name.local_name));
            },
            Ok(Characters(characters)) => {
                return Err(FromXMLError::UnexpectedCharacters(characters));
            },
            Ok(CData(cdata)) => {
                return Err(FromXMLError::UnexpectedCData(cdata));
            },
            Ok(EndElement { .. }) => {
                return Ok(uri);
            },
            Ok(Whitespace(_)) => {
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

fn tag<R: Read>(input: &mut EventReader<R>, mut attributes: Vec<OwnedAttribute>) -> Result<String> {
    let tag = str_attr("tag", &mut attributes)
        .ok_or(FromXMLError::MissingAttribute("tag"))?;
    if attributes.len() > 0 {
        return Err(FromXMLError::UnexpectedAttributes(attributes));
    }
    loop {
        match input.next() {
            Ok(StartElement { name, .. }) => {
                return Err(FromXMLError::UnexpectedElement(name.local_name));
            },
            Ok(Characters(characters)) => {
                return Err(FromXMLError::UnexpectedCharacters(characters));
            },
            Ok(CData(cdata)) => {
                return Err(FromXMLError::UnexpectedCData(cdata));
            },
            Ok(EndElement { .. }) => {
                return Ok(tag);
            },
            Ok(Whitespace(_)) => {
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

fn text<R: Read>(input: &mut EventReader<R>, attributes: Vec<OwnedAttribute>) -> Result<String> {
    let mut text = String::new();
    if attributes.len() > 0 {
        return Err(FromXMLError::UnexpectedAttributes(attributes));
    }
    loop {
        match input.next() {
            Ok(StartElement { name, .. }) => {
                return Err(FromXMLError::UnexpectedElement(name.local_name));
            },
            Ok(Characters(characters)) => {
                text.push_str(&characters);
            },
            Ok(CData(cdata)) => {
                text.push_str(&cdata);
            },
            Ok(EndElement { .. }) => {
                return Ok(text);
            },
            Ok(Whitespace(_)) => {
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

fn plain_string<R : Read>(input: &mut EventReader<R>) -> Result<String> {
    let mut text = String::new();
    loop {
        match input.next() {
            Ok(StartElement { name, .. }) => {
                return Err(FromXMLError::UnexpectedElement(name.local_name));
            },
            Ok(Characters(characters)) => {
                if text.is_empty() {
                    text.push_str(&ASCII_WHITESPACE.replace_all(&characters, " ").trim_start());
                } else {
                    text.push_str(&ASCII_WHITESPACE.replace_all(&characters, " "));
                }
            },
            Ok(CData(cdata)) => {
                text.push_str(&cdata);
            },
            Ok(EndElement { .. }) => {
                return Ok(text.trim_end().to_string());
            },
            Ok(Whitespace(_)) => {
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

fn headword_string<R: Read>(input: &mut EventReader<R>, attributes: Vec<OwnedAttribute>) -> Result<(String, Vec<Marker>)> {
    let mut headword = String::new();
    let mut markers = Vec::new();
    if attributes.len() > 0 {
        return Err(FromXMLError::UnexpectedAttributes(attributes));
    }
    loop {
        match input.next() {
            Ok(StartElement { name, .. }) => {
                if name.local_name == "placeholderMarker" {
                    markers.push(Marker {
                        start_index: headword.chars().count(),
                        end_index: 0,
                    });
                } else {
                    return Err(FromXMLError::UnexpectedElement(name.local_name));
                }
            },
            Ok(Characters(characters)) => {
                headword.push_str(&characters);
            },
            Ok(CData(cdata)) => {
                headword.push_str(&cdata);
            },
            Ok(EndElement { name, .. }) => {
                if name.local_name == "headword" || name.local_name == "text" {
                    if markers.len() > 0 && markers.last().unwrap().end_index == 0 {
                        return Err(FromXMLError::UnclosedPlaceholderMarker);
                    }
                    return Ok((headword.to_string(), markers));
                } else if name.local_name == "placeholderMarker" {
                    if markers.len() == 0 {
                        return Err(FromXMLError::MismatchedEndElement("placeholderMarker".to_string()));
                    }
                    markers.last_mut().unwrap().end_index = headword.chars().count();
                } else {
                    return Err(FromXMLError::MismatchedEndElement(name.local_name));
                }
            },
            Ok(Whitespace(s)) => {
                headword.push_str(&s);
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

lazy_static! {
    static ref ASCII_WHITESPACE: Regex = Regex::new(r"[ \t\n\r\v\f]+").unwrap();
}

fn text_string<R: Read>(input: &mut EventReader<R>, attributes: Vec<OwnedAttribute>) -> Result<(String, Vec<Marker>, Vec<CollocateMarker>)> {
    let mut headword = String::new();
    let mut markers = Vec::new();
    let mut collocate_markers = Vec::new();
    if attributes.len() > 0 {
        return Err(FromXMLError::UnexpectedAttributes(attributes));
    }
    loop {
        match input.next() {
            Ok(StartElement { name, mut attributes, .. }) => {
                if name.local_name == "headwordMarker" {
                    markers.push(Marker {
                        start_index: headword.chars().count(),
                        end_index: 0,
                    });
                } else if name.local_name == "collocateMarker" {
                    collocate_markers.push(CollocateMarker {
                        start_index: headword.chars().count(),
                        end_index: 0,
                        id: str_attr("id", &mut attributes),
                        lemma: str_attr("lemma", &mut attributes),
                        labels: Vec::new(),
                    });
                } else if name.local_name == "label" {
                    if collocate_markers.len() == 0 {
                        return Err(FromXMLError::UnexpectedElement(name.local_name));
                    }
                    collocate_markers.last_mut().unwrap().labels.push(str_attr("tag", &mut attributes).
                        ok_or(FromXMLError::MissingAttribute("tag"))?);
                } else {
                    return Err(FromXMLError::MismatchedEndElement(name.local_name));
                }
            },
            Ok(Characters(characters)) => {
                if headword.is_empty() {
                    headword.push_str(ASCII_WHITESPACE.replace_all(&characters,
                        " ").trim_start());
                } else {
                    headword.push_str(ASCII_WHITESPACE.replace_all(&characters,
                            " ").as_ref());
                }
            },
            Ok(CData(cdata)) => {
                headword.push_str(&cdata);
            },
            Ok(EndElement { name, .. }) => {
                if name.local_name == "text" || name.local_name == "headwordExplanation" {
                    if markers.len() > 0 && markers.last().unwrap().end_index == 0 {
                        return Err(FromXMLError::UnclosedPlaceholderMarker);
                    }
                    return Ok((headword.trim_end().to_string(),
                            markers, collocate_markers));
                } else if name.local_name == "headwordMarker" {
                    if markers.len() == 0 {
                        return Err(FromXMLError::MismatchedEndElement("headwordMarker".to_string()));
                    }
                    markers.last_mut().unwrap().end_index = headword.chars().count();
                } else if name.local_name == "collocateMarker" {
                    if collocate_markers.len() == 0 {
                        return Err(FromXMLError::MismatchedEndElement("collocateMarker".to_string()));
                    }
                    collocate_markers.last_mut().unwrap().end_index = headword.chars().count();
                } else if name.local_name == "label" {
                    // do nothing
                } else {
                    return Err(FromXMLError::UnexpectedElement(name.local_name));
                }
            },
            Ok(Whitespace(_)) => {
                if !headword.is_empty() {
                    headword.push_str(" ");
                }
            },
            Ok(StartDocument { .. }) => {
                return Err(FromXMLError::UnexpectedStartDocument);
            },
            Ok(EndDocument) => {
                return Err(FromXMLError::UnexpectedEndDocument);
            },
            Ok(ProcessingInstruction { .. }) => {
                return Err(FromXMLError::UnexpectedProcessingInstruction);
            },
            Ok(Comment(_)) => { },
            Err(e) => return Err(FromXMLError::XML(e)),
        }
    }
}

fn str_attr(name: &str, attributes: &mut Vec<OwnedAttribute>) -> Option<String> {
    for i in 0..attributes.len() {
        if attributes[i].name.local_name == name {
            return Some(attributes.remove(i).value);
        }
    }
    None
}

fn bool_attr(name: &str, attributes: &mut Vec<OwnedAttribute>) -> Result<Option<bool>> {
    for i in 0..attributes.len() {
        if attributes[i].name.local_name == name {
            let value = attributes.remove(i).value;
            if value == "true" {
                return Ok(Some(true))
            } else if value == "false" {
                return Ok(Some(false))
            } else {
                return Err(FromXMLError::InvalidBooleanAttribute(value))
            }
        }
    }
    Ok(None)
}

fn u32_attr(name: &str, attributes: &mut Vec<OwnedAttribute>) -> Result<Option<u32>> {
    for i in 0..attributes.len() {
        if attributes[i].name.local_name == name {
            let value = attributes.remove(i).value;
            return Ok(Some(value.parse()?));
        }
    }
    Ok(None)
}

#[derive(Debug, Error)]
pub enum XMLErrorWithPosition {
    #[error("{0} at row {1}, column {2}")]
    E(FromXMLError, u64, u64)
}

#[derive(Debug, Error)]
pub enum FromXMLError {
    #[error("XML error: {0}")]
    XML(#[from] xml::reader::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Integer error: {0}")]
    Integer(#[from] std::num::ParseIntError),
    #[error("Mismatched end element: {0}")]
    MismatchedEndElement(String),
    #[error("Unknown attribute: {0}")]
    UnknownAttribute(String),
    #[error("Missing attribute: {0}")]
    MissingAttribute(&'static str),
    #[error("Unexpected element: {0}")]
    UnexpectedElement(String),
    #[error("Unexpected characters: {0}")]
    UnexpectedCharacters(String),
    #[error("Unexpected cdata: {0}")]
    UnexpectedCData(String),
    #[error("Unexpected attributes: {0:?}")]
    UnexpectedAttributes(Vec<OwnedAttribute>),
    #[error("Unexpected start document")]
    UnexpectedStartDocument,
    #[error("Unexpected end document")]
    UnexpectedEndDocument,
    #[error("Unexpected processing instruction")]
    UnexpectedProcessingInstruction,
    #[error("Unclosed placeholder marker")]
    UnclosedPlaceholderMarker,
    #[error("Invalid boolean attribute: {0}")]
    InvalidBooleanAttribute(String),
    #[error("Invalid attribute value: {1} for {0}")]
    InvalidAttributeValue(&'static str, String),
    #[error("Missing element: {0}")]
    MissingElement(&'static str),
}
