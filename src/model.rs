/// This module contains the data model for the lexicon.
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LexicographicResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    pub lang_code: LangCode,
    #[serde(alias = "entry")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub entries: Vec<Entry>,
    #[serde(alias = "translationLanguage")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub translation_languages: Vec<TranslationLanguage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "definitionTypeTag")]
    pub definition_type_tags: Vec<DefinitionTypeTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "inflectedFormTag")]
    pub inflected_form_tags: Vec<InflectedFormTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "labelTag")]
    pub label_tags: Vec<LabelTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "labelTypeTag")]
    pub label_type_tags: Vec<LabelTypeTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "partOfSpeechTag")]
    pub part_of_speech_tags: Vec<PartOfSpeechTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "sourceIdentityTag")]
    pub source_identity_tags: Vec<SourceIdentityTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "transcriptionSchemeTag")]
    pub transcription_scheme_tags: Vec<TranscriptionSchemeTag>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "relation")]
    pub relations: Vec<Relation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "relationType")]
    pub relation_types: Vec<RelationType>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymon_language: Vec<EtymonLanguage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymon_type: Vec<EtymonType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(transparent)]
pub struct LangCode(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Entry { 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub headword: HeadwordString,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub placeholder_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub homograph_number: Option<u32>,
    #[serde(alias = "partOfSpeech")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub parts_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "pronunciation")]
    pub pronunciations: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "inflectedForm")]
    pub inflected_forms: Vec<InflectedForm>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "senses")]
    pub sense: Vec<Sense>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub etymology: Vec<Etymology>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct HeadwordString(pub Vec<HeadwordStringPart>);

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum HeadwordStringPart {
    Text(String),
    PlaceholderMarker(String)
}
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct PartOfSpeech {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InflectedForm {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "pronunciation")]
    pub pronunciations: Vec<Pronunciation>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TextString(pub Vec<TextStringPart>);

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum TextStringPart {
    Text(String),
    HeadwordMarker(String),
    CollocateMarker(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Sense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub indicator: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(alias = "definition")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definitions: Vec<Definition>,
    #[serde(alias = "example")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub examples: Vec<Example>,
    #[serde(alias = "headwordExplanation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_explanations: Vec<HeadwordExplanation>,
    #[serde(alias = "headwordTranslation")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_translations: Vec<HeadwordTranslation>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Definition {
    #[serde(rename = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub definition_type: Vec<String>
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Pronunciation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "transcription")]
    pub transcriptions: Vec<Transcription>,
    #[serde(alias = "label")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Transcription {
    #[serde(alias = "$value")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scheme: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Example {
    pub text: TextString,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub collocate_markers: Vec<CollocateMarker>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub headword_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_identity: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub source_elaboration: Vec<String>,
    #[serde(alias = "label")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sound_file: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "exampleTranslation")]
    pub example_translations: Vec<ExampleTranslation>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TranslationLanguage {
    pub lang_code: LangCode,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordTranslation {
    pub text: HeadwordString,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub placeholder_markers: Vec<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lang_code: Option<LangCode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "partOfSpeech")]
    pub parts_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "pronunciation")]
    pub pronunciations: Vec<Pronunciation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "inflectedForm")]
    pub inflected_forms: Vec<InflectedForm>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct HeadwordExplanation {
    #[serde(alias = "$value")]
    pub text: TextString,
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ExampleTranslation {
    pub text: TextString,
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
    #[serde(alias = "label")]
    pub labels: Vec<Label>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub sound_file: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[serde(default)]
    pub for_headwords: bool,
    #[serde(default)]
    pub for_translations: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<ForLanguage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "forPartOfSpeech")]
    pub for_parts_of_speech: Vec<ForPartOfSpeech>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LabelTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub same_as: Vec<String>,
    #[serde(default)]
    pub for_headwords: bool,
    #[serde(default)]
    pub for_translations: bool,
    #[serde(default)]
    pub for_collocates: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<ForLanguage>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_parts_of_speech: Vec<ForPartOfSpeech>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[serde(default)]
    pub for_headwords: bool,
    #[serde(default)]
    pub for_translations: bool,
    #[serde(default)]
    pub for_etymology: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<ForLanguage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TranscriptionSchemeTag {
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub for_headwords: bool,
    #[serde(default)]
    pub for_translations: bool,
    #[serde(default)]
    pub for_etymology: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub for_languages: Vec<ForLanguage>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ForLanguage {
    pub lang_code: LangCode,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ForPartOfSpeech {
    pub tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[serde(alias = "member")]
    pub members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Member {
    #[serde(rename = "memberID")]
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<String>,
    pub obverse_listing_order: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    #[serde(alias = "memberType")]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MemberType {
    pub role: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Hint {
    Embed,
    Navigate,
    None
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Marker {
    start_index: u32,
    end_index: u32
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CollocateMarker {
    start_index: u32,
    end_index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    lemma: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    label: Vec<Label>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Etymology {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "etymon")]
    pub etymons: Vec<Etymon>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Etymon {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub when: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "etymonUnit")]
    pub etymon_units: Vec<EtymonUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub translation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonUnit {
    pub lang_code: LangCode,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reconstructed: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[serde(alias = "partOfSpeech")]
    pub parts_of_speech: Vec<PartOfSpeech>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub translation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EtymonLanguage {
    pub lang_code: LangCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_name: Option<String>,
}

