use xml::writer::{EventWriter, XmlEvent};
use crate::model::*;
use std::io::Write;

pub trait WriteXML {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error>;
}

impl WriteXML for &LexicographicResource {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("lexicographicResource");
        if let Some(uri) = &self.uri {
            e = e.attr("uri", uri);
        }
        e = e.attr("langCode", self.lang_code.0.as_str());
        writer.write(e)?;
        if let Some(title) = &self.title {
            writer.write(XmlEvent::start_element("title"))?;
            writer.write(XmlEvent::characters(title))?;
            writer.write(XmlEvent::end_element())?;
        }
        for entry in self.entries.iter() {
            entry.write_xml(writer)?;
        }
        for translation_language in self.translation_languages.iter() {
            writer.write(XmlEvent::start_element("translationLanguage")
                .attr("langCode", &translation_language))?;
            writer.write(XmlEvent::end_element())?;
        }
        for definition_type_tag in self.definition_type_tags.iter() {
            definition_type_tag.write_xml(writer)?;
        }
        for inflected_form_tag in self.inflected_form_tags.iter() {
            inflected_form_tag.write_xml(writer)?;
        }
        for label_tag in self.label_tags.iter() {
            label_tag.write_xml(writer)?;
        }
        for label_type_tag in self.label_type_tags.iter() {
            label_type_tag.write_xml(writer)?;
        }
        for part_of_speech_tag in self.part_of_speech_tags.iter() {
            part_of_speech_tag.write_xml(writer)?;
        }
        for source_identity_tag in self.source_identity_tags.iter() {
            source_identity_tag.write_xml(writer)?;
        }
        for transcription_scheme_tag in self.transcription_scheme_tags.iter() {
            transcription_scheme_tag.write_xml(writer)?;
        }
        for relation in self.relations.iter() {
            relation.write_xml(writer)?;
        }
        for relation_type in self.relation_types.iter() {
            relation_type.write_xml(writer)?;
        }
        for etymon_language in self.etymon_languages.iter() {
            etymon_language.write_xml(writer)?;
        }
        for etymon_type in self.etymon_types.iter() {
            etymon_type.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
} 

impl WriteXML for &Entry {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("entry");
        if let Some(id) = &self.id {
            e = e.attr("id", id);
        }
        if let Some(homograph_number) = &self.homograph_number {
            let x = homograph_number.to_string();
            e = e.attr("homographNumber", &x);
            writer.write(e)?;
        } else {
            writer.write(e)?;
        }
        writer.write(XmlEvent::start_element("headword"))?;
        write_headword_string(writer, &self.headword, &self.placeholder_markers)?;
        writer.write(XmlEvent::end_element())?;
        for part_of_speech in self.parts_of_speech.iter() {
            writer.write(XmlEvent::start_element("partOfSpeech")
                .attr("tag", part_of_speech))?;
            writer.write(XmlEvent::end_element())?;
        }
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        for pronunciation in self.pronunciations.iter() {
            pronunciation.write_xml(writer)?;
        }
        for inflected_form in self.inflected_forms.iter() {
            inflected_form.write_xml(writer)?;
        }
        for sense in self.senses.iter() {
            sense.write_xml(writer)?;
        }
        for etymology in self.etymologies.iter() {
            etymology.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &InflectedForm {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("inflectedForm");
        if let Some(tag) = &self.tag {
            e = e.attr("tag", tag);
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::characters(&self.text))?;
        writer.write(XmlEvent::end_element())?;
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        for pronunciation in self.pronunciations.iter() {
            pronunciation.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Sense {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("sense");
        if let Some(id) = &self.id {
            e = e.attr("id", id);
        }
        writer.write(e)?;
        for indicator in self.indicator.iter() {
            writer.write(XmlEvent::start_element("indicator"))?;
            writer.write(XmlEvent::characters(indicator))?;
            writer.write(XmlEvent::end_element())?;
        }
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        for definition in self.definitions.iter() {
            definition.write_xml(writer)?;
        }
        for example in self.examples.iter() {
            example.write_xml(writer)?;
        }
        for headword_explanation in self.headword_explanations.iter() {
            headword_explanation.write_xml(writer)?;
        }
        for headword_translation in self.headword_translations.iter() {
            headword_translation.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Definition {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("definition");
        if let Some(definition_type) = &self.definition_type {
            e = e.attr("definitionType", definition_type);
        }
        writer.write(e)?;
        writer.write(XmlEvent::characters(&self.text))?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Pronunciation {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("pronunciation");
        if let Some(sound_file) = &self.sound_file {
            e = e.attr("soundFile", sound_file);
        }
        writer.write(e)?;
        for transcription in self.transcriptions.iter() {
            transcription.write_xml(writer)?;
        }
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Transcription {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("transcription");
        if let Some(scheme) = &self.scheme {
            e = e.attr("scheme", scheme);
        }
        writer.write(e)?;
        writer.write(XmlEvent::characters(&self.text))?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Example {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("example");
        if let Some(ref source_identity) = &self.source_identity {
            e = e.attr("sourceIdentity", source_identity);
        }
        if let Some(ref source_elaboration) = &self.source_elaboration {
            e = e.attr("sourceElaboration", source_elaboration);
        }
        if let Some(sound_file) = &self.sound_file {
            e = e.attr("soundFile", sound_file);
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("text"))?;
        write_text_string(writer, &self.text, &self.headword_markers, &self.collocate_markers)?;
        writer.write(XmlEvent::end_element())?;
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        for example_translation in self.example_translations.iter() {
            example_translation.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for HeadwordTranslation {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("headwordTranslation");
        if let Some(ref lang_code) = &self.lang_code {
            e = e.attr("langCode", &lang_code.0);
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("text"))?;
        write_headword_string(writer, &self.text, &self.placeholder_markers)?;
        writer.write(XmlEvent::end_element())?;
        for part_of_speech in self.parts_of_speech.iter() {
            writer.write(XmlEvent::start_element("partOfSpeech")
                .attr("tag", part_of_speech))?;
            writer.write(XmlEvent::end_element())?;
        }
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        for pronunciation in self.pronunciations.iter() {
            pronunciation.write_xml(writer)?;
        }
        for inflected_form in self.inflected_forms.iter() {
            inflected_form.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &HeadwordExplanation {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("headwordExplanation");
        if let Some(lang_code) = &self.lang_code {
            e = e.attr("langCode", &lang_code.0);
        }
        writer.write(e)?;
        //writer.write(XmlEvent::start_element("text"))?;
        write_text_string(writer, &self.text, &self.headword_markers, &self.collocate_markers)?;
        //writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &ExampleTranslation {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("exampleTranslation");
        if let Some(ref lang_code) = &self.lang_code {
            e = e.attr("langCode", &lang_code.0);
        }
        if let Some(ref sound_file) = &self.sound_file {
            e = e.attr("soundFile", sound_file);
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("text"))?;
        write_text_string(writer, &self.text, &self.headword_markers, &self.collocate_markers)?;
        writer.write(XmlEvent::end_element())?;
        for label in self.labels.iter() {
            writer.write(XmlEvent::start_element("label")
                .attr("tag", label))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &DefinitionTypeTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("definitionTypeTag");
        e = e.attr("tag", &self.tag);
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &InflectedFormTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("inflectedFormTag");
        e = e.attr("tag", &self.tag);
        if let Some(for_headwords) = &self.for_headwords {
            e = e.attr("forHeadwords", if *for_headwords { "true" } else { "false" });
        }
        if let Some(for_translations) = &self.for_translations {
            e = e.attr("forTranslations", if *for_translations { "true" } else { "false" });
        }
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for for_language in self.for_languages.iter() {
            writer.write(XmlEvent::start_element("forLanguage")
                .attr("langCode", &for_language))?;
            writer.write(XmlEvent::end_element())?;
        }
        for for_part_of_speech in self.for_parts_of_speech.iter() {
            writer.write(XmlEvent::start_element("forPartOfSpeech")
                .attr("tag", for_part_of_speech))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &LabelTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("labelTag")
            .attr("tag", &self.tag);
        if let Some(type_tag) = &self.type_tag {
            e = e.attr("typeTag", type_tag);
        }
        if let Some(for_headwords) = &self.for_headwords {
            e = e.attr("forHeadwords", if *for_headwords { "true" } else { "false" });
        }
        if let Some(for_translations) = &self.for_translations {
            e = e.attr("forTranslations", if *for_translations { "true" } else { "false" });
        }
        if let Some(for_collocates) = &self.for_collocates {
            e = e.attr("forCollocates", if *for_collocates { "true" } else { "false" });
        }
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for for_language in self.for_languages.iter() {
            writer.write(XmlEvent::start_element("forLanguage")
                .attr("langCode", &for_language))?;
            writer.write(XmlEvent::end_element())?;
        }
        for for_part_of_speech in self.for_parts_of_speech.iter() {
            writer.write(XmlEvent::start_element("forPartOfSpeech")
                .attr("tag", for_part_of_speech))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &LabelTypeTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let e = XmlEvent::start_element("labelTypeTag")
            .attr("tag", &self.tag);
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
Ok(())
    }
}

impl WriteXML for &PartOfSpeechTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("partOfSpeechTag")
            .attr("tag", &self.tag);
        if let Some(for_headwords) = &self.for_headwords {
            e = e.attr("forHeadwords", if *for_headwords { "true" } else { "false" });
        }
        if let Some(for_translations) = &self.for_translations {
            e = e.attr("forTranslations", if *for_translations { "true" } else { "false" });
        }
        if let Some(for_etymology) = &self.for_etymology {
            e = e.attr("forEtymology", if *for_etymology { "true" } else { "false" });
        }
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for for_language in self.for_languages.iter() {
            writer.write(XmlEvent::start_element("forLanguage")
                .attr("langCode", &for_language))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &SourceIdentityTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let e = XmlEvent::start_element("sourceIdentityTag")
            .attr("tag", &self.tag);
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &TranscriptionSchemeTag {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("transcriptionSchemeTag")
            .attr("tag", &self.tag);
        if let Some(for_headwords) = &self.for_headwords {
            e = e.attr("forHeadwords", if *for_headwords { "true" } else { "false" });
        }
        if let Some(for_translations) = &self.for_translations {
            e = e.attr("forTranslations", if *for_translations { "true" } else { "false" });
        }
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for for_language in self.for_languages.iter() {
            writer.write(XmlEvent::start_element("forLanguage")
                .attr("langCode", &for_language))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl  WriteXML for &Relation {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let e = XmlEvent::start_element("relation")
.attr("type", &self._type);
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for member in self.members.iter() {
            member.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Member {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("member")
            .attr("ref", &self.ref_);
        if let Some(role) = &self.role {
            e = e.attr("role", role);
        }
        let x = self.obverse_listing_order.to_string();
        e = e.attr("obverseListingOrder", &x);
        writer.write(e)?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &RelationType {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("relationType")
            .attr("type", &self._type);
        match self.scope_restriction {
            Some(ScopeRestriction::SameEntry) => e = e.attr("scopeRestriction", "sameEntry"),
            Some(ScopeRestriction::SameResource) => e = e.attr("scopeRestriction", "sameResource"),
            Some(ScopeRestriction::Any) => e = e.attr("scopeRestriction", "any"),
            None => ()
        }
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for member_type in self.member_types.iter() {
            member_type.write_xml(writer)?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &MemberType {
    #[allow(unused_assignments)]
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("memberType");
        if let Some(role) = &self.role {
            e = e.attr("role", role);
        }
        match self._type {
            MemberTypeType::Sense => e = e.attr("type", "sense"),
            MemberTypeType::Entry => e = e.attr("type", "entry"),
            MemberTypeType::Collocate => e = e.attr("type", "collocate"),
        }
        let mut min_str = String::new();
        if let Some(min) = self.min {
            min_str = min.to_string();
            e = e.attr("min", &min_str);
        }
        let mut max_str = String::new();
        if let Some(max) = self.max {
            max_str = max.to_string();
            e = e.attr("max", &max_str);
        }
        match self.hint {
            Some(Hint::Embed) => e = e.attr("hint", "embed"),
            Some(Hint::Navigate) => e = e.attr("hint", "navigate"),
            Some(Hint::None) => e = e.attr("hint", "none"),
            None => ()
        }
        writer.write(e)?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for same_as in self.same_as.iter() {
            writer.write(XmlEvent::start_element("sameAs")
                .attr("uri", same_as))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Etymology {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        writer.write(XmlEvent::start_element("etymology"))?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        for etymon in self.etymons.iter() {
            etymon.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &Etymon {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("etymon");
        if let Some(when) = &self.when {
            e = e.attr("when", when);
        }
        if let Some(_type) = &self._type {
            e = e.attr("type", _type);
        }
        writer.write(e)?;
        if let Some(note) = &self.note {
            writer.write(XmlEvent::start_element("note"))?;
            writer.write(XmlEvent::characters(note))?;
            writer.write(XmlEvent::end_element())?;
        }
        for etymon_units in self.etymon_units.iter() {
            etymon_units.write_xml(writer)?;
        }
        if let Some(translation) = &self.translation {
            writer.write(XmlEvent::start_element("translation"))?;
            writer.write(XmlEvent::characters(translation))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &EtymonUnit {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("etymonUnit")
            .attr("langCode", &self.lang_code.0);
        if let Some(reconstructed) = &self.reconstructed {
            e = e.attr("reconstructed", if *reconstructed { "true" } else { "false" });
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::characters(&self.text))?;
        writer.write(XmlEvent::end_element())?;
        for part_of_speech in self.parts_of_speech.iter() {
            writer.write(XmlEvent::start_element("partOfSpeech")
                .attr("tag", part_of_speech))?;
            writer.write(XmlEvent::end_element())?;
        }
        if let Some(translation) = &self.translation {
            writer.write(XmlEvent::start_element("translation"))?;
            writer.write(XmlEvent::characters(translation))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &EtymonType {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        writer.write(XmlEvent::start_element("etymonType")
            .attr("type", &self._type))?;
        if let Some(description) = &self.description {
            writer.write(XmlEvent::start_element("description"))?;
            writer.write(XmlEvent::characters(description))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &EtymonLanguage {
    fn write_xml<W : Write>(&self, writer: &mut EventWriter<&mut W>) -> Result<(), xml::writer::Error> {
        writer.write(XmlEvent::start_element("etymonLanguage")
            .attr("langCode", &self.lang_code.0))?;
        if let Some(display_name) = &self.display_name {
            writer.write(XmlEvent::start_element("displayName"))?;
            writer.write(XmlEvent::characters(display_name))?;
            writer.write(XmlEvent::end_element())?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

fn write_headword_string<W : Write>(writer: &mut EventWriter<&mut W>,
    string : &String, placeholder_markers : &Vec<Marker>) -> Result<(), xml::writer::Error> {
    let str_chars : Vec<char> = string.chars().collect();
    let mut i = 0;
    for placeholder_marker in placeholder_markers {
        if placeholder_marker.start_index < i {
            return Err(xml::writer::Error::Io(std::io::Error::new(std::io::ErrorKind::Other,
                "Placeholder markers not in order")));
        }
        if placeholder_marker.start_index > i {
            let s : String = str_chars[i..placeholder_marker.start_index].iter().collect();
            writer.write(XmlEvent::characters(&s))?;
        }
        writer.write(XmlEvent::start_element("placeholderMarker"))?;
        let s : String = str_chars[placeholder_marker.start_index..placeholder_marker.end_index].iter().collect();
        writer.write(XmlEvent::characters(&s))?;
        writer.write(XmlEvent::end_element())?;

        i = placeholder_marker.end_index;
    }
    if i < str_chars.len() {
        let s : String = str_chars[i..].iter().collect();
        writer.write(XmlEvent::characters(&s))?;
    }
    Ok(())
}

fn write_text_string<W : Write>(writer : &mut EventWriter<&mut W>,
    string : &String, headword_markers : &Vec<Marker>,
    collocate_markers : &Vec<CollocateMarker>) -> Result<(), xml::writer::Error>  {
    let str_chars : Vec<char> = string.chars().collect();
    let mut i = 0;
    let mut headword_markers_iter = headword_markers.iter().peekable();
    let mut collocate_markers_iter = collocate_markers.iter().peekable();
    while headword_markers_iter.peek().is_some() || 
        collocate_markers_iter.peek().is_some() {
        if headword_markers_iter.peek().is_none() ||
            (collocate_markers_iter.peek().is_some() &&
                collocate_markers_iter.peek().unwrap().start_index < 
                headword_markers_iter.peek().unwrap().start_index) {
            let collocate_marker = collocate_markers_iter.next().unwrap();
            if collocate_marker.start_index < i {
                return Err(xml::writer::Error::Io(std::io::Error::new(std::io::ErrorKind::Other,
                    "Collocate markers not in order")));
            }
            if collocate_marker.start_index > i {
                let s : String = str_chars[i..collocate_marker.start_index].iter().collect();
                writer.write(XmlEvent::characters(&s))?;
            }
            let mut e = XmlEvent::start_element("collocateMarker"); 
            if let Some(lemma) = &collocate_marker.lemma {
                e = e.attr("lemma", lemma);
            }
            if let Some(id) = &collocate_marker.id {
                e = e.attr("id", id);
            }
            writer.write(e)?;
            let s : String = str_chars[collocate_marker.start_index..collocate_marker.end_index].iter().collect();
            writer.write(XmlEvent::characters(&s))?;
            for label in collocate_marker.labels.iter() {
                writer.write(XmlEvent::start_element("label")
                    .attr("tag", label))?;
                writer.write(XmlEvent::end_element())?;
            }
            writer.write(XmlEvent::end_element())?;
            i = collocate_marker.end_index;
        } else {
            let headword_marker = headword_markers_iter.next().unwrap();
            if headword_marker.start_index < i {
                return Err(xml::writer::Error::Io(std::io::Error::new(std::io::ErrorKind::Other,
                    "Headword markers not in order")));
            }

            if headword_marker.start_index > i {
                let s : String = str_chars[i..headword_marker.start_index].iter().collect();
                writer.write(XmlEvent::characters(&s))?;
            }

            writer.write(XmlEvent::start_element("headwordMarker"))?;
            let s : String = str_chars[headword_marker.start_index..headword_marker.end_index].iter().collect();
            writer.write(XmlEvent::characters(&s))?;
            writer.write(XmlEvent::end_element())?;
            i = headword_marker.end_index;
        }
    }
    if i < str_chars.len() {
        let s : String = str_chars[i..].iter().collect();
        writer.write(XmlEvent::characters(&s))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headword_string() {
        let mut out = Vec::new();
        let mut writer = xml::EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(false)
            .create_writer(&mut out);
        write_headword_string(&mut writer,
            &"continue your studies".to_owned(),
            &vec![Marker { start_index: 9, end_index: 13 }]).unwrap();
        assert_eq!(String::from_utf8(out).unwrap(),
            "continue <placeholderMarker>your</placeholderMarker> studies");
    }
}


