/// This module containss the data model for the lexicon.
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LexicographicResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub uri: Option<String>,
    pub lang_code: LangCode,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub translation_languages: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definition_type_tags: Vec<DefinitionTypeTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub inflected_form_tags: Vec<InflectedFormTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label_tags: Vec<LabelTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label_type_tags: Vec<LabelTypeTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub part_of_speech_tags: Vec<PartOfSpeechTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_identity_tags: Vec<SourceIdentityTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub transcription_scheme_tags: Vec<TranscriptionSchemeTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub relations: Vec<Relation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub relation_types: Vec<RelationType>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymon_languages: Vec<EtymonLanguage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymon_types: Vec<EtymonType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(transparent)]
pub struct LangCode(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Entry { 
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    pub headword: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub placeholder_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub homograph_number: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub parts_of_speech: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pronunciations: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub inflected_forms: Vec<InflectedForm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub senses: Vec<Sense>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymologies: Vec<Etymology>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InflectedForm {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pronunciations: Vec<Pronunciation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Sense {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub indicator: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definitions: Vec<Definition>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub examples: Vec<Example>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_explanations: Vec<HeadwordExplanation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_translations: Vec<HeadwordTranslation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub definition_type: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pronunciation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub transcriptions: Vec<Transcription>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Transcription {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub collocate_markers: Vec<CollocateMarker>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_elaboration: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub example_translations: Vec<ExampleTranslation>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordTranslation {
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub placeholder_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang_code: Option<LangCode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub parts_of_speech: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub pronunciations: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub inflected_forms: Vec<InflectedForm>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordExplanation {
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub collocate_markers: Vec<CollocateMarker>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang_code: Option<LangCode>,
 }

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExampleTranslation {
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub collocate_markers: Vec<CollocateMarker>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang_code: Option<LangCode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct DefinitionTypeTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InflectedFormTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_translations: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_parts_of_speech: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LabelTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub type_tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_collocates: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_parts_of_speech: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LabelTypeTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PartOfSpeechTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_etymology: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SourceIdentityTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TranscriptionSchemeTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_headwords: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_translations: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Relation {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Member {
    #[serde(rename = "ref")]
    pub ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub obverse_listing_order: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RelationType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scope_restriction: Option<ScopeRestriction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub member_types: Vec<MemberType>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ScopeRestriction {
    SameEntry,
    SameResource,
    Any
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MemberType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub _type: MemberTypeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hint: Option<Hint>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MemberTypeType {
    Sense,
    Entry,
    Collocate
}

impl Default for MemberTypeType {
    fn default() -> Self { MemberTypeType::Sense }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Hint {
    Embed,
    Navigate,
    None
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Marker {
    pub start_index: usize,
    pub end_index: usize
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CollocateMarker {
    pub start_index: usize,
    pub end_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lemma: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Etymology {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymons: Vec<Etymon>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Etymon {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub when: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    #[serde(default)]
    pub _type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymon_units: Vec<EtymonUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub translation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonUnit {
    pub lang_code: LangCode,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reconstructed: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "partOfSpeech")]
    #[serde(default)]
    pub parts_of_speech: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub translation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonLanguage {
    pub lang_code: LangCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,
}
