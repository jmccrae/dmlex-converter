/// This module contains the data model for the lexicon.
use serde::Deserialize;
use crate::model::{Marker, CollocateMarker, LangCode};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LexicographicResource {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub uri: Option<String>,
    pub lang_code: LangCode,
    #[serde(default)]
    pub entry: Vec<Entry>,
    #[serde(default)]
    pub translation_language: Vec<TranslationLanguage>,
    #[serde(default)]
    pub definition_type_tag: Vec<DefinitionTypeTag>,
    #[serde(default)]
    pub inflected_form_tag: Vec<InflectedFormTag>,
    #[serde(default)]
    pub label_tag: Vec<LabelTag>,
    #[serde(default)]
    pub label_type_tag: Vec<LabelTypeTag>,
    #[serde(default)]
    pub part_of_speech_tag: Vec<PartOfSpeechTag>,
    #[serde(default)]
    pub source_identity_tag: Vec<SourceIdentityTag>,
    #[serde(default)]
    pub transcription_scheme_tag: Vec<TranscriptionSchemeTag>,
    #[serde(default)]
    pub relation: Vec<Relation>,
    #[serde(default)]
    pub relation_type: Vec<RelationType>,
    #[serde(default)]
    pub etymon_language: Vec<EtymonLanguage>,
    #[serde(default)]
    pub etymon_type: Vec<EtymonType>,
}

