use xml::writer::{EventWriter, XmlEvent};
use crate::model::*;

pub trait WriteXML {
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error>;
}

impl WriteXML for &LexicographicResource {
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("lexicographicResource");
        if let Some(id) = &self.id {
            e = e.attr("id", id);
        }
        if let Some(title) = &self.title {
            e = e.attr("title", title);
        }
        if let Some(uri) = &self.uri {
            e = e.attr("uri", uri);
        }
        e = e.attr("langCode", self.lang_code.0.as_str());
        writer.write(e)?;
        for entry in self.entries.iter() {
            entry.write_xml(writer)?;
        }
        for translation_language in self.translation_languages.iter() {
            translation_language.write_xml(writer)?;
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
        for etymon_language in self.etymon_language.iter() {
            etymon_language.write_xml(writer)?;
        }
        for etymon_type in self.etymon_type.iter() {
            etymon_type.write_xml(writer)?;
        }
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
} 

impl WriteXML for &Entry {
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("entry");
        if let Some(id) = &self.id {
            e = e.attr("id", id);
        }
        if let Some(homograph_number) = &self.homograph_number {
            e = e.attr("homographNumber", self.homograph_number.to_string());
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("headword"))?;
        writer.write(XmlEvent::characters(&self.headword))?;
        writer.write(XmlEvent::end_element())?;
        for part_of_speech in self.parts_of_speech.iter() {
            write.write(XmlEvent::start_element("partOfSpeech")
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("sense");
        if let Some(id) = &self.id {
            e = e.attr("id", id);
        }
        writer.write(e)?;
        for indicator in self.indicator.iter() {
            writer.write(XmlEvent::start_element("indicator"));
            writer.write(XmlEvent::characters(indicator));
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
        writer.write(XmlEvent::characters(&self.text))?;
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

impl WriteXML for HeadwordTranslation {
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("headwordTranslation");
        if let Some(ref lang_code) = &self.lang_code {
            e = e.attr("langCode", &lang_code.0);
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("headwordExplanation");
        if let Some(lang_code) = &self.lang_code {
            e = e.attr("langCode", &lang_code.0);
        }
        writer.write(e)?;
        writer.write(XmlEvent::start_element("text"))?;
        writer.write(XmlEvent::characters(&self.text))?;
        writer.write(XmlEvent::end_element())?;
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &ExampleTranslation {
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
        let mut e = XmlEvent::start_element("exampleTranslation");
        if let Some(ref lang_code) = &self.lang_code {
            e = e.attr("langCode", &lang_code.0);
        }
        if let Some(ref sound_file) = &self.sound_file {
            e = e.attr("soundFile", sound_file);
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
        writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}

impl WriteXML for &DefinitionTypeTag {
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
    fn write_xml(&self, writer: &mut EventWriter<&mut Vec<u8>>) -> Result<(), xml::writer::Error> {
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