impl Into<crate::model::LexicographicResource> for LexicographicResource {
    fn into(self) -> crate::model::LexicographicResource {
        crate::model::LexicographicResource {
            id: self.id,
            title: self.title,
            uri: self.uri,
            lang_code: self.lang_code,
            entries: self.entry.into_iter().map(|x| x.into()).collect(),
            translation_languages: self.translation_language.into_iter().map(|x| x.lang_code.0).collect(),
            definition_type_tags: self.definition_type_tag.into_iter().map(|x| x.into()).collect(),
            inflected_form_tags: self.inflected_form_tag.into_iter().map(|x| x.into()).collect(),
            label_tags: self.label_tag.into_iter().map(|x| x.into()).collect(),
            label_type_tags: self.label_type_tag.into_iter().map(|x| x.into()).collect(),
            part_of_speech_tags: self.part_of_speech_tag.into_iter().map(|x| x.into()).collect(),
            source_identity_tags: self.source_identity_tag.into_iter().map(|x| x.into()).collect(),
            transcription_scheme_tags: self.transcription_scheme_tag.into_iter().map(|x| x.into()).collect(),
            relations: self.relation.into_iter().map(|x| x.into()).collect(),
            relation_types: self.relation_type.into_iter().map(|x| x.into()).collect(),
            etymon_language: self.etymon_language.into_iter().map(|x| x.into()).collect(),
            etymon_type: self.etymon_type.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Entry { 
    #[serde(default)]
    pub id: Option<String>,
    pub headword: HeadwordString,
    #[serde(default)]
    pub homograph_number: Option<u32>,
    #[serde(default)]
    pub part_of_speech: Vec<PartOfSpeech>,
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(default)]
    pub pronunciation: Vec<Pronunciation>,
    #[serde(default)]
    pub inflected_form: Vec<InflectedForm>,
    #[serde(default)]
    pub sense: Vec<Sense>,
    #[serde(default)]
    pub etymology: Vec<Etymology>,
}

impl Into<crate::model::Entry> for Entry {
    fn into(self) -> crate::model::Entry {
        let (headword, placeholder_markers) = self.headword.normalize();
        crate::model::Entry {
            id: self.id,
            headword, placeholder_markers,
            homograph_number: self.homograph_number,
            parts_of_speech: self.part_of_speech.into_iter().map(|x| x.tag).collect(),
            labels: self.label.into_iter().map(|x| x.tag).collect(),
            pronunciations: self.pronunciation.into_iter().map(|x| x.into()).collect(),
            inflected_forms: self.inflected_form.into_iter().map(|x| x.into()).collect(),
            senses: self.sense.into_iter().map(|x| x.into()).collect(),
            etymology: self.etymology.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeadwordString(pub Vec<HeadwordStringPart>);

impl HeadwordString {
    pub fn normalize(&self) -> (String, Vec<Marker>) {
        let mut normalized = String::new();
        let mut markers = Vec::new();
        let mut len = 0;
        for part in &self.0 {
            match part {
                HeadwordStringPart::Text(text) => {
                    normalized.push_str(text);
                    len += text.chars().count();
                },
                HeadwordStringPart::PlaceholderMarker(marker) => {
                    let marker_len = marker.chars().count();
                    markers.push(Marker {
                        start_index: len,
                        end_index: (len + marker_len)
                    });
                    normalized.push_str(marker);
                    len += marker_len;
                }
            }
        }
        (normalized, markers)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HeadwordStringPart {
    Text(String),
    PlaceholderMarker(String)
}
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct PartOfSpeech {
    pub tag: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InflectedForm {
    pub text: String,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(default)]
    pub pronunciation: Vec<Pronunciation>,
}

impl Into<crate::model::InflectedForm> for InflectedForm {
    fn into(self) -> crate::model::InflectedForm {
        crate::model::InflectedForm {
            text: self.text,
            tag: self.tag,
            labels: self.label.into_iter().map(|x| x.tag).collect(),
            pronunciations: self.pronunciation.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextString(pub Vec<TextStringPart>);

impl TextString {
    fn normalize(&self) -> (String, Vec<Marker>, Vec<CollocateMarker>) {
        let mut normalized = String::new();
        let mut markers = Vec::new();
        let mut collocate_markers = Vec::new();
        let mut len = 0;
        for part in &self.0 {
            match part {
                TextStringPart::Text(text) => {
                    normalized.push_str(text);
                    len += text.chars().count();
                },
                TextStringPart::HeadwordMarker(marker) => {
                    let marker_len = marker.chars().count();
                    markers.push(Marker {
                        start_index: len,
                        end_index: (len + marker_len)
                    });
                    normalized.push_str(marker);
                    len += marker_len;
                },
                TextStringPart::CollocateMarker(marker, lemma, label) => {
                    let marker_len = marker.chars().count();
                    collocate_markers.push(CollocateMarker {
                        start_index: len,
                        end_index: (len + marker_len),
                        lemma: lemma.clone(),
                        labels: label.into_iter().map(|x| x.tag.clone()).collect()
                    });
                    normalized.push_str(marker);
                    len += marker_len;
                }
            }
        }
        (normalized, markers, collocate_markers)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextStringPart {
    Text(String),
    HeadwordMarker(String),
    CollocateMarker(String, Option<String>, Vec<Label>)
}

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Sense {
    pub id: Option<String>,
    pub indicator: Vec<String>,
    pub label: Vec<Label>,
    pub definition: Vec<Definition>,
    pub example: Vec<Example>,
    pub headword_explanation: Vec<HeadwordExplanation>,
    pub headword_translation: Vec<HeadwordTranslation>,
}

impl Into<crate::model::Sense> for Sense {
    fn into(self) -> crate::model::Sense {
        crate::model::Sense {
            id: self.id,
            indicator: self.indicator,
            labels: self.label.into_iter().map(|x| x.tag).collect(),
            definitions: self.definition.into_iter().map(|x| x.into()).collect(),
            examples: self.example.into_iter().map(|x| x.into()).collect(),
            headword_explanations: self.headword_explanation.into_iter().map(|x| x.into()).collect(),
            headword_translations: self.headword_translation.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Definition {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(default)]
    pub definition_type: Option<String>
}

impl Into<crate::model::Definition> for Definition {
    fn into(self) -> crate::model::Definition {
        crate::model::Definition {
            text: self.text,
            definition_type: self.definition_type,
        }
    }
}   

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub tag: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Pronunciation {
    pub sound_file: Option<String>,
    pub transcription: Vec<Transcription>,
    pub label: Vec<Label>,
}

impl Into<crate::model::Pronunciation> for Pronunciation {
    fn into(self) -> crate::model::Pronunciation {
        crate::model::Pronunciation {
            sound_file: self.sound_file,
            transcriptions: self.transcription.into_iter().map(|x| x.into()).collect(),
            labels: self.label.into_iter().map(|x| x.tag).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Transcription {
    #[serde(alias = "$value")]
    pub text: String,
    #[serde(default)]
    pub scheme: Option<String>,
}

impl Into<crate::model::Transcription> for Transcription {
    fn into(self) -> crate::model::Transcription {
        crate::model::Transcription {
            text: self.text,
            scheme: self.scheme,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub text: TextString,
    #[serde(default)]
    pub source_identity: Option<String>,
    #[serde(default)]
    pub source_elaboration: Option<String>,
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(default)]
    pub example_translation: Vec<ExampleTranslation>,
}

impl Into<crate::model::Example> for Example {
    fn into(self) -> crate::model::Example {
        let (text, headword_markers, collocate_markers) = self.text.normalize();
        crate::model::Example {
            text, headword_markers, collocate_markers,
            source_identity: self.source_identity,
            source_elaboration: self.source_elaboration,
            labels: self.label.into_iter().map(|x| x.tag).collect(),
            sound_file: self.sound_file,
            example_translations: self.example_translation.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TranslationLanguage {
    pub lang_code: LangCode,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordTranslation {
    pub text: HeadwordString,
    #[serde(default)]
    pub lang_code: Option<LangCode>,
    #[serde(default)]
    pub part_of_speech: Vec<PartOfSpeech>,
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(default)]
    pub pronunciation: Vec<Pronunciation>,
    #[serde(default)]
    pub inflected_form: Vec<InflectedForm>,
}

impl Into<crate::model::HeadwordTranslation> for HeadwordTranslation {
    fn into(self) -> crate::model::HeadwordTranslation {
        let (text, placeholder_markers) = self.text.normalize();
        crate::model::HeadwordTranslation {
            text, placeholder_markers,
            lang_code: self.lang_code,
            parts_of_speech: self.part_of_speech.into_iter().map(|x| x.tag).collect(),
            labels: self.label.into_iter().map(|x| x.tag).collect(),
            pronunciations: self.pronunciation.into_iter().map(|x| x.into()).collect(),
            inflected_forms: self.inflected_form.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordExplanation {
    #[serde(alias = "$value")]
    pub text: TextString,
    #[serde(default)]
    pub lang_code: Option<LangCode>,
 }

impl Into<crate::model::HeadwordExplanation> for HeadwordExplanation {
    fn into(self) -> crate::model::HeadwordExplanation {
        let (text, headword_markers, collocate_markers) = self.text.normalize();
        crate::model::HeadwordExplanation {
            text, headword_markers, collocate_markers,
            lang_code: self.lang_code
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExampleTranslation {
    pub text: TextString,
    #[serde(default)]
    pub lang_code: Option<LangCode>,
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(default)]
    pub sound_file: Option<String>,
}

impl Into<crate::model::ExampleTranslation> for ExampleTranslation {
    fn into(self) -> crate::model::ExampleTranslation {
        let (text, headword_markers, collocate_markers) = self.text.normalize();
        crate::model::ExampleTranslation {
            text, headword_markers, collocate_markers,
            lang_code: self.lang_code,
            labels: self.label.into_iter().map(|x| x.tag).collect(),
            sound_file: self.sound_file,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DefinitionTypeTag {
    pub tag: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
}

impl Into<crate::model::DefinitionTypeTag> for DefinitionTypeTag {
    fn into(self) -> crate::model::DefinitionTypeTag {
        crate::model::DefinitionTypeTag {
            tag: self.tag,
            description: self.description,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InflectedFormTag {
    pub tag: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(default)]
    pub for_language: Vec<ForLanguage>,
    #[serde(default)]
    pub for_part_of_speech: Vec<ForPartOfSpeech>,
}

impl Into<crate::model::InflectedFormTag> for InflectedFormTag {
    fn into(self) -> crate::model::InflectedFormTag {
        crate::model::InflectedFormTag {
            tag: self.tag,
            description: self.description,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
            for_headwords: self.for_headwords,
            for_translations: self.for_translations,
            for_languages: self.for_language.into_iter().map(|x| x.lang_code.0).collect(),
            for_parts_of_speech: self.for_part_of_speech.into_iter().map(|x| x.tag).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LabelTag {
    pub tag: String,
    #[serde(default)]
    pub type_tag: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(default)]
    pub for_collocates: Option<bool>,
    #[serde(default)]
    pub for_language: Vec<ForLanguage>,
    #[serde(default)]
    pub for_parts_of_speech: Vec<ForPartOfSpeech>,
}

impl Into<crate::model::LabelTag> for LabelTag {
    fn into(self) -> crate::model::LabelTag {
        crate::model::LabelTag {
            tag: self.tag,
            type_tag: self.type_tag,
            description: self.description,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
            for_headwords: self.for_headwords,
            for_translations: self.for_translations,
            for_collocates: self.for_collocates,
            for_languages: self.for_language.into_iter().map(|x| x.lang_code.0).collect(),
            for_parts_of_speech: self.for_parts_of_speech.into_iter().map(|x| x.tag).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LabelTypeTag {
    pub tag: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
}

impl Into<crate::model::LabelTypeTag> for LabelTypeTag {
    fn into(self) -> crate::model::LabelTypeTag {
        crate::model::LabelTypeTag {
            tag: self.tag,
            description: self.description,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PartOfSpeechTag {
    pub tag: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(default)]
    pub for_etymology: Option<bool>,
    #[serde(default)]
    pub for_languages: Vec<ForLanguage>,
}

impl Into<crate::model::PartOfSpeechTag> for PartOfSpeechTag {
    fn into(self) -> crate::model::PartOfSpeechTag {
        crate::model::PartOfSpeechTag {
            tag: self.tag,
            description: self.description,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
            for_headwords: self.for_headwords,
            for_translations: self.for_translations,
            for_etymology: self.for_etymology,
            for_languages: self.for_languages.into_iter().map(|x| x.lang_code.0).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SourceIdentityTag {
    pub tag: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
}

impl Into<crate::model::SourceIdentityTag> for SourceIdentityTag {
    fn into(self) -> crate::model::SourceIdentityTag {
        crate::model::SourceIdentityTag {
            tag: self.tag,
            description: self.description,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TranscriptionSchemeTag {
    pub tag: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(default)]
    pub for_languags: Vec<ForLanguage>,
}

impl Into<crate::model::TranscriptionSchemeTag> for TranscriptionSchemeTag {
    fn into(self) -> crate::model::TranscriptionSchemeTag {
        crate::model::TranscriptionSchemeTag {
            tag: self.tag,
            description: self.description,
            for_headwords: self.for_headwords,
            for_translations: self.for_translations,
            for_languages: self.for_languags.into_iter().map(|x| x.lang_code.0).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ForLanguage {
    pub lang_code: LangCode,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ForPartOfSpeech {
    pub tag: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Relation {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub member: Vec<Member>,
}

impl Into<crate::model::Relation> for Relation {
    fn into(self) -> crate::model::Relation {
        crate::model::Relation {
            _type: self._type,
            description: self.description,
            members: self.member.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Member {
    #[serde(rename = "memberID")]
    pub member_id: String,
    #[serde(default)]
    pub role: Option<String>,
    pub obverse_listing_order: u32,
}

impl Into<crate::model::Member> for Member {
    fn into(self) -> crate::model::Member {
        crate::model::Member {
            member_id: self.member_id,
            role: self.role,
            obverse_listing_order: self.obverse_listing_order,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RelationType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub scope_restriction: Option<crate::model::ScopeRestriction>,
    #[serde(default)]
    pub member_type: Vec<MemberType>,
    #[serde(default)]
    pub same_as: Vec<SameAs>,
}

impl Into<crate::model::RelationType> for RelationType {
    fn into(self) -> crate::model::RelationType {
        crate::model::RelationType {
            _type: self._type,
            description: self.description,
            scope_restriction: self.scope_restriction,
            member_types: self.member_type.into_iter().map(|x| x.into()).collect(),
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MemberType {
    pub role: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub _type: crate::model::MemberTypeType,
    #[serde(default)]
    pub min: Option<u32>,
    #[serde(default)]
    pub max: Option<u32>,
    #[serde(default)]
    pub hint: Option<crate::model::Hint>,
    #[serde(default)]
    pub same_as: Vec<SameAs>
}

impl Into<crate::model::MemberType> for MemberType {
    fn into(self) -> crate::model::MemberType {
        crate::model::MemberType {
            role: self.role,
            description: self.description,
            _type: self._type,
            min: self.min,
            max: self.max,
            hint: self.hint,
            same_as: self.same_as.into_iter().map(|x| x.uri).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Etymology {
    pub description: Option<String>,
    pub etymon: Vec<Etymon>,
}

impl Into<crate::model::Etymology> for Etymology {
    fn into(self) -> crate::model::Etymology {
        crate::model::Etymology {
            description: self.description,
            etymons: self.etymon.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(default)]
pub struct Etymon {
    pub when: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub note: Option<String>,
    pub etymon_unit: Vec<EtymonUnit>,
    pub translation: Option<String>,
}

impl Into<crate::model::Etymon> for Etymon {
    fn into(self) -> crate::model::Etymon {
        crate::model::Etymon {
            when: self.when,
            _type: self._type,
            note: self.note,
            etymon_units: self.etymon_unit.into_iter().map(|x| x.into()).collect(),
            translation: self.translation,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonUnit {
    pub lang_code: LangCode,
    pub text: String,
    #[serde(default)]
    pub reconstructed: Option<bool>,
    #[serde(default)]
    pub part_of_speech: Vec<PartOfSpeech>,
    #[serde(default)]
    pub translation: Option<String>,
}

impl Into<crate::model::EtymonUnit> for EtymonUnit {
    fn into(self) -> crate::model::EtymonUnit {
        crate::model::EtymonUnit {
            lang_code: self.lang_code,
            text: self.text,
            reconstructed: self.reconstructed,
            parts_of_speech: self.part_of_speech.into_iter().map(|x| x.tag).collect(),
            translation: self.translation,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl Into<crate::model::EtymonType> for EtymonType {
    fn into(self) -> crate::model::EtymonType {
        crate::model::EtymonType {
            _type: self._type,
            description: self.description,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonLanguage {
    pub lang_code: LangCode,
    #[serde(default)]
    pub display_name: Option<String>,
}

impl Into<crate::model::EtymonLanguage> for EtymonLanguage {
    fn into(self) -> crate::model::EtymonLanguage {
        crate::model::EtymonLanguage {
            lang_code: self.lang_code,
            display_name: self.display_name,
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SameAs {
    pub uri: String
}

